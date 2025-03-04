use crate::structure_data::*;
use itertools::Itertools;

#[cfg(feature = "groan_rs")]
use groan_rs::prelude::*;

#[derive(Clone, Debug)]
pub struct GroData {
    atoms: Vec<crate::structure_data::Atom>,
    residues: Vec<Residue>,
}

#[cfg(feature = "groan_rs")]
impl StructureData for GroData {
    fn load(structure_file: &str) -> Self {
        let system = System::from_file(structure_file).unwrap();

        let system_atoms = system.get_atoms_copy();
        let mut atoms = Vec::with_capacity(system_atoms.len());

        for (id, atom) in system_atoms.iter().enumerate() {
            atoms.push(crate::structure_data::Atom {
                id,
                model_id: 0,
                chain_name: match atom.get_chain() {
                    Some(s) => s.to_string(),
                    None => "None".to_string(),
                },
                residue_id: atom.get_residue_number() as isize,
                residue_name: atom.get_residue_name().to_string(),
                atom_id: atom.get_atom_number(),
                atom_name: atom.get_atom_name().to_string(),
                element: Element::from_atom_name(atom.get_atom_name()),
                // convert to angstrom
                x: atom.get_position().unwrap().x * 10.0,
                y: atom.get_position().unwrap().y * 10.0,
                z: atom.get_position().unwrap().z * 10.0,
            });
        }

        let mut residues = Vec::with_capacity(system_atoms.len());
        for (id, (_residue_id, group)) in (&atoms.iter().chunk_by(|a| a.residue_id))
            .into_iter()
            .enumerate()
        {
            let group_atoms: Vec<_> = group.cloned().collect();
            let first_atom = group_atoms.clone().into_iter().next().unwrap();
            residues.push(Residue {
                id,
                model_id: 0,
                chain_name: first_atom.chain_name.clone(),
                residue_id: first_atom.residue_id,
                residue_name: first_atom.residue_name.clone(),
                atoms: group_atoms,
            });
        }

        Self { atoms, residues }
    }

    fn atoms(&self) -> &Vec<crate::structure_data::Atom> {
        &self.atoms
    }

    fn residues(&self) -> &Vec<Residue> {
        &self.residues
    }
}
