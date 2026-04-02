use tokio::sync::mpsc;
#[tokio::main]
async fn main() {
    // 3.1
    tokio::spawn(async move {
        loop {
            println!("Waiting for Go...");
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    });
    // make main thred sleep
    tokio::time::sleep(std::time::Duration::from_secs(5)).await;
    // 3.2
    mpsc_example().await;
}

async fn mpsc_example() {
    let (tx, mut rx) = mpsc::channel(8);
    // send data in a separate task
    tokio::spawn(async move {
        for _ in 0..5 {
            println!("Sending data...");
            tx.send(23).await.unwrap();
        }
    });
    while let Some(value) = rx.recv().await {
        println!("Received: {}", value);
    }
}
