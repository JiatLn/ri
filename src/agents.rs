use std::{collections::HashMap, fs, hash::Hash};

use crate::commands::Command;

#[derive(Debug, PartialEq, Eq, Hash)]
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
    pub fn get_agent_hash_map(agent: Agent) -> HashMap<Command, Option<String>> {
        match agent {
            Agent::Npm | Agent::None => HashMap::from([
                (Command::Agent, Some("npm $0".to_string())),
                (Command::Run, Some("npm run $0".to_string())),
                (Command::Install, Some("npm i $0".to_string())),
                (Command::Frozen, Some("npm ci".to_string())),
                (Command::Global, Some("npm i -g $0".to_string())),
                (Command::Add, Some("npm i $0".to_string())),
                (Command::Upgrade, Some("npm update $0".to_string())),
                (Command::UpgradeInteractive, None),
                (Command::Execute, Some("npx $0".to_string())),
                (Command::Uninstall, Some("npm uninstall $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("npm uninstall -g $0".to_string()),
                ),
            ]),
            Agent::Bun => todo!(),
            Agent::Yarn => HashMap::from([
                (Command::Agent, Some("yarn $0".to_string())),
                (Command::Run, Some("yarn run $0".to_string())),
                (Command::Install, Some("yarn install $0".to_string())),
                (
                    Command::Frozen,
                    Some("yarn install --frozen-lockfile".to_string()),
                ),
                (Command::Global, Some("yarn global add $0".to_string())),
                (Command::Add, Some("yarn add $0".to_string())),
                (Command::Upgrade, Some("yarn upgrade $0".to_string())),
                (
                    Command::UpgradeInteractive,
                    Some("yarn upgrade-interactive $0".to_string()),
                ),
                (Command::Execute, Some("yarn dlx $0".to_string())),
                (Command::Uninstall, Some("yarn remove $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("yarn global remove $0".to_string()),
                ),
            ]),
            Agent::Pnpm => HashMap::from([
                (Command::Agent, Some("pnpm $0".to_string())),
                (Command::Run, Some("pnpm run $0".to_string())),
                (Command::Install, Some("pnpm i $0".to_string())),
                (
                    Command::Frozen,
                    Some("pnpm i --frozen-lockfile".to_string()),
                ),
                (Command::Global, Some("pnpm add -g $0".to_string())),
                (Command::Add, Some("pnpm add $0".to_string())),
                (Command::Upgrade, Some("pnpm update $0".to_string())),
                (
                    Command::UpgradeInteractive,
                    Some("pnpm update -i $0".to_string()),
                ),
                (Command::Execute, Some("pnpm dlx $0".to_string())),
                (Command::Uninstall, Some("pnpm remove $0".to_string())),
                (
                    Command::GlobalUninstall,
                    Some("pnpm remove --global $0".to_string()),
                ),
            ]),
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

    println!("Current agent is\n{:?}", &agent);

    match agent {
        Agent::None => {
            // TODO: pick agent by user
            Agent::Npm
        }
        _ => agent,
    }
}
