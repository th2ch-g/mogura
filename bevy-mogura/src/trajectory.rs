use crate::structure::*;
use crate::*;

#[derive(Clone)]
pub struct MoguraTrajectoryPlugins;

impl Plugin for MoguraTrajectoryPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_trajectory);
    }
}

#[allow(clippy::type_complexity)]
fn update_trajectory(
    mut mogura_state: ResMut<MoguraState>,
    mogura_selections: Res<MoguraSelections>,
    mut current_visualized_atoms: Query<
        (&mut Transform, &AtomID),
        (Without<InterpolationID>, Without<BondID>),
    >,
    mut current_visualized_bonds: Query<
        (&mut Transform, &BondID),
        (Without<InterpolationID>, Without<AtomID>),
    >,
    mut current_visualized_tubes: Query<
        (&mut Transform, &InterpolationID),
        (Without<AtomID>, Without<BondID>),
    >,
    parent_query: Query<(&StructureParams, &Children)>,
) {
    if (mogura_state.update_trajectory
        || mogura_state.update_tmp_trajectory
        || mogura_state.loop_trajectory)
        && mogura_state.structure_data.is_some()
        && mogura_state.trajectory_data.is_some()
    {
        if mogura_selections.0.is_empty() {
            return;
        }
        let current_frame_id = mogura_state.current_frame_id;
        let frame = mogura_state
            .trajectory_data
            .as_ref()
            .unwrap()
            .frame(current_frame_id);

        for (structure_params, childlen) in parent_query.iter() {
            let drawing_method = mogura_selections.0[structure_params.id].drawing_method;

            match drawing_method {
                DrawingMethod::Line
                | DrawingMethod::Ball
                | DrawingMethod::BallAndStick
                | DrawingMethod::Stick => {
                    for child in childlen.iter() {
                        if let Ok((mut transform, atom_id)) =
                            current_visualized_atoms.get_mut(*child)
                        {
                            let position = frame.positions()[atom_id.id()];
                            transform.translation =
                                Vec3::new(position[0], position[1], position[2]);
                        }

                        if let Ok((mut transform, bond_id)) =
                            current_visualized_bonds.get_mut(*child)
                        {
                            let position1 = frame.positions()[bond_id.atomid1()];
                            let position2 = frame.positions()[bond_id.atomid2()];
                            let start = Vec3::new(position1[0], position1[1], position1[2]);
                            let end = Vec3::new(position2[0], position2[1], position2[2]);
                            let pos_1_4 = start + (end - start) * 0.25;
                            // let pos_3_4 = start + (end - start) * 0.75;
                            let direction = end - start;
                            let length = direction.length();
                            if length > GENERAL_BOND_CUTOFF {
                                continue;
                            }
                            let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                            transform.translation = pos_1_4;
                            transform.rotation = rotation;
                            transform.scale = Vec3::ONE * length / 2.;
                        }
                    }
                }
                DrawingMethod::Tube => {
                    let atoms = mogura_state.structure_data.as_ref().unwrap().atoms();
                    let mut target_atoms = Vec::with_capacity(atoms.len());

                    for atom in atoms {
                        if !atom.is_backbone()
                            || (atom.is_backbone() && atom.atom_name() == "HA")
                            || (atom.is_backbone() && atom.atom_name() == "O")
                        {
                            continue;
                        }
                        target_atoms.push(atom.id());
                    }

                    let mut points = Vec::with_capacity(target_atoms.len() * INTERPOLATION_STEPS);
                    let mut interpolation_id = 0;
                    for i in 1..target_atoms.len() - 2 {
                        for j in 0..=INTERPOLATION_STEPS {
                            let t = j as f32 / INTERPOLATION_STEPS as f32;
                            let point = catmull_rom_interpolate(
                                frame.positions()[target_atoms[i - 1]].into(),
                                frame.positions()[target_atoms[i]].into(),
                                frame.positions()[target_atoms[i + 1]].into(),
                                frame.positions()[target_atoms[i + 2]].into(),
                                t,
                            );
                            points.push((point, interpolation_id));
                            interpolation_id += 1;
                        }
                    }

                    for child in childlen.iter() {
                        if let Ok((mut transform, interpolation_id)) =
                            current_visualized_tubes.get_mut(*child)
                        {
                            let start_id = interpolation_id.start_id();
                            let end_id = interpolation_id.end_id();
                            let start = points[start_id].0;
                            let end = points[end_id].0;
                            let direction = end - start;
                            let length = direction.length();
                            let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                            if length > GENERAL_BOND_CUTOFF / INTERPOLATION_STEPS as f32 * 2. {
                                continue;
                            }
                            transform.translation = start;
                            transform.rotation = rotation;
                            transform.scale = Vec3::ONE * length;
                        }
                    }
                }
            }
        }

        if mogura_state.update_trajectory {
            mogura_state.next_frame_id();
        }

        if mogura_state.loop_trajectory {
            mogura_state.loop_frame_id();
        }

        mogura_state.update_tmp_trajectory = false;
    }
}
