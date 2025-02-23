mod input;
mod structure_data;
mod trajectory_data;

pub mod prelude {
    pub use crate::structure_data::gro::GroData;
    pub use crate::structure_data::pdb::PDBData;
    pub use crate::structure_data::{
        structure_loader, structure_loader_from_content, Atom, Element, StructureData,
    };
    pub use crate::trajectory_data::{trajectory_loader, TrajectoryData};
}
