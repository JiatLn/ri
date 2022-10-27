use std::process::{Child, Command};

use crate::agents::{Agent, ShortCommand};

pub struct TheCommand {
    command: Command,
    arg_0: String,
}

impl TheCommand {
    pub fn new() -> TheCommand {
        if cfg!(target_os = "windows") {
            TheCommand {
                command: Command::new("cmd"),
                arg_0: "/C".to_string(),
            }
        } else {
            TheCommand {
                command: Command::new("sh"),
                arg_0: "-c".to_string(),
            }
        }
    }

    pub fn run(&mut self, args: Vec<String>) -> Child {
        self.command.arg(&self.arg_0).args(args).spawn().unwrap()
    }

    pub fn gene_command(&self, agent: Agent, args: Vec<String>) -> String {
        println!("{:?}", args);
        if args.len() == 1 {
            match args[0].as_str().into() {
                ShortCommand::Ri => format!("{} {}", agent.to_string(), "install".to_string()),
                ShortCommand::Rr => format!("{} {}", agent.to_string(), "run".to_string()),
                ShortCommand::Ru => format!("{} {}", agent.to_string(), "upgrade".to_string()),
                _ => "todo".to_string(),
            }
        } else {
            "[ri] unknown the command".to_string()
        }
    }
}
