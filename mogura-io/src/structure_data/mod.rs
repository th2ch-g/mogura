pub mod pdb;
use crate::structure_data::pdb::PDBData;

pub fn structure_loader(structure_file: &str) -> impl StructureData {
    let extension = std::path::Path::new(structure_file)
        .extension()
        .and_then(|ext| ext.to_str());
    if let Some(extension) = extension {
        match extension {
            "pdb" => PDBData::load(structure_file),
            "gro" => {
                todo!();
            }
            _ => {
                unimplemented!("This extension is not supported.")
            }
        }
    } else {
        panic!("structure_file: {} has no extension.", structure_file);
    }
}

pub trait StructureData: Sync + Send {
    fn load(structure_file: &str) -> Self
    where
        Self: Sized;
    // fn export(output_path: &str);
    fn atoms(&self) -> &Vec<Atom>;
    fn center(&self) -> [f32; 3] {
        let mut center = [0., 0., 0.];
        for atom in self.atoms() {
            center[0] += atom.x();
            center[1] += atom.y();
            center[2] += atom.z();
        }
        center[0] /= self.atoms().len() as f32;
        center[1] /= self.atoms().len() as f32;
        center[2] /= self.atoms().len() as f32;
        center
    }
}

// impl Clone for Box<dyn StructureData> {
//     fn clone(&self) -> Box<dyn StructureData> {
//         self.clone_box()
//     }
// }

#[derive(Debug, Clone)]
pub struct Atom {
    id: usize,
    model_id: usize,
    chain_name: String,
    residue_id: isize,
    residue_name: String,
    atom_id: usize,
    atom_name: String,
    element: Option<pdbtbx::Element>,
    x: f32, // angstrom
    y: f32, // angstrom
    z: f32, // angstrom
            // charge: usize,
}

impl Atom {
    pub fn x(&self) -> f32 {
        self.x
    }
    pub fn y(&self) -> f32 {
        self.y
    }
    pub fn z(&self) -> f32 {
        self.z
    }
    pub fn xyz(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}

// pub struct Element {
//
// }
