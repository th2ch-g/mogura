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
    fn bonds(&self) -> Vec<(usize, usize)> {
        const GENERAL_BOND_CUTOFF: f32 = 1.6;
        let n = self.atoms().len();
        let mut bonds = Vec::with_capacity(n * n);
        let atoms = self.atoms();
        for i in 0..n {
            for j in 0..i {
                if atoms[i].distance(&atoms[j]) <= GENERAL_BOND_CUTOFF {
                    bonds.push((i, j));
                }
            }
        }
        bonds
    }
    fn secondary_structure(&self, mode: SecondaryStructureAlgothms) -> Vec<SecondaryStructureType> {
        let atoms = self.atoms();
        let n = atoms.len();
        let mut sstype = Vec::with_capacity(n);
        match mode {
            SecondaryStructureAlgothms::DSSP => {
            }
            _ => {
                unimplemented!("{:?} is not supported", mode);
            }
        }
        sstype
    }
}

#[derive(Debug, Clone)]
pub enum SecondaryStructureAlgothms {
    DSSP,
    // STRIDE,
    // SST,
}

#[derive(Debug, Clone)]
pub enum SecondaryStructureType {
    DSSPType,
    // STRIDEType,
    // SSTType,
}

// DSSP v4.
// https://doi.org/10.1021/acs.jcim.3c01344
#[derive(Debug, Clone)]
pub enum DSSPType {
    H, // 4-helix (alpha-helix)
    B, // residue in isolated beta-bridge (beta-bridge)
    E, // extended strand participates in beta-ladder (beta-strand)
    G, // 3-helix (3_10-helix)
    I, // 5-helix (pi-helix)
    P, // kappa-helix (polyproline II helix)
    S, // bend
    T, // H-bonded turn
    Break, // =, !, break
    Loop,  // ~, <space> loop
}

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
    pub fn distance2(&self, other: &Atom) -> f32 {
        let dx = self.x - other.x;
        let dy = self.y - other.y;
        let dz = self.z - other.z;
        dx.powi(2) + dy.powi(2) + dz.powi(2)
    }
    pub fn distance(&self, other: &Atom) -> f32 {
        self.distance2(other).sqrt()
    }
}

// pub struct Element {
//
// }
