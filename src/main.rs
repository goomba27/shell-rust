use std::env;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Commands {
    Echo,
    Exit,
    Type,
    Pwd,
    Cd,
}
impl Commands {
    fn from_name(value: &str) -> Option<Self> {
        match value {
            "echo" => Some(Commands::Echo),
            "exit" => Some(Commands::Exit),
            "type" => Some(Commands::Type),
            "pwd" => Some(Commands::Pwd),
            "cd" => Some(Commands::Cd),
            _ => None,
        }
    }

    fn kind(&self) -> CommandType {
        match self {
            Commands::Echo | Commands::Exit | Commands::Type | Commands::Pwd | Commands::Cd => {
                CommandType::Builtin
            }
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

fn main() -> anyhow::Result<()> {
    loop {
        print!("$ ");
        io::stdout().flush().unwrap();

        let line = read_line();
        if line.is_empty() {
            continue;
        }

        //TODO: move to split_line
        let (cmd, args) = line.split_once(' ').unwrap_or((&line, ""));
        let args = args.trim();

        //TODO: move to execute
        match Commands::from_name(cmd) {
            Some(Commands::Exit) => break,
            Some(Commands::Echo) => println!("{}", args),
            Some(Commands::Type) => handle_type(args),
            Some(Commands::Pwd) => {
                let path = env::current_dir()?;
                println!("{}", path.display());
            }
            Some(Commands::Cd) => {
                let new_path = Path::new(args);
                let change = env::set_current_dir(&new_path);
                if change.is_err() {
                    println!("cd: {args}: No such file or directory");
                }
                // if change.is_ok() {
                //     println!("{}", new_path.display());
                // } else {
                //     println!("cd: {args}: No such file or directory");
                // }
            }
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

fn read_line() -> String {
    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Failed to take input");

    input.trim().to_string()
}

fn split_line(line: String) -> Vec<String> {
    todo!()
}

fn execute() {
    todo!()
}
