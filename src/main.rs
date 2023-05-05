#[tokio::main]
async fn main() {
    daily_file_mover::init().await;
}
