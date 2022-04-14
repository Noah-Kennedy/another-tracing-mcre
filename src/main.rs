#[tokio::main(flavor = "current_thread")]
async fn main() {
    tracing_subscriber::fmt().init();
    tokio::spawn(do_request()).await.unwrap();
}

async fn do_request() {
    let response = reqwest::get("https://google.com").await.unwrap();
    tracing::info!("{}", response.text().await.unwrap());
}