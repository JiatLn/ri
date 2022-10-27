use std::{collections::HashMap, fs};

#[derive(Debug)]
pub enum Agent {
    Bun,
    Pnpm,
    Npm,
    Yarn,
    None,
}

impl Agent {
    pub fn to_string(self) -> String {
        match self {
            Agent::Bun => "bun".to_string(),
            Agent::Npm => "npm".to_string(),
            Agent::Yarn => "yarn".to_string(),
            Agent::Pnpm => "pnpm".to_string(),
            _ => "todo".to_string(),
        }
    }
}

pub enum ShortCommand {
    Ri,
    Rr,
    Ru,
    Unkown,
}

impl From<&str> for ShortCommand {
    fn from(s: &str) -> Self {
        match s {
            "ri" => ShortCommand::Ri,
            "rr" => Self::Rr,
            "ru" => Self::Ru,
            _ => ShortCommand::Unkown,
        }
    }
}

pub struct Agents {
    pub lock_map: HashMap<String, Agent>,
}

impl Agents {
    pub fn new() -> Agents {
        Agents {
            lock_map: HashMap::from([
                ("bun.lockb".to_string(), Agent::Bun),
                ("pnpm-lock.yaml".to_string(), Agent::Pnpm),
                ("yarn.lock".to_string(), Agent::Yarn),
                ("package-lock.json".to_string(), Agent::Npm),
                ("npm-shrinkwrap.json".to_string(), Agent::Npm),
            ]),
        }
    }
}

pub fn get_current_agent() -> Agent {
    let mut agent = Agent::None;

    let agents = Agents::new();
    for (file_name, the_agent) in agents.lock_map.into_iter() {
        let is_found = fs::read(&file_name).is_ok();
        if is_found {
            agent = the_agent;
            break;
        }
    }

    match agent {
        Agent::None => panic!("agent not found!"),
        _ => agent,
    }
}
