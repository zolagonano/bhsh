use ctrlc::set_handler;
use std::io::{self, BufRead};

fn main() {
    let _ = set_handler(move || {
    });

    let stdin = io::stdin();
    let mut stdin_lock = stdin.lock();
    let mut input = String::new();

    loop {
        let _ = stdin_lock.read_line(&mut input);
        input.clear();
    }
}
