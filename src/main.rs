fn main() {
    if let Err(e) = daily_file_mover::init() {
        eprintln!("Error: {:#}", e);
        std::process::exit(1);
    }
}
