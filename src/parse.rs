use std::fmt::format;

use crate::{agents::Agent, commands::Command, opt::SubCommand};

#[derive(Debug)]
pub struct Parser {
    pub command: Command,
    pub args: Option<Vec<String>>,
}

impl Parser {
    pub fn parser_cmd(cmd: Option<SubCommand>) -> Parser {
        match cmd {
            None => Parser {
                command: Command::Install,
                args: None,
            },
            Some(sub_command) => match sub_command {
                SubCommand::Un { package_name } => Parser {
                    command: Command::Uninstall,
                    args: Some(vec![package_name]),
                },
                SubCommand::R { run_name } => match run_name {
                    None => Parser {
                        command: Command::Run,
                        args: None,
                    },
                    Some(name) => Parser {
                        command: Command::Run,
                        args: Some(vec![name]),
                    },
                },
                SubCommand::Other(v) => Parser::parser_other_args(v),
            },
        }
    }

    fn parser_other_args(args: Vec<String>) -> Parser {
        if args.len() > 0 {
            Parser {
                command: Command::Install,
                args: Some(args.clone()),
            }
        } else {
            Parser {
                command: Command::Unkown,
                args: None,
            }
        }
    }
}

impl Parser {
    pub fn gene_command(&mut self, agent: Agent) -> String {
        let hash_map = Agent::get_agent_hash_map(agent);

        // instand of yarn install xxx => yarn add xxx
        match &agent {
            Agent::Yarn | Agent::Pnpm => {
                if self.command == Command::Install && self.args.is_some() {
                    self.command = Command::Add
                }
            }
            _ => (),
        }

        match hash_map.get(&self.command) {
            None => panic!("got none"),
            Some(cmd) => match &cmd {
                None => panic!("got ignore"),
                Some(cmd) => {
                    let command = cmd.clone();
                    if command.contains("$0") {
                        match &self.args {
                            None => command.replace("$0", "").trim().to_string(),
                            Some(arg) => command.replace("$0", &arg.join(" ")),
                        }
                    } else {
                        command
                    }
                }
            },
        }
    }
}
