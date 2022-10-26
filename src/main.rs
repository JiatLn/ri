mod args;
mod command;

fn main() {
    let args = args::Args::new();

    let mut command = command::TheCommand::new();

    command.run(args);
}
