pub mod gro;
pub mod pdb;
use crate::structure_data::gro::GroData;
use crate::structure_data::pdb::PDBData;

pub fn structure_loader(structure_file: &str) -> Box<dyn StructureData> {
    let extension = std::path::Path::new(structure_file)
        .extension()
        .and_then(|ext| ext.to_str());
    if let Some(extension) = extension {
        match extension {
            "pdb" => Box::new(PDBData::load(structure_file)),
            "gro" => {
                #[cfg(feature = "groan_rs")]
                {
                    Box::new(GroData::load(structure_file))
                }

                #[cfg(not(feature = "groan_rs"))]
                {
                    unimplemented!("This extension is not supported.");
                }
            },
            _ => {
                unimplemented!("This extension is not supported.")
            }
        }
    } else {
        panic!("structure_file: {} has no extension.", structure_file);
    }
}

pub fn structure_loader_from_content(content: &str, extension: &str) -> Box<dyn StructureData> {
    match extension {
        "pdb" => Box::new(PDBData::load_from_content(content)),
        "gro" => {
            unimplemented!("gro is not supported for loading from content")
        }
        _ => {
            unimplemented!("This extension is not supported.")
        }
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
    fn secondary_structure(
        &self,
        mode: SecondaryStructureAlgorithms,
    ) -> Vec<SecondaryStructureType> {
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

                        let (prev_residue_o, _prev_residue_n, prev_residue_c, _prev_residue_ca) =
                            if let Some(prev_residue) = prev_residue.backbone() {
                                (
                                    prev_residue.0,
                                    prev_residue.1,
                                    prev_residue.2,
                                    prev_residue.3,
                                )
                            } else {
                                continue;
                            };
                        let (residue_o, residue_n, residue_c, _residue_ca) =
                            if let Some(residue) = residue.backbone() {
                                (residue.0, residue.1, residue.2, residue.3)
                            } else {
                                continue;
                            };

                        let r_h = residue_c.distance(&residue_o);

                        let r_on = prev_residue_o.distance(&residue_n);
                        let r_ch = prev_residue_c.distance(&residue_n) + r_h;
                        let r_oh = prev_residue_o.distance(&residue_c) + r_h;
                        let r_cn = prev_residue_c.distance(&residue_o);

                        let energy =
                            0.084 * 332.0 * (1.0 / r_on + 1.0 / r_cn - 1.0 / r_oh - 1.0 / r_cn);

                        if energy < -0.5 {
                            hydrogen_bonds[i].push(j);
                            hydrogen_bonds[j].push(i);
                        }
                    }
                }
                // determine sstype
                // check DSSPType::H
                for i in 0..n - 4 {
                    if hydrogen_bonds[i].contains(&(i + 4)) {
                        sstype[i] = SecondaryStructureType::H;
                        sstype[i + 1] = SecondaryStructureType::H;
                        sstype[i + 2] = SecondaryStructureType::H;
                        sstype[i + 4] = SecondaryStructureType::H;
                    }
                }

                // check DSSPType::E
                // for i in 0..n {
                //     // if hydrogen_bonds[i].len() > 0 {
                //     //     sstype[i] = SecondaryStructureType::DSSPType(DSSPType::E);
                //     // }
                // }
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
        let atom_o = self.atoms.iter().find(|atom| atom.atom_name == "O")?;
        let atom_n = self.atoms.iter().find(|atom| atom.atom_name == "N")?;
        let atom_c = self.atoms.iter().find(|atom| atom.atom_name == "C")?;
        let atom_ca = self.atoms.iter().find(|atom| atom.atom_name == "CA")?;

        Some((
            atom_o.clone(),
            atom_n.clone(),
            atom_c.clone(),
            atom_ca.clone(),
        ))
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
    element: Option<Element>,
    x: f32, // angstrom
    y: f32, // angstrom
    z: f32, // angstrom
            // charge: usize,
}

