use crate::{
    agents::Agent,
    commands::Command,
    error::CommonError,
    opt::{Opt, SubCommand},
    utils::{self, exclude},
};

#[derive(Debug)]
pub struct Parser {
    pub command: Command,
    args: Option<Vec<String>>,
}

impl Parser {
    pub fn parser_opt(opt: &Opt) -> Result<Parser, CommonError> {
        if opt.frozen {
            return Ok(Parser {
                command: Command::Frozen,
                args: None,
            });
        }

        if opt.clean {
            return Self::parse_clean(opt);
        }

        let parser = Parser::parse_cmd(opt)?;

        Ok(parser)
    }

    fn parse_clean(opt: &Opt) -> Result<Parser, CommonError> {
        match &opt.cmd {
            Some(v) => match v {
                SubCommand::Other(c) => {
                    if &c[0] == "lock" {
                        let remove =
                            utils::ask_confirm_question("Do you want to remove lockfile?")?;

                        if remove && !opt.debug {
                            utils::remove_lock_files()?;
                            println!("remove success!")
                        }
                    }
                }
                _ => (),
            },
            None => {
                let remove = utils::ask_confirm_question("Do you want to remove node_modules?")?;

                if remove && !opt.debug {
                    utils::remove_dir_all_file_with_path("node_modules")?;
                    println!("remove success!")
                }
            }
        }

        Ok(Parser {
            command: Command::IgnoredCommand,
            args: None,
        })
    }

    fn parse_cmd(opt: &Opt) -> Result<Parser, CommonError> {
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
                        let package_json = utils::read_json_file("package.json")?;
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
                                let ans = utils::select_a_choice(
                                    &script_choices,
                                    "run",
                                    "Script to run",
                                )?;

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
    pub fn gene_command(&mut self, agent: Agent) -> Option<String> {
        if self.command == Command::IgnoredCommand {
            return None;
        }

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
            Some(cmd) => match &cmd {
                Some(cmd) => {
                    let command = cmd.clone();
                    if command.contains("$0") {
                        match &self.args {
                            None => Some(command.replace("$0", "").trim().to_string()),
                            Some(arg) => Some(command.replace("$0", &arg.join(" "))),
                        }
                    } else {
                        Some(command)
                    }
                }
                None => None,
            },
            None => None,
        }
    }
}
