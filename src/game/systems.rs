use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::animation::components::CyclicImage;
use crate::game::SimulationState;


pub fn toggle_simulation(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
){
    if keyboard_input.just_pressed(KeyCode::P){
        if simulation_state.0 == SimulationState::Running {
            commands.insert_resource(NextState(Some(SimulationState::Paused)));
            println!("Simulation paused.");
        } else if simulation_state.0 == SimulationState::Paused {
            commands.insert_resource(NextState(Some(SimulationState::Running)));
            println!("Simulation resumed.");
        }
    }
}

pub fn spawn_bg(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();
    let width = 800.0;
    let height = window.height();
    let multiples = 4;
    for id in 0..multiples {
        let x = id as f32 * width;
        let y = height / 2.0;
        let texture_handle = asset_server.load("spritesheets/sheet.png");    

        //let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(0.0, 0.0));
        let texture_atlas = TextureAtlas::from_grid(
            texture_handle, 
            Vec2::new(width, 835.0 - 355.0), 
            1, 
            1, 
            None, 
            Some(Vec2::new(0.0, 355.0))
        );
        //0,355 - 800,835
        //texture_atlas.add_texture(Rect::new(0.0, 355.0, width, 835.0));        
        
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        let transform = Transform::from_xyz(x,y, 0.0);

        commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite {
                        index: 0,
                        custom_size: Some(Vec2::new(width, height)),
                        ..default()
                    },
                    transform: transform,
                    ..default()
                },                
                CyclicImage{
                    speed: Vec3::new(-1.0, 0.0, 0.0),
                    zone: Rect::new(x, y, x + width, y + height)
                }
            ));
    }
}

pub fn spawn_floor(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
)
{
    let width = 808.0;
    let height = 71.0;
    let multiples = 4;
    for id in 0..multiples {
        let x = id as f32 * width;
        let y = 35.0;
        let texture_handle = asset_server.load("spritesheets/sheet.png");    

        let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(0.0, 0.0));
        //0,355 - 800,835
        texture_atlas.add_texture(Rect::new(0.0, 142.5, width, 213.0));        
        
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let transform = Transform::from_xyz(id as f32 * width,y, 10.0);

        commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform: transform,
                    ..default()
                },                
                CyclicImage{
                    speed: Vec3::new(-4.0, 0.0, 0.0),
                    zone: Rect::new(x, y, x + width, y + height)
                }
            ));
    }
}

pub fn spawn_obstacle(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    let width = 108.0;
    let multiples = 3;
    for i in 0..multiples {
        let texture_handle = asset_server.load("spritesheets/sheet.png");    

        let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(0.0, 0.0));
        
        texture_atlas.add_texture(Rect::new(0.0, 1757.5, width, 1996.0));        
        
        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        let transform = Transform::from_xyz(1500.0 + (300.0 * i as f32),119.5, 0.0)
            .with_scale(Vec3::new(1.0,2.0, 0.0));

        commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,
                    sprite: TextureAtlasSprite::new(0),
                    transform: transform,
                    ..default()
                },                
                CyclicImage{
                    speed: Vec3::new(-2.0, 0.0, 0.0),
                    zone: Rect::new(0.0, 0.0, width, 1996.0)
                }
            ));
    }
}
