use crate::structure::*;
use crate::*;
use bevy::prelude::*;
use mogura_io::prelude::*;

pub fn update_trajectory(
    mut mogura_state: ResMut<MoguraState>,
    // mut current_visualized_atoms: Query<(&mut Transform, &AtomID), With<structure::StructureParams>>,
    // mut current_visualized_bonds: Query<(&mut Transform, &BondID), With<structure::StructureParams>>,
    mut current_visualized_atoms: Query<(&mut Transform, &AtomID)>,
    // mut current_visualized_bonds: Query<(&mut Transform, &BondID)>,
) {
    if mogura_state.dotrajectory {
        if mogura_state.trajectory_data.is_some() {
            let current_frame_id = mogura_state.current_frame_id;
            let frame = mogura_state
                .trajectory_data
                .as_ref()
                .unwrap()
                .frame(current_frame_id);

            for (mut transform, atom_id) in current_visualized_atoms.iter_mut() {
                let position = frame.positions()[atom_id.id()];
                transform.translation = Vec3::new(position[0], position[1], position[2]);
            }

            let n_frame = mogura_state.trajectory_data.as_ref().unwrap().n_frame();
            mogura_state.next_frame_id(n_frame);
        }
    }
}
