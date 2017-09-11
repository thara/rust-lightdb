use std::io;
use std::io::prelude::*;
use std::process;

fn main() {
    loop {
        print!("db > ");
        io::stdout().flush().ok();

        let mut input = String::new();
        io::stdin().read_line(&mut input).unwrap();

        match input.trim().as_ref() {
            ".exit" => {
                println!("Bye.");
                process::exit(0);
            },
            s => {
                println!("Unrecognized command : {}", s);
            }
        }
    }
}
