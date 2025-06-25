mod commands;
mod cui;

use anyhow::Result;

pub async fn init() -> Result<()> {
    let app = cui::Cui::new().await;
    app.process().await
}
