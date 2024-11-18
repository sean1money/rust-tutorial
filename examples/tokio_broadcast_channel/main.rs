use tokio::sync::broadcast;

#[tokio::main]
async fn main() {
    let (tx, mut rx1) = broadcast::channel(1024);

    let mut rx2 = tx.subscribe();

    tokio::spawn(async move {
        assert_eq!(rx1.recv().await.unwrap(), 1);
        assert_eq!(rx1.recv().await.unwrap(), 2);
    });

    tokio::spawn(async move {
        assert_eq!(rx2.recv().await.unwrap(), 1);
        assert_eq!(rx2.recv().await.unwrap(), 2);
    });

    tx.send(1).unwrap();
    tx.send(2).unwrap();
}
