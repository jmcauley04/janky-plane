use bevy::prelude::*;
use systems::*;
use super::components::*;
use crate::AppState;

pub mod components;
mod systems;

pub const SHOT_FORCE: f32 = 1000.0;

pub struct ProjectilesPlugin;

impl Plugin for ProjectilesPlugin {
    fn build(&self, app: &mut App) {
        app
        // Enter State Systems
        // Systems
        .add_systems(
            (
                player_fire,
                projectile_despawner,
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
        )
        .add_system(projectile_hit);
        // Exit State Systems
        
    }
}
