use itertools::Itertools;

const ATOMCAPA: usize = 50000;

#[derive(Clone, Debug)]
pub struct PDBSystem {
    pub atoms: Vec<PDBAtom>,
    pub bonds: Vec<crate::bond::Bond>,
    pub vertices: Vec<crate::model::Vertex>,
    pub indecies: Vec<crate::model::Index>,
}

impl From<&pdbtbx::PDB> for PDBSystem {
    fn from(from: &pdbtbx::PDB) -> Self {
        let mut atoms = Vec::with_capacity(ATOMCAPA);
        let mut id = 0;
        for i in 0..from.model_count() {
            let model = from.model(i).unwrap();
            for j in 0..model.chain_count() {
                let chain = model.chain(j).unwrap();
                for k in 0..chain.residue_count() {
                    let residue = chain.residue(k).unwrap();
                    for l in 0..residue.atom_count() {
                        let atom = residue.atom(l).unwrap();
                        atoms.push(PDBAtom {
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
                            x: atom.x() as f32,
                            y: atom.y() as f32,
                            z: atom.z() as f32,
                        });
                        id += 1;
                    }
                }
            }
        }
        Self {
            atoms,
            bonds: vec![],
            vertices: vec![],
            indecies: vec![],
        }
    }
}

impl PDBSystem {
    pub fn center(&self) -> [f32; 3] {
        let mut x = 0.0;
        let mut y = 0.0;
        let mut z = 0.0;
        for atom in self.atoms.iter() {
            x += atom.x;
            y += atom.y;
            z += atom.z;
        }
        [
            x / self.atoms.len() as f32,
            y / self.atoms.len() as f32,
            z / self.atoms.len() as f32,
        ]
    }

    pub fn set_line_model(&mut self) {
        self.vertices = Vec::with_capacity(ATOMCAPA);
        self.indecies = Vec::with_capacity(ATOMCAPA);
        for atom in self.atoms.iter() {
            self.vertices.push(crate::model::Vertex {
                position: atom.xyz(),
                normal: atom.xyz(),
                color: [0.0, 0.5, 1.0],
            });
        }
        for bond in self.bonds.iter() {
            self.indecies.push(crate::model::Index {
                ids: bond.pair1 as u16,
            });
            self.indecies.push(crate::model::Index {
                ids: bond.pair2 as u16,
            });
        }
    }

    pub fn which_group_is_selected(
        &self,
        mouse_gazer: ([f32; 3], [f32; 3]),
        is_touched: &mut bool,
        group_to_select: &crate::settings::GroupToSelect,
    ) -> std::collections::HashSet<usize> {
        // calc mouse gaze vector
        let (gazer_p, gazer_q) = mouse_gazer;

        // which atoms is nearest
        let mut nearest_atom: Option<PDBAtom> = None;
        let mut nearest_dist = std::f32::INFINITY;
        const ALLOWED_RADIUS: f32 = 1.0;
        for atom in self.atoms.iter() {
            let xyz = atom.xyz();
            let t = (gazer_q[0] * (xyz[0] - gazer_p[0])
                + gazer_q[1] * (xyz[1] - gazer_p[1])
                + gazer_q[2] * (xyz[2] - gazer_p[2]))
                / (gazer_q[0].powi(2) + gazer_q[1].powi(2) + gazer_q[2].powi(2));
            let dist = ((gazer_p[0] + t * gazer_q[0] - xyz[0]).powi(2)
                + (gazer_p[1] + t * gazer_q[1] - xyz[1]).powi(2)
                + (gazer_p[2] + t * gazer_q[2] - xyz[2]).powi(2))
            .sqrt();
            if nearest_dist > dist {
                nearest_dist = dist;
                nearest_atom = Some(atom.clone());
            }
            if nearest_dist <= ALLOWED_RADIUS {
                break;
            }
        }
        dbg!(&nearest_atom, &nearest_dist);
        // for debug
        let t = -gazer_p[1] / gazer_q[1];
        println!(
            "x: {}, y: {}, z: {}",
            gazer_p[0] + t * gazer_q[0],
            gazer_p[1] + t * gazer_q[1],
            gazer_p[2] + t * gazer_q[2]
        );

        // translate from atom to group
        let mut group = std::collections::HashSet::new();
        if nearest_dist <= ALLOWED_RADIUS {
            *is_touched = true;
            match group_to_select {
                crate::settings::GroupToSelect::Atoms => {
                    if let Some(atom) = nearest_atom {
                        group.insert(atom.id);
                    }
                }
                crate::settings::GroupToSelect::Residues => {
                    if let Some(nearest_atom) = nearest_atom {
                        for atom in self.atoms.iter() {
                            if (nearest_atom.residue_id == atom.residue_id)
                                && (nearest_atom.chain_name == atom.chain_name)
                            {
                                group.insert(atom.id);
                            }
                        }
                    }
                }
                crate::settings::GroupToSelect::Molecules => {
                    if let Some(nearest_atom) = nearest_atom {
                        for atom in self.atoms.iter() {
                            if nearest_atom.chain_name == atom.chain_name {
                                group.insert(atom.id);
                            }
                        }
                    }
                }
            }
        } else {
            *is_touched = false;
        }
        dbg!(&is_touched);

        group
    }

    pub fn move_with_nnp(
        &mut self,
        delta_x: f32,
        delta_y: f32,
        move_vec: ([f32; 3], [f32; 3]),
        selected_group: &crate::settings::SelectedGroup,
    ) {
    }

    pub fn move_without_nnp(
        &mut self,
        delta_x: f32,
        delta_y: f32,
        move_vec: ([f32; 3], [f32; 3]),
        selected_group: &crate::settings::SelectedGroup,
    ) {
        // prepare move_x, move_y, move_z
        const MOVE_SCALE: f32 = 100.0;
        let move_x = -delta_y / MOVE_SCALE * move_vec.0[0] + delta_x / MOVE_SCALE * move_vec.1[0];
        let move_y = -delta_y / MOVE_SCALE * move_vec.0[1] + delta_x / MOVE_SCALE * move_vec.1[1];
        let move_z = -delta_y / MOVE_SCALE * move_vec.0[2] + delta_x / MOVE_SCALE * move_vec.1[2];

        // apply
        for i in 0..self.atoms.len() {
            if selected_group.atoms.contains(&self.atoms[i].id) {
                self.atoms[i].x += move_x;
                self.atoms[i].y += move_y;
                self.atoms[i].z += move_z;
            }
        }
    }

    pub fn update_bonds_all(&mut self) {
        self.bonds = Vec::with_capacity(ATOMCAPA);
        for comb in (0..self.atoms.len()).combinations(2) {
            let bond = crate::bond::Bond::new(
                comb[0],
                comb[1],
                &self.atoms[comb[0]].atom_name,
                &self.atoms[comb[1]].atom_name,
            );
            if bond.is_formed(&self.atoms) {
                self.bonds.push(bond);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct PDBAtom {
    pub id: usize,
    pub model_id: usize,
    pub chain_name: String,
    pub residue_id: isize,
    pub residue_name: String,
    pub atom_id: usize,
    pub atom_name: String,
    pub x: f32, // angstrom
    pub y: f32,
    pub z: f32,
    // pub charge: usize,
}

impl PDBAtom {
    pub fn xyz(&self) -> [f32; 3] {
        [self.x, self.y, self.z]
    }
}
