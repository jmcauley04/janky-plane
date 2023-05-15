use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use crate::game::projectiles::systems::spawn_bullet;

use crate::{
    animation::components::*, 
    game::{
        components::*,
        projectiles::components::*, 
        player::components::*
    }};

use super::{components::*, MOVE_FORCE};

const PREFERRED_SPOT: Vec2 = Vec2::new(1000.0, 600.0);

pub fn spawn_enemies(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut texture_atlases: ResMut<Assets<TextureAtlas>>,
) {
    spawn_enemy(&mut commands, &asset_server, &mut texture_atlases,
        Transform::from_xyz(1000.0, 700.0, 500.0));
    spawn_enemy(&mut commands, &asset_server, &mut texture_atlases,
        Transform::from_xyz(1000.0, 500.0, 500.0));
    spawn_enemy(&mut commands, &asset_server, &mut texture_atlases,
        Transform::from_xyz(1000.0, 300.0, 500.0));
}

fn spawn_enemy(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    texture_atlases: &mut ResMut<Assets<TextureAtlas>>,
    transform: Transform,
){    
    let plane_size = 40.0;
    let texture_handle = asset_server.load("spritesheets/planes.png");    
    let mut texture_atlas = TextureAtlas::new_empty(texture_handle, Vec2::new(plane_size, plane_size));
    //88, 73
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
                transform: transform,
                ..default()
            },
            animation_indices,
            AnimationTimer(Timer::from_seconds(0.05, TimerMode::Repeating)),
            FireSpeedTimer(Timer::from_seconds(0.25, TimerMode::Repeating)),
        ))           
        .insert(Enemy)
        .insert(Health(50))
        .insert(Name::new("ChoopaMe"))
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
        .insert(Glider)            
        .insert(CollisionGroups::new(Group::GROUP_2,Group::GROUP_1));    
}

pub fn fly_enemies(
    mut query: Query<(&Transform, &mut ExternalForce), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
){
for player_transform in player_query.iter(){
    for (transform, mut external_force) in query.iter_mut(){
        
        let target_spot = player_transform.translation.truncate() + Vec2::new(-300.0, 10.0);
        external_force.force = (MOVE_FORCE  * time.delta_seconds() * (target_spot - transform.translation.truncate())
            .clamp(Vec2::new(-1.0, -1.0), Vec2::new(1.0, 1.0)))
            .clamp(Vec2::new(-10.0, -10.0), Vec2::new(10.0, 10.0));    
    }
}
}

pub fn fire_enemies(
    mut enemy_query: Query<(&Transform, &mut FireSpeedTimer, &CollisionGroups), With<Enemy>>,
    player_query: Query<&Transform, With<Player>>,
    mut commands: Commands,
    time: Res<Time>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
){
    for (transform, mut timer, collision_groups) in enemy_query.iter_mut(){
        timer.tick(time.delta());
        let top_left = transform.up() * 100.0;
        let bottom_right = transform.down() * 100.0 + transform.right() * 1200.0;
        for player_transform in player_query.iter(){       
            let fire_zone = Rect::new(
                transform.translation.x + top_left.x, 
                transform.translation.y + top_left.y, 
                transform.translation.x + bottom_right.x, 
                transform.translation.y + bottom_right.y
            );
            
            if fire_zone.contains(player_transform.translation.truncate()) && timer.just_finished(){         
                spawn_bullet(&mut commands, &asset_server, &audio, &transform, collision_groups);
            }
        }
    }    
}
