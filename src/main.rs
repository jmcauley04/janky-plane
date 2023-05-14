use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use components::*;
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
        .add_plugins(DefaultPlugins
            .set(ImagePlugin::default_nearest())
            .set(WindowPlugin {
                primary_window: Some(Window {
                    title: "Janky Plane".into(),
                    ..default()
                }),
                ..default()
        }))
        .add_plugin(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(200.0))
        .insert_resource(RapierConfiguration {
            gravity: Vec2::new(0.0, -20.0),
            ..default()
        })
        //.add_plugin(RapierDebugRenderPlugin::default())
        .add_state::<AppState>()
        .add_startup_system(spawn_camera)
        .add_plugin(MainMenuPlugin)
        .add_plugin(GamePlugin)
        .add_system(transition_to_game_state)
        .add_system(transition_to_main_menu_state)
        .run();
}
