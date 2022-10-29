#![cfg_attr(debug_assertions, allow(dead_code, unused_imports, unused_variables))]

use structopt::StructOpt;

mod agents;
mod commands;
mod opt;
mod parse;

fn main() {
    let opt = opt::Opt::from_args();

    let agent = agents::get_current_agent();

    let parser = parse::Parser::parser_cmd(opt.cmd);

    println!("The command is:\n{}", parser.gene_command(agent));
}
