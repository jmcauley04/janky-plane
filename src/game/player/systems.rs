use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use crate::animation::components::*;
use crate::components::Projectile;
use super::PLAYER_SIZE;

pub const FLAP_SPEED : f32 = 4.0;
pub const FLAP_MAX : f32 = 200.0;
pub const GRAVITY_SPEED : f32 = 1.0;
pub const GRAVITY_MAX : f32 = -10.0;
pub const PLAYER_COUNT : usize = 1;

#[derive(Component)]
pub struct Plane{
    pub speed: Vec2,
}

pub fn spawn_player(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    let window = window_query.get_single().unwrap();
    
    for player_number in 0..PLAYER_COUNT {
        let player_name = format!("Player {}", player_number + 1);
        let x_delta = PLAYER_SIZE * player_number as f32;
        let texture_handle = asset_server.load("spritesheets/planes.png");    
        let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(PLAYER_SIZE, PLAYER_SIZE));
        //88, 73
        if player_number == 0{
            for i in 0..4 {
                let rect_start = match i {
                    0 => (0.0, 0.0), //2
                    1 => (0.0, 73.0), //1
                    2 => (0.0, 0.0), //2
                    3 => (0.0, 365.0), //3
                    _ => (i as f32,i as f32)
                };
                texture_atlas.add_texture(Rect::new(rect_start.0, rect_start.1, rect_start.0 + 88.0, rect_start.1 + 72.5));
            } 
        } else if  player_number == 1 {
            for i in 0..4 {
                let rect_start = match i {
                    0 => (0.0, 438.0), //2
                    1 => (88.0, 0.0), //1
                    2 => (0.0, 438.0), //2
                    3 => (88.0, 292.0), //3
                    _ => (i as f32,i as f32)
                };
                texture_atlas.add_texture(Rect::new(rect_start.0, rect_start.1, rect_start.0 + 88.0, rect_start.1 + 72.5));
            } 
        } else if  player_number == 2 {
            for i in 0..4 {
                let rect_start = match i {
                    0 => (88.0, 146.0), //2
                    1 => (88.0, 219.0), //1
                    2 => (88.0, 146.0), //2
                    3 => (88.0, 73.0), //3
                    _ => (i as f32,i as f32)
                };
                texture_atlas.add_texture(Rect::new(rect_start.0, rect_start.1, rect_start.0 + 88.0, rect_start.1 + 72.5));
            } 
        } else if  player_number == 3 {
            for i in 0..4 {
                let rect_start = match i {
                    0 => (0.0, 219.0), //2
                    1 => (0.0, 292.0), //1
                    2 => (0.0, 219.0), //2
                    3 => (0.0, 146.0), //3
                    _ => (i as f32,i as f32)
                };
                texture_atlas.add_texture(Rect::new(rect_start.0, rect_start.1, rect_start.0 + 88.0, rect_start.1 + 72.5));
            } 
        }

        let texture_atlas_handle = texture_atlases.add(texture_atlas);
        
        // Use only the subset of sprites in the sheet that make up the run animation
        let animation_indices = AnimationIndices { first: 0, last: 3 };
        commands.spawn((
                SpriteSheetBundle {
                    texture_atlas: texture_atlas_handle,                    
                    sprite: TextureAtlasSprite {
                        index: animation_indices.first,
                        custom_size: Some(Vec2::new(88.0 * 0.7,73.0 * 0.7)),
                        ..default()
                    },
                    transform: Transform::from_xyz(window.width() / 3.0 - x_delta, window.height() / 2.0, 0.0),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                Plane{
                    speed: Vec2::new(0.0, 0.0),
                }
            ))
            .insert(Name::new(player_name));
    }
}

pub fn despawn_players(
    mut commands: Commands,
    player_query: Query<Entity, With<Plane>>,
){
    for plane_entity in player_query.iter(){
        commands.entity(plane_entity).despawn();
    }
}

