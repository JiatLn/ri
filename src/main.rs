#![cfg_attr(
    debug_assertions,
    allow(dead_code, unused_imports, unused_variables, unused_mut)
)]

mod agents;
mod args;
mod command;
fn main() {
    let args = args::Args::new();

    let current_agent = agents::get_current_agent();

    println!("current agent is {:?}", &current_agent);

    println!("got the args: {:?}", args);

    let command = command::TheCommand::new();

    let command = command.gene_command(current_agent, args);

    println!("the command is {:?}", command);

    // command.run(args);
}
