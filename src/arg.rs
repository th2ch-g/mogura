use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(version, about, arg_required_else_help = true)]
pub struct MainArg {
    /// Path to PDB file (.pdb, .pdb.gz)
    pub pdbfile: String,
}

pub fn arg() -> MainArg {
    MainArg::parse()
}
