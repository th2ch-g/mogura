use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct MainArg {
    /// Path to PDB file (.pdb, .cif)
    pub pdbfile: Option<String>,
}

pub fn arg() -> MainArg {
    MainArg::parse()
}
