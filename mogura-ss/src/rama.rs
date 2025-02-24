use crate::*;
use std::f32::consts::PI;

fn find_atom(res: &Residue, name: &str) -> Option<Atom> {
    res.atoms.iter().find(|atom| atom.name == name).cloned()
}

fn vec_sub(a: &Atom, b: &Atom) -> [f32; 3] {
    [a.x - b.x, a.y - b.y, a.z - b.z]
}

fn cross(u: &[f32; 3], v: &[f32; 3]) -> [f32; 3] {
    [
        u[1] * v[2] - u[2] * v[1],
        u[2] * v[0] - u[0] * v[2],
        u[0] * v[1] - u[1] * v[0],
    ]
}

fn dot(u: &[f32; 3], v: &[f32; 3]) -> f32 {
    u[0] * v[0] + u[1] * v[1] + u[2] * v[2]
}

fn norm(v: &[f32; 3]) -> f32 {
    (v[0] * v[0] + v[1] * v[1] + v[2] * v[2]).sqrt()
}

fn normalize(v: &[f32; 3]) -> [f32; 3] {
    let n = norm(v);
    if n == 0.0 {
        [0.0, 0.0, 0.0]
    } else {
        [v[0] / n, v[1] / n, v[2] / n]
    }
}

fn dihedral(p1: &Atom, p2: &Atom, p3: &Atom, p4: &Atom) -> f32 {
    let b1 = [p2.x - p1.x, p2.y - p1.y, p2.z - p1.z];
    let b2 = [p3.x - p2.x, p3.y - p2.y, p3.z - p2.z];
    let b3 = [p4.x - p3.x, p4.y - p3.y, p4.z - p3.z];

    let n1 = cross(&b1, &b2);
    let n2 = cross(&b2, &b3);

    let b2_norm = normalize(&b2);
    let m1 = normalize(&n1);
    let m2 = normalize(&n2);

    let x = dot(&m1, &m2);
    let y = dot(&cross(&m1, &m2), &b2_norm);
    let angle = y.atan2(x);

    angle.to_degrees()
}

pub fn assign_ss(residues_in_protein: &Vec<Residue>) -> Vec<SS> {
    let n_res = residues_in_protein.len();
    let mut ss = vec![SS::Loop; n_res];

    for i in 0..n_res {
        let current = &residues_in_protein[i];
        let n_atom = match find_atom(current, "N") {
            Some(a) => a,
            None => continue,
        };
        let ca_atom = match find_atom(current, "CA") {
            Some(a) => a,
            None => continue,
        };
        let c_atom = match find_atom(current, "C") {
            Some(a) => a,
            None => continue,
        };

        // φ = dihedral(C(i-1), N(i), CA(i), C(i))
        let phi = if i > 0 {
            let prev = &residues_in_protein[i - 1];
            if let Some(prev_c) = find_atom(prev, "C") {
                Some(dihedral(&prev_c, &n_atom, &ca_atom, &c_atom))
            } else {
                None
            }
        } else {
            None
        };

        // ψ = dihedral(N(i), CA(i), C(i), N(i+1))
        let psi = if i < n_res - 1 {
            let next = &residues_in_protein[i + 1];
            if let Some(next_n) = find_atom(next, "N") {
                Some(dihedral(&n_atom, &ca_atom, &c_atom, &next_n))
            } else {
                None
            }
        } else {
            None
        };

        if let (Some(phi_val), Some(psi_val)) = (phi, psi) {
            // α-helix, φ ∈ [–90, –30], ψ ∈ [–77, –17]）
            if phi_val >= -90.0 && phi_val <= -30.0 && psi_val >= -77.0 && psi_val <= -17.0 {
                ss[i] = SS::H;
            }
            // β-strand, φ ∈ [–150, –90], ψ ∈ [90, 180]）
            else if phi_val >= -150.0 && phi_val <= -90.0 && psi_val >= 90.0 && psi_val <= 180.0 {
                ss[i] = SS::E;
            }
        }
    }

    ss
}
