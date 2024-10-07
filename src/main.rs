use bevy::prelude::*;
use bevy_mogura::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(MoguraPlugins)
        .run();
}
