use bevy::prelude::*;
use leafwing_input_manager::prelude::InputManagerPlugin;
use systems::*;
use components::*;

use crate::AppState;

use super::SimulationState;

mod systems;
pub mod components;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app
        .add_plugin(InputManagerPlugin::<Action>::default())
        // Enter State Systems
        .add_system(spawn_player.in_schedule(OnEnter(AppState::Game)))
        // Systems
        .add_systems(
            (
                player_movement,
                glide_effect,
                // confine_player_movement,
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
        )
        // Exit State Systems
        .add_system(despawn_players.in_schedule(OnExit(AppState::Game)));
    }
}


pub const PLAYER_SIZE: f32 = 40.0;