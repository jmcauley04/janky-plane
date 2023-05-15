use bevy::prelude::*;
use systems::*;

mod systems;
pub mod components;

pub const MOVE_FORCE: f32 = 1500.0;
pub struct EnemyPlugin;

impl Plugin for EnemyPlugin {
    fn build(&self, app: &mut App) {
        app        
        .add_startup_system(spawn_enemies)
        .add_system(fly_enemies)
        .add_system(fire_enemies);
    }
}