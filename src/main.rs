fn main() {
    if let Err(err) = pdf::run() {
        eprintln!("error: {err}");
        std::process::exit(1);
    }
}
