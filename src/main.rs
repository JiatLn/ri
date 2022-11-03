use structopt::StructOpt;

mod agents;
mod commands;
mod opt;
mod parser;
mod runner;
mod utils;

fn main() {
    let opt = opt::Opt::from_args();

    let agent = agents::get_current_agent();

    let mut parser = parser::Parser::parser_opt(&opt);

    let cmd = parser.gene_command(agent);

    if opt.debug {
        println!("{}", cmd);
    } else {
        runner::Runner::run(cmd);
    }
}
