use std::process;

use monkey_rust::repl;

fn main() {
    if let Err(e) = repl::start() {
        eprintln!("Application error: {e}");
        process::exit(1);
    }
}
