use bevy::prelude::*;
use systems::*;

use crate::{AppState, game::components::SimulationState};

pub mod components;
mod systems;

pub struct AnimationPlugin;

impl Plugin for AnimationPlugin {
    fn build(&self, app: &mut App) {
        app
        // Systems
        .add_systems(
            (
                animate_sprite,
                cycle_images
            )
                .in_set(OnUpdate(AppState::Game))
                .in_set(OnUpdate(SimulationState::Running))
        );
    }
}