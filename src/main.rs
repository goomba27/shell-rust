use std::io::{self, Write};
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Commands {
    Echo,
    Exit,
    Type,
}
impl Commands {
    fn from_name(value: &str) -> Option<Self> {
        match value {
            "echo" => Some(Commands::Echo),
            "exit" => Some(Commands::Exit),
            "type" => Some(Commands::Type),
            _ => None,
        }
    }

    fn kind(&self) -> CommandType {
        match self {
            Commands::Echo | Commands::Exit | Commands::Type => CommandType::Builtin,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandType {
    Builtin,
}
impl CommandType {
    fn from_name(cmd: &str) -> Option<Self> {
        Commands::from_name(cmd).map(|c| c.kind())
    }
}

fn main() -> anyhow::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect("Failed to take input");

        let line = input.trim();
        if line.is_empty() {
            continue;
        }

        let (cmd, args) = line.split_once(' ').unwrap_or((line, ""));
        let args = args.trim();

        match Commands::from_name(cmd) {
            Some(Commands::Exit) => break,
            Some(Commands::Echo) => println!("{}", args),
            Some(Commands::Type) => handle_type(args),
            None => match which::which(cmd) {
                Ok(_) => {
                    if let Err(e) = Command::new(cmd).args(args.split_whitespace()).status() {
                        eprintln!("{cmd}: {e}");
                    }
                }
                Err(_) => println!("{cmd}: command not found"),
            },
        }
    }
    Ok(())
}

fn handle_type(args: &str) {
    match CommandType::from_name(args) {
        Some(CommandType::Builtin) => println!("{} is a shell builtin", args),
        None => check_executable(args),
    }
}

fn check_executable(args: &str) {
    match which::which(args) {
        Ok(path) => println!("{args} is {}", path.display()),
        Err(_) => println!("{args}: not found"),
    }
}
