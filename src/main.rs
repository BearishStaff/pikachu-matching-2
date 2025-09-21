use bevy::{prelude::*};
mod systems;
mod components;

use systems::*;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .insert_resource(SelectedTiles::default())
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems( Update ,select_tile)
        .add_systems( Update,process_selection)
        .run();
}