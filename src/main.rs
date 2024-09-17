use tokio;
use tokio_util::sync::CancellationToken;
use std::time::Duration;

#[tokio::main]
async fn main() {
    // Создаем CancellationToken
    let cancellation_token = CancellationToken::new();
    let token = cancellation_token.clone();

    // Запускаем асинхронную задачу
    let handle = tokio::spawn(async move {
        let mut interval = tokio::time::interval(Duration::from_secs(1));
        loop {
            tokio::select! {
                _ = interval.tick() => {
                    println!("Task is running");
                }
                _ = token.cancelled() => {
                    println!("Task is cancelled");
                    break;
                }
            }
        }
    });

    // Запускаем основную задачу
    tokio::time::sleep(Duration::from_secs(5)).await;
    cancellation_token.cancel(); // Отправляем сигнал отмены

    handle.await.unwrap();
}
