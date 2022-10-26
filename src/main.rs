mod args;
mod command;

fn main() {
    let args = args::Args::new();

    println!("got the args: {:?}", args);

    let mut command = command::TheCommand::new();

    command.run(args);
}
