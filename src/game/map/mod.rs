use bevy::prelude::*;
use systems::*;

mod systems;
mod environment_maps;
pub mod components;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app        
        .add_startup_system(spawn_bg)
        .add_startup_system(spawn_tiles);
    }
}