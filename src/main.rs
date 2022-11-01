use structopt::StructOpt;

mod agents;
mod commands;
mod opt;
mod parse;
mod utils;

fn main() {
    let opt = opt::Opt::from_args();

    let agent = agents::get_current_agent();

    dbg!(&opt);

    let mut parser = parse::Parser::parser_opt(opt);

    println!("The command is:\n{}", parser.gene_command(agent));
}
