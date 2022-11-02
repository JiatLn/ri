use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Ri", about = "A rust version ni.")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    #[structopt(short, long)]
    pub frozen: bool,

    #[structopt(short, long)]
    pub debug: bool,
}

#[derive(StructOpt, Debug, Clone)]
pub enum SubCommand {
    /// uninstall package
    ///
    /// TODO:
    /// add description
    Un { package_name: String },

    /// run
    R { run_name: Option<String> },

    #[structopt(external_subcommand)]
    Other(Vec<String>),
}
