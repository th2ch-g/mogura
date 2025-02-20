use clap::Parser;

#[derive(Parser, Debug, Clone)]
#[clap(version, about)]
pub struct MainArg {
    /// Path to structure file (e.g. pdb, gro...)
    pub structure_file: Option<String>,

    /// Path to trajectory file (e.g. xtc)
    pub trajectory_file: Option<String>,
}

impl MainArg {
    pub fn new() -> Self {
        Self::parse()
    }
}
