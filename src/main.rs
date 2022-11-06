use error::CommonError;
use structopt::StructOpt;

mod agents;
mod commands;
mod error;
mod opt;
mod parser;
mod runner;
mod utils;

fn run() -> Result<(), CommonError> {
    let opt = opt::Opt::from_args();

    let agent = agents::get_current_agent()?;

    let cmd = parser::Parser::parser_opt(&opt)?.gene_command(agent);

    match cmd {
        Some(str) => {
            println!("{}", str);

            if !opt.debug {
                runner::Runner::run(str)?;
            }

            Ok(())
        }
        None => Ok(()),
    }
}

fn main() -> () {
    match run() {
        Ok(_) => (),
        Err(err) => eprintln!("{}", err),
    }
}
