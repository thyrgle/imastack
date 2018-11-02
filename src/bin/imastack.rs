extern crate imastack;

use std::io;
use std::io::Write;

/// Simple REPL for the imastack langauge.
fn main() {
    loop {
        let mut code = String::new();
        print!("> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut code)
            .expect("Failed to read line");
        let output = imastack::eval(code.trim());
        for num in output {
            print!("{} ", num);
        }
        println!()
    }
}
