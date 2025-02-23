mod dssp;

#[derive(Debug, Clone)]
pub struct Residue {
    atoms: Vec<Atom>,
}

#[derive(Debug, Clone)]
pub struct Atom {
    name: String,
    x: f32,
    y: f32,
    z: f32,
}

#[derive(Clone, Debug, PartialEq)]
pub enum SSAlgorithm {
    DSSP, // currently DSSP classify only H or B or Loop
          // STRIDE,
          // SST,
}

#[derive(Debug, Clone)]
pub enum SS {
    // DSSP v4, https://doi.org/10.1021/acs.jcim.3c01344
    H,     // 4-helix (alpha-helix)
    B,     // residue in isolated beta-bridge (beta-bridge)
    E,     // extended strand participates in beta-ladder (beta-strand)
    G,     // 3-helix (3_10-helix)
    I,     // 5-helix (pi-helix)
    P,     // kappa-helix (polyproline II helix)
    S,     // bend
    T,     // H-bonded turn
    Break, // =, !, break
    Loop,  // ~, <space> loop
}

pub fn assign_ss(atoms_in_protein: &Vec<Atom>, algo: SSAlgorithm) -> Vec<SS> {
    match algo {
        SSAlgorithm::DSSP => dssp::assign_ss(atoms_in_protein),
        _ => todo!(),
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use mogura_io::prelude::*;

    fn assign_ss_from_moguraIO(
        atoms_in_protein: &Vec<mogura_io::prelude::Atom>,
        algo: SSAlgorithm,
    ) -> Vec<SS> {
        assign_ss(&convert_from_mogura_io(atoms_in_protein), algo)
    }

    fn convert_from_mogura_io(
        atoms_in_protein: &Vec<mogura_io::prelude::Atom>,
    ) -> Vec<crate::Atom> {
        let mut res = Vec::with_capacity(atoms_in_protein.len());
        for atom in atoms_in_protein {
            res.push(crate::Atom {
                name: atom.atom_name().to_string(),
                x: atom.x(),
                y: atom.y(),
                z: atom.z(),
            });
        }
        res
    }

    #[test]
    fn pdb_5AWL_dssp_simple() {
        let pdb = pollster::block_on(async { PDBData::download("5AWL").await });

        let protein = match pdb {
            Ok(pdb) => pdb.protein(),
            Err(err) => panic!("PDB load error: {}", err),
        };

        let ss = assign_ss_from_moguraIO(&protein, SSAlgorithm::DSSP);

        dbg!(&ss);

        panic!();
    }
}
