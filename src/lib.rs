mod commands;
mod cui;

pub async fn init() {
    let app = cui::Cui::new().await;
    app.process().await;
}
