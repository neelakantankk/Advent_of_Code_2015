use std::process;

fn main() {
    if let Err(e) = day_04::run() {
        eprintln!("Ran into an error: {e}");
        process::exit(1);
    }
}
