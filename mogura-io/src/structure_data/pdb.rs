use crate::structure_data::*;

#[cfg(target_arch = "wasm32")]
use wasm_bindgen::prelude::*;

#[derive(Clone, Debug)]
pub struct PDBData {
    atoms: Vec<Atom>,
    residues: Vec<Residue>,
}

impl StructureData for PDBData {
    fn load(structure_file: &str) -> Self {
        let content = std::fs::read_to_string(structure_file).unwrap();
        Self::load_from_content(&content)
    }

    fn atoms(&self) -> &Vec<Atom> {
        &self.atoms
    }

    fn residues(&self) -> &Vec<Residue> {
        &self.residues
    }
}

impl PDBData {
    pub fn load_from_content(content: &str) -> Self {
        let reader = std::io::BufReader::new(std::io::Cursor::new(content));

        // let (input_pdb, _errors) = pdbtbx::open_pdb_raw(
        //     reader,
        //     pdbtbx::Context::show("a.pdb"), // random name
        //     pdbtbx::StrictnessLevel::Loose,
        // )
        // .unwrap();

        let (input_pdb, _errors) = pdbtbx::ReadOptions::new()
            .set_format(pdbtbx::Format::Pdb)
            .set_level(pdbtbx::StrictnessLevel::Loose)
            .read_raw(reader)
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
                            element: if let Some(element) = atom.element() {
                                Element::from_symbol(element.symbol())
                            } else {
                                None
                            },
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

    #[cfg(not(target_arch = "wasm32"))]
    pub async fn download(pdbid: &str) -> anyhow::Result<Self, anyhow::Error> {
        let response = reqwest::blocking::Client::new()
            .get(format!("https://files.rcsb.org/view/{}.pdb", pdbid))
            .send()?;
        let status_code = response.status().as_u16();
        let content = response.text()?;

        if status_code == 200 {
            Ok(Self::load_from_content(&content))
        } else {
            Err(anyhow::anyhow!("Failed to download PDB file for {}", pdbid))
        }
    }

    #[cfg(target_arch = "wasm32")]
    pub async fn download(pdbid: &str) -> anyhow::Result<Self, anyhow::Error> {
        let mut opts = web_sys::RequestInit::new();
        opts.method("GET");
        opts.mode(web_sys::RequestMode::Cors);
        let url = format!("https://files.rcsb.org/view/{}.pdb", pdbid);
        let request = web_sys::Request::new_with_str_and_init(&url, &opts).unwrap();
        let window = gloo::utils::window();
        let resp_value = wasm_bindgen_futures::JsFuture::from(window.fetch_with_request(&request))
            .await
            .unwrap();
        let resp: web_sys::Response = resp_value.dyn_into().unwrap();
        let text = wasm_bindgen_futures::JsFuture::from(resp.text().unwrap())
            .await
            .unwrap();

        Ok(Self::load_from_content(&text.as_string().unwrap()))
    }
}