impl Atom {
    pub fn id(&self) -> usize {
        self.id
    }
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
    pub fn element(&self) -> &Option<Element> {
        &self.element
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
    pub fn element_symbol(&self) -> &str {
        self.element
            .as_ref()
            .map(|element| element.to_symbol())
            .unwrap_or("")
    }
}

// ref: https://github.com/douweschulte/pdbtbx/blob/master/src/structs/elements.rs
#[derive(Debug, Copy, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum Element {
    /// Element Hydrogen (H) atomic number: 1
    H = 1,
    /// Element Helium (He) atomic number: 2
    He,
    /// Element Lithium (Li) atomic number: 3
    Li,
    /// Element Beryllium (Be) atomic number: 4
    Be,
    /// Element Boron (B) atomic number: 5
    B,
    /// Element Carbon (C) atomic number: 6
    C,
    /// Element Nitrogen (N) atomic number: 7
    N,
    /// Element Oxygen (O) atomic number: 8
    O,
    /// Element Fluorine (F) atomic number: 9
    F,
    /// Element Neon (Ne) atomic number: 10
    Ne,
    /// Element Sodium (Na) atomic number: 11
    Na,
    /// Element Magnesium (Mg) atomic number: 12
    Mg,
    /// Element Aluminium (Al) atomic number: 13
    Al,
    /// Element Silicon (Si) atomic number: 14
    Si,
    /// Element Phosphorus (P) atomic number: 15
    P,
    /// Element Sulfur (S) atomic number: 16
    S,
    /// Element Chlorine (Cl) atomic number: 17
    Cl,
    /// Element Argon (Ar) atomic number: 18
    Ar,
    /// Element Potassium (K) atomic number: 19
    K,
    /// Element Calcium (Ca) atomic number: 20
    Ca,
    /// Element Scandium (Sc) atomic number: 21
    Sc,
    /// Element Titanium (Ti) atomic number: 22
    Ti,
    /// Element Vanadium (V) atomic number: 23
    V,
    /// Element Chromium (Cr) atomic number: 24
    Cr,
    /// Element Manganese (Mn) atomic number: 25
    Mn,
    /// Element Iron (Fe) atomic number: 26
    Fe,
    /// Element Cobalt (Co) atomic number: 27
    Co,
    /// Element Nickel (Ni) atomic number: 28
    Ni,
    /// Element Copper (Cu) atomic number: 29
    Cu,
    /// Element Zinc (Zn) atomic number: 30
    Zn,
    /// Element Gallium (Ga) atomic number: 31
    Ga,
    /// Element Germanium (Ge) atomic number: 32
    Ge,
    /// Element Arsenic (As) atomic number: 33
    As,
    /// Element Selenium (Se) atomic number: 34
    Se,
    /// Element Bromine (Br) atomic number: 35
    Br,
    /// Element Krypton (Kr) atomic number: 36
    Kr,
    /// Element Rubidium (Rb) atomic number: 37
    Rb,
    /// Element Strontium (Sr) atomic number: 38
    Sr,
    /// Element Yttrium (Y) atomic number: 39
    Y,
    /// Element Zirconium (Zr) atomic number: 40
    Zr,
    /// Element Niobium (Nb) atomic number: 41
    Nb,
    /// Element Molybdenum (Mo) atomic number: 42
    Mo,
    /// Element Technetium (Tc) atomic number: 43
    Tc,
    /// Element Ruthenium (Ru) atomic number: 44
    Ru,
    /// Element Rhodium (Rh) atomic number: 45
    Rh,
    /// Element Palladium (Pd) atomic number: 46
    Pd,
    /// Element Silver (Ag) atomic number: 47
    Ag,
    /// Element Cadmium (Cd) atomic number: 48
    Cd,
    /// Element Indium (In) atomic number: 49
    In,
    /// Element Tin (Sn) atomic number: 50
    Sn,
    /// Element Antimony (Sb) atomic number: 51
    Sb,
    /// Element Tellurium (Te) atomic number: 52
    Te,
    /// Element Iodine (I) atomic number: 53
    I,
    /// Element Xenon (Xe) atomic number: 54
    Xe,
    /// Element Caesium (Cs) atomic number: 55
    Cs,
    /// Element Barium (Ba) atomic number: 56
    Ba,
    /// Element Lanthanum (La) atomic number: 57
    La,
    /// Element Cerium (Ce) atomic number: 58
    Ce,
    /// Element Praseodymium (Pr) atomic number: 59
    Pr,
    /// Element Neodymium (Nd) atomic number: 60
    Nd,
    /// Element Promethium (Pm) atomic number: 61
    Pm,
    /// Element Samarium (Sm) atomic number: 62
    Sm,
    /// Element Europium (Eu) atomic number: 63
    Eu,
    /// Element Gadolinium (Gd) atomic number: 64
    Gd,
    /// Element Terbium (Tb) atomic number: 65
    Tb,
    /// Element Dysprosium (Dy) atomic number: 66
    Dy,
    /// Element Holmium (Ho) atomic number: 67
    Ho,
    /// Element Erbium (Er) atomic number: 68
    Er,
    /// Element Thulium (Tm) atomic number: 69
    Tm,
    /// Element Ytterbium (Yb) atomic number: 70
    Yb,
    /// Element Lutetium (Lu) atomic number: 71
    Lu,
    /// Element Hafnium (Hf) atomic number: 72
    Hf,
    /// Element Tantalum (Ta) atomic number: 73
    Ta,
    /// Element Tungsten (W) atomic number: 74
    W,
    /// Element Rhenium (Re) atomic number: 75
    Re,
    /// Element Osmium (Os) atomic number: 76
    Os,
    /// Element Iridium (Ir) atomic number: 77
    Ir,
    /// Element Platinum (Pt) atomic number: 78
    Pt,
    /// Element Gold (Au) atomic number: 79
    Au,
    /// Element Mercury (Hg) atomic number: 80
    Hg,
    /// Element Thallium (Tl) atomic number: 81
    Tl,
    /// Element Lead (Pb) atomic number: 82
    Pb,
    /// Element Bismuth (Bi) atomic number: 83
    Bi,
    /// Element Polonium (Po) atomic number: 84
    Po,
    /// Element Astatine (At) atomic number: 85
    At,
    /// Element Radon (Rn) atomic number: 86
    Rn,
    /// Element Francium (Fr) atomic number: 87
    Fr,
    /// Element Radium (Ra) atomic number: 88
    Ra,
    /// Element Actinium (Ac) atomic number: 89
    Ac,
    /// Element Thorium (Th) atomic number: 90
    Th,
    /// Element Protactinium (Pa) atomic number: 91
    Pa,
    /// Element Uranium (U) atomic number: 92
    U,
    /// Element Neptunium (Np) atomic number: 93
    Np,
    /// Element Plutonium (Pu) atomic number: 94
    Pu,
    /// Element Americium (Am) atomic number: 95
    Am,
    /// Element Curium (Cm) atomic number: 96
    Cm,
    /// Element Berkelium (Bk) atomic number: 97
    Bk,
    /// Element Californium (Cf) atomic number: 98
    Cf,
    /// Element Einsteinium (Es) atomic number: 99
    Es,
    /// Element Fermium (Fm) atomic number: 100
    Fm,
    /// Element Mendelevium (Md) atomic number: 101
    Md,
    /// Element Nobelium (No) atomic number: 102
    No,
    /// Element Lawrencium (Lr) atomic number: 103
    Lr,
    /// Element Rutherfordium (Rf) atomic number: 104
    Rf,
    /// Element Dubnium (Db) atomic number: 105
    Db,
    /// Element Seaborgium (Sg) atomic number: 106
    Sg,
    /// Element Bohrium (Bh) atomic number: 107
    Bh,
    /// Element Hassium (Hs) atomic number: 108
    Hs,
    /// Element Meitnerium (Mt) atomic number: 109
    Mt,
    /// Element Darmstadtium (Ds) atomic number: 110
    Ds,
    /// Element Roentgenium (Rg) atomic number: 111
    Rg,
    /// Element Copernicium (Cn) atomic number: 112
    Cn,
    /// Element Nihonium (Nh) atomic number: 113
    Nh,
    /// Element Flerovium (Fl) atomic number: 114
    Fl,
    /// Element Moscovium (Mc) atomic number: 115
    Mc,
    /// Element Livermorium (Lv) atomic number: 116
    Lv,
    /// Element Tennessine (Ts) atomic number: 117
    Ts,
    /// Element Oganesson (Og) atomic number: 118
    Og,
}

