use std::process::Command;

pub struct Runner {
    executor: Executor,
}

impl Runner {
    pub fn run(args: Vec<String>) -> () {
        let mut runner = Self::new();
        let _child = runner
            .executor
            .command
            .arg(runner.executor.arg_0)
            .args(args)
            .spawn()
            .unwrap();
    }
}

pub struct Executor {
    command: Command,
    arg_0: String,
}

impl Runner {
    fn new() -> Runner {
        let executor = if cfg!(target_os = "windows") {
            Executor {
                command: Command::new("cmd"),
                arg_0: "/C".to_string(),
            }
        } else {
            Executor {
                command: Command::new("sh"),
                arg_0: "-c".to_string(),
            }
        };
        Runner { executor }
    }
}
