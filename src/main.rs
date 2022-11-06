use error::CommonError;
use structopt::StructOpt;

mod agents;
mod commands;
mod error;
mod opt;
mod parser;
mod runner;
mod utils;

fn main() -> Result<(), CommonError> {
    let opt = opt::Opt::from_args();

    let agent = agents::get_current_agent()?;

    let mut parser = parser::Parser::parser_opt(&opt);

    let cmd = parser.gene_command(agent);

    println!("{}", cmd);

    if opt.debug {
        return Ok(());
    }

    match runner::Runner::run(cmd) {
        Ok(_) => Ok(()),
        Err(err) => panic!("{}", err),
    }
}
