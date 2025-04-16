/// Handle the WebSocket connection, if the connection is valid, it will be kept alive.
/// If not, the connection will be closed.
async fn handle_websocket<
    Pool: TransactionPool + 'static,
    Events: Clone + Send + Sync + 'static + CommittedStateSubscriptions,
>(
    socket: WebSocket,
    who: SocketAddr,
    context: RestContext<Pool, Events>,
) {
    let rest_config = context.rest_config;
    let message_buffer_capacity = rest_config.ws_message_buffer_capacity;

    // Split the socket into sender and receiver.
    let (mut socket_sender, mut socket_receiver) = socket.split();

    // Create a channel for the communication between the socket sender and socket receiver.
    let (socket_tx, mut socket_rx) = mpsc::channel::<Message>(message_buffer_capacity);

    let safe_conn_state = Arc::new(Mutex::new(ConnState::new()));
    let ping_state = safe_conn_state.clone();
    let pong_state = safe_conn_state.clone();

    let keepalive_timeout = rest_config.ws_keepalive_timeout_duration;

    // Create a ping interval for the socket, every `ws_ping_interval_duration` seconds send a ping message to the client.
    let mut ping_interval = tokio::time::interval(rest_config.ws_ping_interval_duration);

    // Every `ws_keepalive_checker_duration` seconds, check if the connection is alive.
    let mut keepalive_checker = tokio::time::interval(rest_config.ws_keepalive_checker_duration);

    let streams = Arc::new(RwLock::new(PubsubStreams::new()));
    let pubsub = context.pubsub.clone();

    // Send task for sending messages to the client.
    let mut sender_task = tokio::spawn(async move {
        // Try to send a close message to the client.
        async fn try_send_close(sender: &mut SplitSink<WebSocket, Message>, timeout: Duration) {
            info!(
                "connection keepalive timeout, close the connection, no pong response in {} seconds",
                timeout.as_secs()
            );
            let close_msg = Message::Close(Some(CloseFrame {
                code: CLOSE_CODE_TIMEOUT,
                reason: Utf8Bytes::from_static("connection keepalive timeout"),
            }));
            let _ = sender.send(close_msg).await;
        }

        loop {
            tokio::select! {
                _ = ping_interval.tick() => {
                    // Check if the connection is alive timeout.
                    if ping_state.lock().await.is_timeout(keepalive_timeout) {
                        try_send_close(&mut socket_sender, keepalive_timeout).await;
                        break;
                    }

                    let ping = socket_sender.send(Message::Ping(Bytes::from_static(PING_PAYLOAD))).await;
                    match ping {
                        Ok(_) => {}
                        Err(e) => {
                            // If sent failed, maybe the connection has trouble, and try next time.
                            // Until the alive timeout, the connection will be closed.
                            error!("Error in sending ping message: {e}");
                        }
                    }
                }
                _ = keepalive_checker.tick() => {
                    if ping_state.lock().await.is_timeout(keepalive_timeout) {
                        try_send_close(&mut socket_sender, keepalive_timeout).await;
                        break;
                    }
                }
                reply_msg = socket_rx.recv() => {
                    if let Some(reply_msg) = reply_msg {
                        // Use `feed` or `send_all` to instead of `send`?
                        if socket_sender.send(reply_msg).await.is_err() {
                            // ? quit directly if error occurs
                            break;
                        }
                    }
                }
            }
        }
    });

    // Receive task for receive messages from the client.
    let mut receiver_task = tokio::spawn(async move {
        while let Some(message) = socket_receiver.next().await {
            match message {
                Ok(message) => {
                    let reply_message = match message {
                        Message::Close(close) => {
                            // close message: don't reply to client, quit the task directly.
                            info!("connection actively closed: {close:?}");
                            break;
                        }
                        Message::Pong(pong) => {
                            debug!("{who} received pong: {pong:?}");
                            // Don't reply for the pong message, only update the last active time.
                            pong_state.lock().await.update_last_active();
                            continue;
                        }
                        Message::Ping(ping) => {
                            debug!("ping: {ping:?}");
                            Message::Pong(ping)
                        }
                        Message::Binary(binary) => {
                            debug!("received binary from {who}: {binary:?}");
                            let text = Utf8Bytes::try_from(binary);
                            match text {
                                Ok(text) => {
                                    process_recv_message(
                                        text,
                                        &context.id_provider,
                                        streams.clone(),
                                        socket_tx.clone(),
                                        pubsub.clone(),
                                    )
                                    .await
                                }
                                Err(e) => build_error_message(RestError::UnsupportedWSCodec(
                                    e.to_string(),
                                )),
                            }
                        }
                        Message::Text(text) => {
                            debug!("received text message from {who}: {text:?}");
                            process_recv_message(
                                text,
                                &context.id_provider,
                                streams.clone(),
                                socket_tx.clone(),
                                pubsub.clone(),
                            )
                            .await
                        }
                    };

                    // Reply the message that the client sent.
                    info!("reply message to client: {reply_message:?}");
                    let sent = socket_tx.send(reply_message).await;
                    match sent {
                        Ok(_) => {}
                        Err(e) => {
                            error!("Error in sending message to reply channel: {e}");
                        }
                    }
                }
                Err(e) => {
                    error!("websocket receiver error: {e}");
                }
            }
        }
    });

    tokio::select! {
        s = (&mut sender_task) => {
            if let Err(e) = s {
                error!("Error in sender task: {e}");
            }
            receiver_task.abort();
        }
        r = (&mut receiver_task) => {
            if let Err(e) = r {
                error!("Error in receiver task: {e}");
            }
            sender_task.abort();
        }
    }

    // drop?

    info!("WebSocket connection disconnected with {who}");
}

