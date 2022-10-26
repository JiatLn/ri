use std::process::{Child, Command};

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
}
