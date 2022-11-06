use std::process;

use crate::{
    agents::Agent,
    commands::Command,
    error::CommonError,
    opt::{Opt, SubCommand},
    utils::{exclude, read_json_file, select_a_choice},
};

#[derive(Debug)]
pub struct Parser {
    pub command: Command,
    pub args: Option<Vec<String>>,
}

impl Parser {
    pub fn parser_opt(opt: &Opt) -> Parser {
        if opt.frozen {
            return Parser {
                command: Command::Frozen,
                args: None,
            };
        }
        let parser = Parser::parser_cmd(opt);
        match parser {
            Ok(p) => p,
            Err(e) => {
                eprintln!("===========================");
                eprintln!("{}", e);
                eprintln!("===========================");
                process::exit(0);
            }
        }
    }

    fn parser_cmd(opt: &Opt) -> Result<Parser, CommonError> {
        match &opt.cmd {
            None => Ok(Parser {
                command: Command::Install,
                args: None,
            }),
            Some(sub_command) => match sub_command {
                SubCommand::Un { package_name } => match opt.global {
                    true => Ok(Parser {
                        command: Command::GlobalUninstall,
                        args: Some(package_name.clone()),
                    }),
                    false => Ok(Parser {
                        command: Command::Uninstall,
                        args: Some(package_name.clone()),
                    }),
                },
                SubCommand::R { run_name } => match run_name {
                    None => {
                        let package_json = read_json_file("package.json")?;
                        let script = package_json.scripts.ok_or(CommonError::NotFound(
                            "package.json scripts field not found!".to_string(),
                        ))?;

                        let script_choices = script
                            .iter()
                            .map(|(k, v)| format!("{} - {}", k, v))
                            .collect::<Vec<String>>();

                        match script_choices.len() {
                            0 => Err(CommonError::JsonParseError(
                                "package.json scripts field is empty!".to_string(),
                            )),
                            _ => {
                                let ans = select_a_choice(&script_choices, "run", "Script to run")?;

                                Ok(Parser {
                                    command: Command::Run,
                                    args: Some(vec![ans]),
                                })
                            }
                        }
                    }
                    Some(name) => Ok(Parser {
                        command: Command::Run,
                        args: Some(vec![name.to_string()]),
                    }),
                },
                SubCommand::Other(v) => Ok(Parser::parser_other_args(v.clone())),
            },
        }
    }

    fn parser_other_args(args: Vec<String>) -> Parser {
        if args.contains(&String::from("-g")) {
            return Parser {
                command: Command::Global,
                args: Some(exclude(args, "-g")),
            };
        }
        Parser {
            command: Command::Add,
            args: Some(args),
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