impl Element {
    pub fn from_atom_name(atom_name: &str) -> Option<Element> {
        match atom_name.chars().next() {
            Some(c) => match c {
                'H' => Some(Element::H),
                'C' => Some(Element::C),
                'N' => Some(Element::N),
                'O' => Some(Element::O),
                'S' => Some(Element::S),
                _ => None,
            },
            None => None,
        }
    }

    pub fn from_symbol(symbol: &str) -> Option<Element> {
        match symbol {
            s if s.eq_ignore_ascii_case("H") => Some(Element::H),
            s if s.eq_ignore_ascii_case("He") => Some(Element::He),
            s if s.eq_ignore_ascii_case("Li") => Some(Element::Li),
            s if s.eq_ignore_ascii_case("Be") => Some(Element::Be),
            s if s.eq_ignore_ascii_case("B") => Some(Element::B),
            s if s.eq_ignore_ascii_case("C") => Some(Element::C),
            s if s.eq_ignore_ascii_case("N") => Some(Element::N),
            s if s.eq_ignore_ascii_case("O") => Some(Element::O),
            s if s.eq_ignore_ascii_case("F") => Some(Element::F),
            s if s.eq_ignore_ascii_case("Ne") => Some(Element::Ne),
            s if s.eq_ignore_ascii_case("Na") => Some(Element::Na),
            s if s.eq_ignore_ascii_case("Mg") => Some(Element::Mg),
            s if s.eq_ignore_ascii_case("Al") => Some(Element::Al),
            s if s.eq_ignore_ascii_case("Si") => Some(Element::Si),
            s if s.eq_ignore_ascii_case("P") => Some(Element::P),
            s if s.eq_ignore_ascii_case("S") => Some(Element::S),
            s if s.eq_ignore_ascii_case("Cl") => Some(Element::Cl),
            s if s.eq_ignore_ascii_case("Ar") => Some(Element::Ar),
            s if s.eq_ignore_ascii_case("K") => Some(Element::K),
            s if s.eq_ignore_ascii_case("Ca") => Some(Element::Ca),
            s if s.eq_ignore_ascii_case("Sc") => Some(Element::Sc),
            s if s.eq_ignore_ascii_case("Ti") => Some(Element::Ti),
            s if s.eq_ignore_ascii_case("V") => Some(Element::V),
            s if s.eq_ignore_ascii_case("Cr") => Some(Element::Cr),
            s if s.eq_ignore_ascii_case("Mn") => Some(Element::Mn),
            s if s.eq_ignore_ascii_case("Fe") => Some(Element::Fe),
            s if s.eq_ignore_ascii_case("Co") => Some(Element::Co),
            s if s.eq_ignore_ascii_case("Ni") => Some(Element::Ni),
            s if s.eq_ignore_ascii_case("Cu") => Some(Element::Cu),
            s if s.eq_ignore_ascii_case("Zn") => Some(Element::Zn),
            s if s.eq_ignore_ascii_case("Ga") => Some(Element::Ga),
            s if s.eq_ignore_ascii_case("Ge") => Some(Element::Ge),
            s if s.eq_ignore_ascii_case("As") => Some(Element::As),
            s if s.eq_ignore_ascii_case("Se") => Some(Element::Se),
            s if s.eq_ignore_ascii_case("Br") => Some(Element::Br),
            s if s.eq_ignore_ascii_case("Kr") => Some(Element::Kr),
            s if s.eq_ignore_ascii_case("Rb") => Some(Element::Rb),
            s if s.eq_ignore_ascii_case("Sr") => Some(Element::Sr),
            s if s.eq_ignore_ascii_case("Y") => Some(Element::Y),
            s if s.eq_ignore_ascii_case("Zr") => Some(Element::Zr),
            s if s.eq_ignore_ascii_case("Nb") => Some(Element::Nb),
            s if s.eq_ignore_ascii_case("Mo") => Some(Element::Mo),
            s if s.eq_ignore_ascii_case("Tc") => Some(Element::Tc),
            s if s.eq_ignore_ascii_case("Ru") => Some(Element::Ru),
            s if s.eq_ignore_ascii_case("Rh") => Some(Element::Rh),
            s if s.eq_ignore_ascii_case("Pd") => Some(Element::Pd),
            s if s.eq_ignore_ascii_case("Ag") => Some(Element::Ag),
            s if s.eq_ignore_ascii_case("Cd") => Some(Element::Cd),
            s if s.eq_ignore_ascii_case("In") => Some(Element::In),
            s if s.eq_ignore_ascii_case("Sn") => Some(Element::Sn),
            s if s.eq_ignore_ascii_case("Sb") => Some(Element::Sb),
            s if s.eq_ignore_ascii_case("Te") => Some(Element::Te),
            s if s.eq_ignore_ascii_case("I") => Some(Element::I),
            s if s.eq_ignore_ascii_case("Xe") => Some(Element::Xe),
            s if s.eq_ignore_ascii_case("Cs") => Some(Element::Cs),
            s if s.eq_ignore_ascii_case("Ba") => Some(Element::Ba),
            s if s.eq_ignore_ascii_case("La") => Some(Element::La),
            s if s.eq_ignore_ascii_case("Ce") => Some(Element::Ce),
            s if s.eq_ignore_ascii_case("Pr") => Some(Element::Pr),
            s if s.eq_ignore_ascii_case("Nd") => Some(Element::Nd),
            s if s.eq_ignore_ascii_case("Pm") => Some(Element::Pm),
            s if s.eq_ignore_ascii_case("Sm") => Some(Element::Sm),
            s if s.eq_ignore_ascii_case("Eu") => Some(Element::Eu),
            s if s.eq_ignore_ascii_case("Gd") => Some(Element::Gd),
            s if s.eq_ignore_ascii_case("Tb") => Some(Element::Tb),
            s if s.eq_ignore_ascii_case("Dy") => Some(Element::Dy),
            s if s.eq_ignore_ascii_case("Ho") => Some(Element::Ho),
            s if s.eq_ignore_ascii_case("Er") => Some(Element::Er),
            s if s.eq_ignore_ascii_case("Tm") => Some(Element::Tm),
            s if s.eq_ignore_ascii_case("Yb") => Some(Element::Yb),
            s if s.eq_ignore_ascii_case("Lu") => Some(Element::Lu),
            s if s.eq_ignore_ascii_case("Hf") => Some(Element::Hf),
            s if s.eq_ignore_ascii_case("Ta") => Some(Element::Ta),
            s if s.eq_ignore_ascii_case("W") => Some(Element::W),
            s if s.eq_ignore_ascii_case("Re") => Some(Element::Re),
            s if s.eq_ignore_ascii_case("Os") => Some(Element::Os),
            s if s.eq_ignore_ascii_case("Ir") => Some(Element::Ir),
            s if s.eq_ignore_ascii_case("Pt") => Some(Element::Pt),
            s if s.eq_ignore_ascii_case("Au") => Some(Element::Au),
            s if s.eq_ignore_ascii_case("Hg") => Some(Element::Hg),
            s if s.eq_ignore_ascii_case("Tl") => Some(Element::Tl),
            s if s.eq_ignore_ascii_case("Pb") => Some(Element::Pb),
            s if s.eq_ignore_ascii_case("Bi") => Some(Element::Bi),
            s if s.eq_ignore_ascii_case("Po") => Some(Element::Po),
            s if s.eq_ignore_ascii_case("At") => Some(Element::At),
            s if s.eq_ignore_ascii_case("Rn") => Some(Element::Rn),
            s if s.eq_ignore_ascii_case("Fr") => Some(Element::Fr),
            s if s.eq_ignore_ascii_case("Ra") => Some(Element::Ra),
            s if s.eq_ignore_ascii_case("Ac") => Some(Element::Ac),
            s if s.eq_ignore_ascii_case("Th") => Some(Element::Th),
            s if s.eq_ignore_ascii_case("Pa") => Some(Element::Pa),
            s if s.eq_ignore_ascii_case("U") => Some(Element::U),
            s if s.eq_ignore_ascii_case("Np") => Some(Element::Np),
            s if s.eq_ignore_ascii_case("Pu") => Some(Element::Pu),
            s if s.eq_ignore_ascii_case("Am") => Some(Element::Am),
            s if s.eq_ignore_ascii_case("Cm") => Some(Element::Cm),
            s if s.eq_ignore_ascii_case("Bk") => Some(Element::Bk),
            s if s.eq_ignore_ascii_case("Cf") => Some(Element::Cf),
            s if s.eq_ignore_ascii_case("Es") => Some(Element::Es),
            s if s.eq_ignore_ascii_case("Fm") => Some(Element::Fm),
            s if s.eq_ignore_ascii_case("Md") => Some(Element::Md),
            s if s.eq_ignore_ascii_case("No") => Some(Element::No),
            s if s.eq_ignore_ascii_case("Lr") => Some(Element::Lr),
            s if s.eq_ignore_ascii_case("Rf") => Some(Element::Rf),
            s if s.eq_ignore_ascii_case("Db") => Some(Element::Db),
            s if s.eq_ignore_ascii_case("Sg") => Some(Element::Sg),
            s if s.eq_ignore_ascii_case("Bh") => Some(Element::Bh),
            s if s.eq_ignore_ascii_case("Hs") => Some(Element::Hs),
            s if s.eq_ignore_ascii_case("Mt") => Some(Element::Mt),
            s if s.eq_ignore_ascii_case("Ds") => Some(Element::Ds),
            s if s.eq_ignore_ascii_case("Rg") => Some(Element::Rg),
            s if s.eq_ignore_ascii_case("Cn") => Some(Element::Cn),
            s if s.eq_ignore_ascii_case("Nh") => Some(Element::Nh),
            s if s.eq_ignore_ascii_case("Fl") => Some(Element::Fl),
            s if s.eq_ignore_ascii_case("Mc") => Some(Element::Mc),
            s if s.eq_ignore_ascii_case("Lv") => Some(Element::Lv),
            s if s.eq_ignore_ascii_case("Ts") => Some(Element::Ts),
            s if s.eq_ignore_ascii_case("Og") => Some(Element::Og),
            _ => None,
        }
    }

