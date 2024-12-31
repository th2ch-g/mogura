use bevy::prelude::*;
use bevy_mogura_trackball::prelude::*;
use structure::LineMaterial;

mod camera;
mod light;
mod structure;

pub mod prelude {
    pub use crate::MoguraPlugins;
}

#[derive(Clone, Resource)]
pub struct MoguraPlugins {
    pub input_structure: Option<String>,
}

impl Default for MoguraPlugins {
    fn default() -> Self {
        Self {
            input_structure: None,
        }
    }
}

impl Plugin for MoguraPlugins {
    fn build(&self, app: &mut App) {
        app
            .insert_resource(self.clone())
            .add_plugins(MaterialPlugin::<LineMaterial>::default())
            .add_plugins(TrackballPlugin)
            .add_systems(Startup, light::setup_light)
            // .add_systems(
            //     Startup,
            //     (
            //         (camera::setup_camera, structure::setup_structure)
            //             .before(camera::set_look_at_center),
            //         camera::set_look_at_center,
            //     ),
            // )
            .add_systems(Startup, structure::setup_structure)
            .add_systems(Startup, camera::setup_camera);
        // .add_systems(Update, camera::update_camera_pos)
        // .add_systems(Update, camera::update_camera_scroll)
        // .add_systems(Update, camera::update_camera_mode);
    }
}
