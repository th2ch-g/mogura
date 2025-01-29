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
    fn secondary_structure(&self, mode: SecondaryStructureAlgorithms) -> Vec<SecondaryStructureType> {
        let residues = self.residues();
        let n = residues.len();
        let mut sstype = vec![SecondaryStructureType::Loop; n];
        let mut hydrogen_bonds = vec![vec![]; n];

        match mode {
            SecondaryStructureAlgorithms::DSSP => {
                // check hydrogen bonds in all-pair
                for i in 0..n {
                    for j in 0..i {
                        let prev_residue = &residues[i];
                        let residue = &residues[j];

                        let (prev_residue_O, prev_residue_N, prev_residue_C, _) =
                            if let Some(prev_residue) = prev_residue.backbone() {
                                (prev_residue.0, prev_residue.1, prev_residue.2, prev_residue.3)
                            } else {
                                continue
                            };
                        let (residue_O, residue_N, residue_C, _) =
                            if let Some(residue) = residue.backbone() {
                                (residue.0, residue.1, residue.2, residue.3)
                            } else {
                                continue
                            };

                        let r_H = residue_C.distance(&residue_O);

                        let r_ON = prev_residue_O.distance(&residue_N);
                        let r_CH = prev_residue_C.distance(&residue_N) + r_H;
                        let r_OH = prev_residue_O.distance(&residue_C) + r_H;
                        let r_CN = prev_residue_C.distance(&residue_O);

                        let energy = 0.084 * 332.0 *  (1.0 / r_ON + 1.0 / r_CH - 1.0 / r_OH - 1.0 / r_CN);

                        if energy < -0.5 {
                            hydrogen_bonds[i].push(j);
                            hydrogen_bonds[j].push(i);
                        }
                    }
                }
                // determine sstype
                // check DSSPType::H
                for i in 0..n-4 {
                    if hydrogen_bonds[i].contains(&(i+4)) {
                        sstype[i] = SecondaryStructureType::H;
                        sstype[i+1] = SecondaryStructureType::H;
                        sstype[i+2] = SecondaryStructureType::H;
                        sstype[i+4] = SecondaryStructureType::H;
                    }
                }

                // check DSSPType::E
                for i in 0..n {
                    // if hydrogen_bonds[i].len() > 0 {
                    //     sstype[i] = SecondaryStructureType::DSSPType(DSSPType::E);
                    // }
                }

            }
            _ => {
                unimplemented!("{:?} is not supported", mode);
            }
        }
        sstype
    }
    fn residues(&self) -> &Vec<Residue>;
}

#[derive(Debug, Clone)]
pub enum SecondaryStructureAlgorithms {
    DSSP,
    // STRIDE,
    // SST,
}

#[derive(Debug, Clone)]
pub enum SecondaryStructureType {
    // DSSP v4.
    // https://doi.org/10.1021/acs.jcim.3c01344
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
pub struct Residue {
    id: usize,
    model_id: usize,
    chain_name: String,
    residue_id: isize,
    residue_name: String,
    atoms: Vec<Atom>,
}

impl Residue {
    pub fn atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    pub fn center(&self) -> [f32; 3] {
        let mut center = [0., 0., 0.];
        for atom in &self.atoms {
            center[0] += atom.x();
            center[1] += atom.y();
            center[2] += atom.z();
        }
        center[0] /= self.atoms.len() as f32;
        center[1] /= self.atoms.len() as f32;
        center[2] /= self.atoms.len() as f32;
        center
    }
    pub fn backbone(&self) -> Option<(Atom, Atom, Atom, Atom)> {
        let atom_O = if let Some(atom_O) = self
            .atoms
            .iter()
            .find(|atom| atom.atom_name == "O") {
                atom_O
            } else {
                return None;
            };
        let atom_N = if let Some(atom_N) = self
            .atoms
            .iter()
            .find(|atom| atom.atom_name == "N") {
                atom_N
            } else {
                return None;
            };
        let atom_C = if let Some(atom_C) = self
            .atoms
            .iter()
            .find(|atom| atom.atom_name == "C") {
                atom_C
            } else {
                return None;
            };
        let atom_CA = if let Some(atom_CA) = self
            .atoms
            .iter()
            .find(|atom| atom.atom_name == "CA") {
                atom_CA
            } else {
                return None;
            };
        Some((atom_O.clone(), atom_N.clone(), atom_C.clone(), atom_CA.clone()))
    }
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
