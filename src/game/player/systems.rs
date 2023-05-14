use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use leafwing_input_manager::prelude::*;
use bevy::window::PrimaryWindow;
use crate::animation::components::*;
use crate::game::projectiles::components::*;
use super::PLAYER_SIZE;
use super::components::*;

pub const MOVE_FORCE: f32 = 1500.0;
pub const PLAYER_COUNT : usize = 1;

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
                    transform: Transform::from_xyz(window.width() / 3.0 - x_delta, window.height() / 2.0, 500.0),
                    ..default()
                },
                animation_indices,
                AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
                FireSpeedTimer(Timer::from_seconds(0.15, TimerMode::Repeating)),
            ))
            .insert(InputManagerBundle::<Action> {
                action_state: ActionState::default(),
                input_map: InputMap::default() 
                    .insert(DualAxis::left_stick(), Action::Move)
                    .insert(VirtualDPad::wasd(), Action::Move)
                    .insert(VirtualDPad::arrow_keys(), Action::Move)
                    .build(),
            })
            .insert(Name::new(player_name))
            .insert(RigidBody::Dynamic) // 
            .insert(Ccd::enabled())
            .insert(Velocity::default())
            .insert(Collider::capsule_x(14.0, 20.0)) // shape of collider
            .insert(ExternalForce { // used for physics and to control movement
                force: Vec2::new(0.0, 0.0),
                torque: 0.0,
            })
            .insert(Damping { // friction
                linear_damping: 0.1,
                angular_damping: 5.0,
            })
            .insert(Restitution::coefficient(1.0))
            .insert(Player)
            .insert(Glider)            
            .insert(CollisionGroups::new(Group::GROUP_1,Group::GROUP_2));
    }
}

pub fn despawn_players(
    mut commands: Commands,
    player_query: Query<Entity, With<Player>>,
){
    for plane_entity in player_query.iter(){
        commands.entity(plane_entity).despawn();
    }
}

pub fn player_movement(
    mut query: Query<(&ActionState<Action>, &mut ExternalForce), With<Player>>,
    time: Res<Time>,
){
    for (action_state, mut external_force) in query.iter_mut(){
        if let Some(axis_vector) = action_state.clamped_axis_pair(Action::Move){
            external_force.force = MOVE_FORCE  * time.delta_seconds() * (axis_vector.xy() + 0.2 * Vec2::new(0.0, axis_vector.xy().x));
        }        
    }
}


// pub fn confine_player_movement(
//     window_query: Query<&Window, With<PrimaryWindow>>,
//     mut player_query: Query<(&mut Player, &Transform), With<Player>>
// )
// {
//     let window = window_query.get_single().unwrap();
//     let height_cap = 0.8;

//     for (mut plane, transform) in player_query.iter_mut(){
//         if transform.translation.y > window.height() * height_cap && plane.speed.y > 0.0 {
//             let adjusted_max = FLAP_MAX * (window.height() - transform.translation.y) / (window.height() * (1.0 - height_cap));
//             plane.speed.y = match plane.speed.y > adjusted_max{
//                 true => adjusted_max,
//                 false => plane.speed.y
//             };
//         } else if transform.translation.y < 0.0 && plane.speed.y < 0.0 {
//             plane.speed.y = 0.0;
//         }  
//     }
// }

pub fn glide_effect(
    mut player_query: Query<(&mut Transform, &Velocity), With<Glider>>,
){
    for (mut transform, velocity) in player_query.iter_mut(){
        if velocity.linvel.y > 0.0{
            transform.rotation = Quat::from_rotation_z(Vec3::new(100.0, velocity.linvel.y, 0.0).angle_between(Vec3::new(100.0, 0.0, 0.0)));
        } else {            
            transform.rotation = Quat::from_rotation_z(-Vec3::new(100.0, velocity.linvel.y, 0.0).angle_between(Vec3::new(100.0, 0.0, 0.0)));
        }        
    }
}
