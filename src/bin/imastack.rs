extern crate imastack;

use std::io;
use std::io::Write;

fn main() {
    loop {
        let mut code = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut code)
            .expect("Failed to read line");
        imastack::eval(code.trim().to_string());
    }
}
