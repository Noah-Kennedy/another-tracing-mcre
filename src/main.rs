#[tokio::main(flavor = "current_thread")]
async fn main() {
    tokio::spawn(do_request()).await.unwrap();
}

async fn do_request() {
    tracing::info!("{}", std::future::ready("test").await);
}