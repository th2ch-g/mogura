use bevy::prelude::*;

pub fn setup_light(mut commands: Commands) {
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
        commands.spawn(DirectionalLightBundle {
            transform: Transform::from_translation(position).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        });
    }
}
