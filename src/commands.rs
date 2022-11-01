#[derive(Debug, PartialEq, Eq, Hash)]
pub enum Command {
    Agent,
    Add,
    Install,
    Run,
    Unkown,
    Frozen,
    Global,
    Upgrade,
    Uninstall,
    GlobalUninstall,
    Execute,
    UpgradeInteractive,
}