pub fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut Plane, &Name), With<Plane>>
){
    for (mut plane, name) in player_query.iter_mut(){

        if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::W) {
            plane.speed.y += FLAP_SPEED;
            
            if plane.speed.y > FLAP_MAX {
                plane.speed.y = FLAP_MAX;
            }
        } else if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::S) {
            plane.speed.y -= FLAP_SPEED;
            
            if plane.speed.y > FLAP_MAX {
                plane.speed.y = FLAP_MAX;
            }
        }

        if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::A) {
            plane.speed.x -= FLAP_SPEED;

            if plane.speed.x < -FLAP_MAX {
                plane.speed.x = -FLAP_MAX;
            }            
        } else if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::D) {
            plane.speed.x += FLAP_SPEED;

            if plane.speed.x > FLAP_MAX {
                plane.speed.x = FLAP_MAX;
            }            
        }
    }
}

pub fn player_fire(
    mut commands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    player_query: Query<(&Plane, &Transform, &Name), With<Plane>>,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
){
    for (plane, transform, name) in player_query.iter(){
        if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::Space){         
            println!("FIRE!!!");
            let texture_handle = asset_server.load("spritesheets/planes.png");    
            let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(PLAYER_SIZE, PLAYER_SIZE));
            
            texture_atlas.add_texture(Rect::new(0.0, 0.0, 88.0, 72.5));

            let texture_atlas_handle = texture_atlases.add(texture_atlas);
            
            // Use only the subset of sprites in the sheet that make up the run animation
            let animation_indices = AnimationIndices { first: 0, last: 3 };
            commands.spawn((
                    SpriteSheetBundle {
                        texture_atlas: texture_atlas_handle,                    
                        sprite: TextureAtlasSprite {
                            index: animation_indices.first,
                            custom_size: Some(Vec2::new(88.0 * 0.7,73.0 * 0.7)),
                            ..default()
                        },
                        transform: transform.clone(),
                        ..default()
                    },
                    Projectile{
                        speed: 1000,
                        mass: 3,
                    }
                ));
        }
    }
    
}

pub fn confine_player_movement(
    window_query: Query<&Window, With<PrimaryWindow>>,
    mut player_query: Query<(&mut Plane, &Transform), With<Plane>>
)
{
    let window = window_query.get_single().unwrap();
    let height_cap = 0.8;

    for (mut plane, transform) in player_query.iter_mut(){
        if transform.translation.y > window.height() * height_cap && plane.speed.y > 0.0 {
            let adjusted_max = FLAP_MAX * (window.height() - transform.translation.y) / (window.height() * (1.0 - height_cap));
            plane.speed.y = match plane.speed.y > adjusted_max{
                true => adjusted_max,
                false => plane.speed.y
            };
        } else if transform.translation.y < 0.0 && plane.speed.y < 0.0 {
            plane.speed.y = 0.0;
        }  
    }
}

pub fn gravity_effect(
    mut player_query: Query<(&mut Transform, &mut Plane), With<Plane>>,
    time: Res<Time>
){
    for (mut transform, mut plane) in player_query.iter_mut(){
        
        let direction = Vec3::new(plane.speed.x, plane.speed.y, 0.0);
        transform.translation += direction * time.delta_seconds();

        if plane.speed.y > 0.0{
            transform.rotation = Quat::from_rotation_z(Vec3::new(100.0, plane.speed.y, 0.0).angle_between(Vec3::new(100.0, 0.0, 0.0)));
        } else {            
            transform.rotation = Quat::from_rotation_z(-Vec3::new(100.0, plane.speed.y, 0.0).angle_between(Vec3::new(100.0, 0.0, 0.0)));
        }

        let target_speed = plane.speed.y - GRAVITY_SPEED;
        if target_speed < GRAVITY_MAX {
            plane.speed.y = target_speed;
        } else{
            plane.speed.y = target_speed;
        }
            
    }
}

pub fn projectile_motion(
    mut projectile_query: Query<(&mut Transform, &Projectile), With<Projectile>>,
    time: Res<Time>
)
{
    for (mut transform, projectile) in projectile_query.iter_mut(){
        let move_dir = transform.rotation * Vec3::new(projectile.speed as f32, 0.0, 0.0) * time.delta_seconds();
        transform.translation += move_dir;
    }
}