use futures::{stream, StreamExt};

#[tokio::main]
async fn main() {
    let mut stream = stream::iter(vec![1, 2, 3, 4, 5]);
    let result = stream.next().await;
    println!("{:?}", result);
}
