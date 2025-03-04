use bevy::prelude::*;
use bevy_trackball::prelude::*;

#[derive(Clone)]
pub struct MoguraCameraPlugins;

impl Plugin for MoguraCameraPlugins {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::srgb(1., 1., 1.)))
            .add_plugins(TrackballPlugin)
            .add_systems(Startup, setup_camera);
    }
}

fn setup_camera(mut commands: Commands) {
    let [target, eye, up] = [Vec3::ZERO, Vec3::Z * 30., Vec3::Y];
    commands.spawn((
        TrackballController::default(),
        TrackballCamera::look_at(target, eye, up),
        Camera3d::default(),
    ));
}
