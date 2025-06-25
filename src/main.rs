#[tokio::main]
async fn main() {
    if let Err(e) = daily_file_mover::init().await {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
