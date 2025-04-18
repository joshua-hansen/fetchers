fn main() {
    if let Err(e) = fetchers::run() {
        eprintln!("Application Error: {}", e);
    }
}
