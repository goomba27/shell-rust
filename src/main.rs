#[allow(unused_imports)]
use std::io::{self, Write};

fn main() {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to take input");

        let line = input.trim();
        let (cmd, args) = line.split_once(' ').unwrap_or((line, ""));

        match cmd {
            "exit" => break,
            "echo" => println!("{}", args),
            _ => println!("{}: command not found", cmd),
        }
    }
}
