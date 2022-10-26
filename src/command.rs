use std::process::{Child, Command};

pub struct TheCommand {
    command: Command,
}

impl TheCommand {
    pub fn new() -> TheCommand {
        let command = if cfg!(target_os = "windows") {
            Command::new("cmd")
        } else {
            Command::new("sh")
        };
        TheCommand { command }
    }

    pub fn run(&mut self, args: Vec<String>) -> Child {
        self.command.args(args).spawn().unwrap()
    }
}
