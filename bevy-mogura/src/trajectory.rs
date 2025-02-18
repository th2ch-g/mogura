use crate::structure::*;
use crate::*;
use bevy::prelude::*;
use mogura_io::prelude::*;

pub fn update_trajectory(
    mut mogura_state: ResMut<MoguraState>,
    mut current_visualized_atoms: Query<(&mut Transform, &AtomID), Without<BondID>>,
    mut current_visualized_bonds: Query<(&mut Transform, &BondID), Without<AtomID>>,
) {
    if mogura_state.update_trajectory
        || mogura_state.update_tmp_trajectory
        || mogura_state.loop_trajectory
    {
        if mogura_state.trajectory_data.is_some() {
            let current_frame_id = mogura_state.current_frame_id;
            let frame = mogura_state
                .trajectory_data
                .as_ref()
                .unwrap()
                .frame(current_frame_id);

            match mogura_state.drawing_method {
                DrawingMethod::Line
                | DrawingMethod::VDW
                | DrawingMethod::Licorise
                | DrawingMethod::Bonds => {
                    for (mut transform, atom_id) in current_visualized_atoms.iter_mut() {
                        let position = frame.positions()[atom_id.id()];
                        transform.translation = Vec3::new(position[0], position[1], position[2]);
                    }

                    for (mut transform, bond_id) in current_visualized_bonds.iter_mut() {
                        let position1 = frame.positions()[bond_id.atomid1()];
                        let position2 = frame.positions()[bond_id.atomid2()];
                        let start = Vec3::new(position1[0], position1[1], position1[2]);
                        let end = Vec3::new(position2[0], position2[1], position2[2]);
                        let center = (start + end) / 2.;
                        let direction = end - start;
                        let length = direction.length();
                        let rotation = Quat::from_rotation_arc(Vec3::Y, direction.normalize());
                        transform.translation = center;
                        transform.rotation = rotation;
                    }
                }
                _ => (),
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
}
