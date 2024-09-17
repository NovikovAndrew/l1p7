use tokio::sync::mpsc;
use tokio::time::{sleep, Duration};

#[tokio::main]
async fn main() {
    let (tx, mut rx) = mpsc::channel(32);

    // Запускаем задачу, которая будет обрабатывать сообщения
    tokio::spawn(async move {
        while let Some(message) = rx.recv().await {
            println!("Получено сообщение: {:?}", message);
        }
        println!("Канал закрыт, задача завершена.");
    });

    // Отправляем сообщения
    tx.send("Привет".to_string()).await.unwrap();
    tx.send("Мир".to_string()).await.unwrap();

    // Закрываем канал
    drop(tx);

    // Ждем немного, чтобы убедиться, что задача завершится после закрытия канала
    sleep(Duration::from_secs(1)).await;
}
