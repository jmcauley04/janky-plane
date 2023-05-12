use bevy::prelude::*;
use game::GamePlugin;
use main_menu::MainMenuPlugin;
use systems::*;

pub mod components;
pub mod systems;
pub mod game;
pub mod main_menu;
pub mod animation;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(ImagePlugin::default_nearest()))
        .add_state::<AppState>()
        .add_startup_system(spawn_camera)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}

#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum AppState{
    #[default]
    MainMenu,
    Game,
    GameOver,
}