mod input;
mod structure_data;

pub mod prelude {
    pub use crate::structure_data::pdb::PDBData;
    pub use crate::structure_data::{structure_loader, Atom, StructureData, SecondaryStructureAlgorithms, SecondaryStructureType};
}
