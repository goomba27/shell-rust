use std::io::{self, Write};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Command {
    Echo,
    Exit,
    Type,
}
impl Command {
    fn from_name(value: &str) -> Option<Self> {
        match value {
            "echo" => Some(Command::Echo),
            "exit" => Some(Command::Exit),
            "type" => Some(Command::Type),
            _ => None,
        }
    }

    fn kind(&self) -> CommandType {
        match self {
            Command::Echo | Command::Exit | Command::Type => CommandType::Builtin,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum CommandType {
    Builtin,
}
impl CommandType {
    fn from_name(cmd: &str) -> Option<Self> {
        Command::from_name(cmd).map(|c| c.kind())
    }
}

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
        let args = args.trim();

        match Command::from_name(cmd) {
            Some(Command::Exit) => break,
            Some(Command::Echo) => println!("{}", args),
            Some(Command::Type) => handle_type(args),
            None => println!("{}: command not found", cmd),
        }
    }
}

fn handle_type(args: &str) {
    match CommandType::from_name(args) {
        Some(CommandType::Builtin) => println!("{} is a shell builtin", args),
        None => match which::which(args) {
            Ok(path) => println!("{args} is {}", path.display()),
            Err(_) => println!("{args}: not found"),
        },
    }

    //let cmd = CommandType::from_name(args);

    //     if cmd.is_some() && cmd.unwrap() == CommandType::Builtin {
    //         return println!("{} is a shell builtin", args);
    //     }
    //
    //     match which::which(args) {
    //         Ok(path) => println!("{args} is {}", path.display()),
    //         Err(_) => println!("{args}: not found"),
    //     }
}
