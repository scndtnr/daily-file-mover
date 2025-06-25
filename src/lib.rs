mod commands;
mod cui;

pub fn init() {
    let app = cui::Cui::new();
    app.process();
}
