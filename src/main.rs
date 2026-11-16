#[allow(unused_imports)]
use std::io::{self, Write};

// enum Type {
//     Executable,
//     Builtin,
//     Unrecognized,
// }

enum Builtin {
    Echo,
    Exit,
    Type,
}
impl Builtin {
    fn is_builtin(cmd: &str) -> String {
        if Builtin::try_from(cmd).is_ok() {
            return format!("{} is a shell builtin", cmd);
        };

        return format!("{}: not found", cmd);
    }
}
impl TryFrom<&str> for Builtin {
    type Error = ();
    fn try_from(cmd: &str) -> Result<Self, Self::Error> {
        match cmd {
            "echo" => Ok(Builtin::Echo),
            "exit" => Ok(Builtin::Exit),
            "type" => Ok(Builtin::Type),
            _ => Err(()),
        }
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

        match cmd {
            "type" => {
                let res = Builtin::is_builtin(args.trim());
                println!("{res}");
            }
            "exit" => break,
            "echo" => println!("{}", args),
            _ => println!("{}: command not found", cmd),
        }
    }
}
