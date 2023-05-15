use bevy::prelude::*;
use crate::game::projectiles::ProjectilesPlugin;
use crate::game::player::PlayerPlugin;
use crate::game::enemies::EnemyPlugin;
use crate::animation::AnimationPlugin;
use crate::AppState;
use components::*;
use crate::game::map::MapPlugin;

use self::systems::toggle_simulation;

mod projectiles;
mod map;
mod enemies;
mod player;
mod systems;
pub mod components;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {        
        app.add_state::<SimulationState>()
            .add_plugin(PlayerPlugin)
            .add_plugin(ProjectilesPlugin)
            .add_plugin(AnimationPlugin)
            .add_plugin(MapPlugin)
            .add_plugin(EnemyPlugin)
            .add_systems(
                (
                    toggle_simulation,
                )
                .in_set(OnUpdate(AppState::Game)));
    }
}
