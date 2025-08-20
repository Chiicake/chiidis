use std::sync::Arc;
use tokio::sync::mpsc;

#[tokio::main]
async fn main() {

    let (tx, mut rx) = mpsc::channel(32);
    let tx = Arc::new(tx);
    let tx1 = tx.clone();
    let tx2 = tx.clone();


    tokio::spawn(async move {
        tx1.send("sending from first handle").await.unwrap();
    });

    tokio::spawn(async move {
        tx2.send("sending from second handle").await.unwrap();
    });

    while let Some(message) = rx.recv().await {
        println!("GOT = {}", message);
    }
    use mini_redis::client;
    // The `move` keyword is used to **move** ownership of `rx` into the task.
    let manager = tokio::spawn(async move {
        // Establish a connection to the server
        let mut client = client::connect("127.0.0.1:6379").await.unwrap();

        // Start receiving messages
        while let Some(cmd) = rx.recv().await {
            use Command::*;

            match cmd {
                Get { key } => {
                    client.get(&key).await;
                }
                Set { key, val } => {
                    client.set(&key, val).await;
                }
            }
        }
    });


}

