#[derive(Clone, Debug)]
pub struct Bond {
    pub pair1: usize,
    pub pair2: usize,
    pub ty1: AtomType,
    pub ty2: AtomType,
}

impl Bond {
    pub fn new(pair1: usize, pair2: usize, ty1: &str, ty2: &str) -> Self {
        Self {
            pair1,
            pair2,
            ty1: Self::str2atomtype(ty1),
            ty2: Self::str2atomtype(ty2),
        }
    }

    pub fn str2atomtype(ty: &str) -> AtomType {
        match ty {
            "C" => AtomType::C,
            "O" => AtomType::O,
            "N" => AtomType::N,
            "H" => AtomType::H,
            _ => AtomType::Unknown,
        }
    }

    pub fn distance(&self, atoms: &[crate::pdb::PDBAtom]) -> f32 {
        ((atoms[self.pair1].x - atoms[self.pair2].x).powi(2)
            + (atoms[self.pair1].y - atoms[self.pair2].y).powi(2)
            + (atoms[self.pair1].z - atoms[self.pair2].z).powi(2))
        .powf(0.5)
    }

    pub fn is_formed(&self, pdbatoms: &[crate::pdb::PDBAtom]) -> bool {
        // self.distance(pdbatoms) <= 2.0
        self.distance(pdbatoms) <= 1.6
    }
}

#[derive(Debug, Clone)]
pub enum AtomType {
    C,
    O,
    N,
    H,
    S,
    P,
    Ca,
    K,
    Cl,
    Na,
    Unknown,
}
