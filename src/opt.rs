use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "Ri", about = "A rust version ni.")]
pub struct Opt {
    #[structopt(subcommand)]
    pub cmd: Option<SubCommand>,

    #[structopt(short, long)]
    pub frozen: bool,

    /// Debug mode will not run the command
    #[structopt(short, long)]
    pub debug: bool,

    #[structopt(short, long)]
    pub global: bool,

    /// Remove node_modules
    #[structopt(short, long)]
    pub clean: bool,
}

#[derive(StructOpt, Debug, Clone)]
pub enum SubCommand {
    /// Uninstall package
    Un { package_name: Vec<String> },

    /// Run script
    R { run_name: Option<String> },

    #[structopt(external_subcommand)]
    Other(Vec<String>),
}