async fn process_recv_message<
    Events: Clone + Send + Sync + 'static + CommittedStateSubscriptions,
>(
    text: Utf8Bytes,
    id_provider: &SubscriptionIdProvider,
    streams: Arc<RwLock<PubsubStreams>>,
    message_tx: mpsc::Sender<Message>,
    pubsub: PubSub<Events>,
) -> Message {
    let subscription = serde_json::from_str::<SubscriptionMessage>(&text);
    match subscription {
        Ok(subscription) => {
            let reply = match subscription {
                SubscriptionMessage::Subscribe(subscription) => {
                    let stream = subscription.stream;
                    let subscription_id = match &stream {
                        SubscriptionStream::Checkpoints { name, full } => {
                            let maybe_subscription_id =
                                { streams.read().await.subscription_id_by_name(name).cloned() };

                            if let Some(subscription_id) = maybe_subscription_id {
                                // Already subscribed, return the subscription id.
                                subscription_id
                            } else {
                                let subscription_id = id_provider.next_id();
                                let entry = SubscriptionSink::new(
                                    subscription_id.clone(),
                                    stream.clone(),
                                    message_tx,
                                );

                                let full = *full;
                                let entry_clone = entry.clone();

                                let stream_push_task = tokio::spawn(async move {
                                    let sub_id = entry_clone.subscription_id().clone();
                                    let st_name = entry_clone.stream_name().clone();
                                    let _ = pipe_from_stream(
                                        entry_clone,
                                        pubsub.new_checkpoints_stream(full),
                                    )
                                    .await;
                                    info!("stream push task done: subscription_id: {sub_id:?} stream_name: {st_name:?}");
                                });

                                streams.write().await.add_subscription(
                                    name.clone(),
                                    entry,
                                    stream_push_task,
                                );

                                subscription_id
                            }
                        }
                    };

                    SubscriptionResponse::new(
                        subscription.id,
                        SubscriptionResult::SubscribeResult(subscription_id),
                    )
                }
                SubscriptionMessage::Unsubscribe(subscription) => {
                    let request_id = subscription.id;
                    let contains = { streams.read().await.contains(&subscription.stream) };
                    if contains {
                        info!("start tounsubscribe stream: {:?}", subscription.stream);
                        streams
                            .write()
                            .await
                            .cancel_subscription(&subscription.stream);
                    }

                    // if the stream not subscribed, also return success.
                    SubscriptionResponse::new(
                        request_id,
                        SubscriptionResult::UnsubscribeResult(true),
                    )
                }
            };

            let reply_text = serde_json::to_string(&reply).unwrap();
            Message::Text(Utf8Bytes::from(reply_text))
        }
        Err(e) => {
            // Error when parsing subscription message, return error code.
            error!("Error in parsing subscription message: {e}");
            build_error_message(RestError::InvalidSubscriptionMessage)
        }
    }
}

fn build_error_message(error: RestError) -> Message {
    let error_data = error.into();
    let reply = SubscriptionResponse::new(0, SubscriptionResult::Error(error_data));
    let reply_text = serde_json::to_string(&reply).unwrap();
    Message::Text(Utf8Bytes::from(reply_text))
}
