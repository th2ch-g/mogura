mod rama;

#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct Residue {
    name: String,
    atoms: Vec<Atom>,
}

impl Residue {
    pub fn new(name: String, atoms: Vec<Atom>) -> Self {
        Self { name, atoms }
    }
}

#[derive(Debug, Clone)]
pub struct Atom {
    name: String,
    x: f32,
    y: f32,
    z: f32,
}

impl Atom {
    pub fn new(name: String, x: f32, y: f32, z: f32) -> Self {
        Self { name, x, y, z }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum SSAlgorithm {
    Ramachandran,
    DSSP,
    STRIDE,
    SST,
}

#[derive(Debug, Clone, PartialEq)]
pub enum SS {
    // DSSP v4, https://doi.org/10.1021/acs.jcim.3c01344
    H, // 4-helix (alpha-helix)
    // B,     // residue in isolated beta-bridge (beta-bridge)
    E, // extended strand participates in beta-ladder (beta-strand)
    // G,     // 3-helix (3_10-helix)
    // I,     // 5-helix (pi-helix)
    // P,     // kappa-helix (polyproline II helix)
    // S,     // bend
    // T,     // H-bonded turn
    // Break, // =, !, break
    Loop, // ~, <space> loop
}

pub fn assign_ss(residues_in_protein: &Vec<Residue>, algo: SSAlgorithm) -> Vec<SS> {
    match algo {
        SSAlgorithm::Ramachandran => rama::assign_ss(residues_in_protein),
        SSAlgorithm::DSSP => todo!(),
        SSAlgorithm::STRIDE => todo!(),
        SSAlgorithm::SST => todo!(),
    }
}

#[allow(unused)]
#[cfg(test)]
mod tests {
    use crate::*;
    use itertools::Itertools;
    use mogura_io::prelude::*;

    struct Atoms(pub Vec<mogura_io::prelude::Atom>); // to aviod E0117

    impl From<Atoms> for Vec<crate::Residue> {
        fn from(atoms_in_protein: Atoms) -> Self {
            atoms_in_protein
                .0
                .into_iter()
                .chunk_by(|atom| atom.residue_name().to_string())
                .into_iter()
                .map(|(res_name, group)| crate::Residue {
                    name: res_name.to_string(),
                    atoms: group
                        .map(|atom| crate::Atom {
                            name: atom.atom_name().to_string(),
                            x: atom.x(),
                            y: atom.y(),
                            z: atom.z(),
                        })
                        .collect(),
                })
                .collect()
        }
    }

    // #[test]
    // fn pdb_5AWL_rama() {
    //     let pdb = pollster::block_on(async { PDBData::download("5AWL").await });
    //
    //     let protein = match pdb {
    //         Ok(pdb) => pdb.protein(),
    //         Err(err) => panic!("PDB load error: {}", err),
    //     };
    //
    //     let ss = assign_ss(&Atoms(protein).into(), SSAlgorithm::Ramachandran);
    //
    //     // gmx 2024.4
    //     // ~~~SSS~~~~
    //     // assert_eq!(ss, vec![SS::Loop, SS::Loop, SS::Loop, SS::S, SS::S, SS::S, SS::Loop, SS::Loop, SS::Loop, SS::Loop]);
    //
    //     assert_eq!(
    //         ss,
    //         vec![
    //             SS::Loop,
    //             SS::Loop,
    //             SS::Loop,
    //             SS::E,
    //             SS::E,
    //             SS::E,
    //             SS::Loop,
    //             SS::Loop,
    //             SS::Loop,
    //             SS::Loop
    //         ]
    //     );
    // }

    // #[test]
    // fn pdb_8gng() {
    //     let pdb = pollster::block_on(async { PDBData::download("8GNG").await });
    //
    //     let protein = match pdb {
    //         Ok(pdb) => pdb.protein(),
    //         Err(err) => panic!("PDB load error: {}", err),
    //     };
    //
    //     let ss = assign_ss(&Atoms(protein).into(), SSAlgorithm::Ramachandran);
    //
    //     dbg!(&ss);
    //
    //     panic!();
    // }
    //
    // #[test]
    // fn gro_a2a_simple() {
    //     let gro = structure_loader("../../example-input/a2a.gro");
    //
    //     let protein = gro.protein();
    //
    //     let ss = assign_ss(&Atoms(protein).into(), SSAlgorithm::Ramachandran);
    //
    //     dbg!(&ss);
    //
    //     panic!();
    // }
}
