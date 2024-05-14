use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct MainArg {
    /// Path to PDB file (.pdb)
    pub pdbfile: Option<String>,
}

pub fn arg() -> MainArg {
    MainArg::parse()
}