    pub fn to_symbol(self) -> &'static str {
        ELEMENT_SYMBOLS[self as usize]
    }
}

const ELEMENT_SYMBOLS: [&str; 118] = [
    "H", "HE", "LI", "BE", "B", "C", "N", "O", "F", "NE", "NA", "MG", "AL", "SI", "P", "S", "CL",
    "AR", "K", "CA", "SC", "TI", "V", "CR", "MN", "FE", "CO", "NI", "CU", "ZN", "GA", "GE", "AS",
    "SE", "BR", "KR", "RB", "SR", "Y", "ZR", "NB", "MO", "TC", "RU", "RH", "PD", "AG", "CD", "IN",
    "SN", "SB", "TE", "I", "XE", "CS", "BA", "LA", "CE", "PR", "ND", "PM", "SM", "EU", "GD", "TB",
    "DY", "HO", "ER", "TM", "YB", "LU", "HF", "TA", "W", "RE", "OS", "IR", "PT", "AU", "HG", "TL",
    "PB", "BI", "PO", "AT", "RN", "FR", "RA", "AC", "TH", "PA", "U", "NP", "PU", "AM", "CM", "BK",
    "CF", "ES", "FM", "MD", "NO", "LR", "RF", "DB", "SG", "BH", "HS", "MT", "DS", "RG", "CN", "NH",
    "FL", "MC", "LV", "TS", "OG",
];
