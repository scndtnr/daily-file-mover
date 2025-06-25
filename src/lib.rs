mod commands;
mod cui;

use anyhow::Result;

pub fn init() -> Result<()> {
    let app = cui::Cui::new();
    app.process()
}
