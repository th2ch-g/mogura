mod input;
mod structure_data;
mod trajectory_data;

pub mod prelude {
    pub use crate::structure_data::gro::GroData;
    pub use crate::structure_data::pdb::PDBData;
    pub use crate::structure_data::{
        Atom, Element, GENERAL_BOND_CUTOFF, StructureData, structure_loader,
        structure_loader_from_content,
    };
    pub use crate::trajectory_data::{TrajectoryData, trajectory_loader};
}
