use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct MainArg {
    /// Path to structure file (e.g. pdb,gro...)
    pub structure_file: Option<String>,
}

pub fn arg() -> MainArg {
    MainArg::parse()
}
