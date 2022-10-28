#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

mod agents;
mod args;
mod command;

use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Ri", about = "A rust version ni.")]
struct Opt {
    #[structopt(subcommand)]
    cmd: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// install package
    ///
    /// TODO
    /// add description
    I { package_name: Option<String> },

    /// run
    R,

    #[structopt(external_subcommand)]
    Other(Vec<String>),
}

fn main() {
    let opt = Opt::from_args();
    println!("{:?}", &opt.cmd);

    // let args = args::Args::new();

    // let current_agent = agents::get_current_agent();

    // println!("current agent is {:?}", &current_agent);

    // let command = command::TheCommand::new();

    // let command = command.gene_command(current_agent, args);

    // println!("the command is {:?}", command);

    // command.run(args);
}
