use crate::structure_data::*;

#[derive(Clone)]
pub struct PDBData {
    atoms: Vec<Atom>,
    residues: Vec<Residue>,
}

impl StructureData for PDBData {
    fn load(structure_file: &str) -> Self {
        let content = std::fs::read_to_string(structure_file).unwrap();
        let reader = std::io::BufReader::new(std::io::Cursor::new(content));
        let (input_pdb, _errors) = pdbtbx::open_pdb_raw(
            reader,
            pdbtbx::Context::show("a.pdb"), // random anme
            pdbtbx::StrictnessLevel::Loose,
        )
        .unwrap();
        let mut id = 0;
        let mut atoms = Vec::new();
        let mut residues = Vec::new();

        for i in 0..input_pdb.model_count() {
            let model = input_pdb.model(i).unwrap();
            for j in 0..input_pdb.chain_count() {
                let chain = input_pdb.chain(j).unwrap();
                for k in 0..chain.residue_count() {
                    let residue = chain.residue(k).unwrap();
                    let mut tmp_atoms = Vec::new();
                    for l in 0..residue.atom_count() {
                        let atom = residue.atom(l).unwrap();
                        let persed_atom = Atom {
                            id,
                            model_id: model.serial_number(),
                            chain_name: chain.id().to_string(),
                            residue_id: residue.serial_number(),
                            residue_name: match residue.name() {
                                Some(name) => name.to_string(),
                                None => "None".to_string(),
                            },
                            atom_id: atom.serial_number(),
                            atom_name: atom.name().to_string(),
                            element: atom.element().copied(),
                            x: atom.x() as f32,
                            y: atom.y() as f32,
                            z: atom.z() as f32,
                        };
                        atoms.push(persed_atom.clone());
                        tmp_atoms.push(persed_atom.clone());
                        id += 1;
                    }

                    residues.push(Residue {
                        id,
                        model_id: model.serial_number(),
                        chain_name: chain.id().to_string(),
                        residue_id: residue.serial_number(),
                        residue_name: match residue.name() {
                            Some(name) => name.to_string(),
                            None => "None".to_string(),
                        },
                        atoms: tmp_atoms,
                    });
                }
            }
        }

        Self { atoms, residues }
    }

    fn atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    fn residues(&self) -> &Vec<Residue> {
        &self.residues
    }
}
