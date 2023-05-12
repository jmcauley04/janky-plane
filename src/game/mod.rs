use bevy::prelude::*;
use crate::game::player::PlayerPlugin;
use crate::animation::AnimationPlugin;
use crate::AppState;
use systems::*;

use self::systems::toggle_simulation;

mod player;
mod systems;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {        
        app.add_state::<SimulationState>()
            .add_plugin(PlayerPlugin)
            .add_plugin(AnimationPlugin)
            .add_startup_system(spawn_bg)
            .add_startup_system(spawn_floor)
            //.add_startup_system(spawn_obstacle)
            .add_system(toggle_simulation.run_if(in_state(AppState::Game)));
    }
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum SimulationState {
    Running,
    #[default]
    Paused,
}