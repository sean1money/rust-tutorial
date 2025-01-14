use metrics::Counter;
use metrics_derive::Metrics;

fn main() {
    let metrics = MeteredReceiverMetrics::new("txpool.total-txs");
    metrics.messages_received_total.increment(1);

    println!("{:?}", metrics.messages_received_total);
    MeteredReceiverMetrics::describe("txpool.total-txs");
}

#[derive(Clone, Metrics)]
#[metrics(dynamic = true)]
struct MeteredReceiverMetrics {
    /// Number of messages received
    messages_received_total: Counter,
}
