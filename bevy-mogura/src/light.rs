use bevy::prelude::*;

fn setup_light(mut commands: Commands) {
    let base = 1.0;
    let positions = [
        Vec3::new(base, base, base),
        Vec3::new(-base, base, base),
        Vec3::new(-base, -base, base),
        Vec3::new(-base, -base, -base),
        Vec3::new(base, -base, base),
        Vec3::new(base, base, -base),
        Vec3::new(base, -base, -base),
        Vec3::new(-base, base, -base),
    ];
    for position in positions {
        commands.spawn((
            DirectionalLight::default(),
            Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
        ));
    }
}

#[derive(Clone)]
pub struct MoguraLightPlugins;

impl Plugin for MoguraLightPlugins {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_light);
    }
}
