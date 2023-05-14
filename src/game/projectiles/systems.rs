use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::player::components::Player;

use super::{components::*, SHOT_FORCE};

pub fn projectile_despawner(
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,
    projectile_query: Query<(Entity, &Transform), With<Projectile>>,
)
{
    let window = window_query.get_single().unwrap();

    for (entity, transform) in projectile_query.iter(){
        let v2 = transform.translation.truncate();
        let screen = Rect::new(0.0, 0.0, window.width(), window.height());
        if screen.min.y > v2.y || screen.min.x - 50.0 > v2.x || screen.max.x + 50.0 < v2.x{
            commands.entity(entity).despawn();
        }
    }
}

pub fn projectile_hit(
    mut commands: Commands, 
    projectile_colliders: Query<Entity, With<Projectile>>,
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
){
    for projectile_entity in projectile_colliders.iter(){
        for (_, _, intersecting) in rapier_context.intersections_with(projectile_entity){
            if intersecting {
                commands.entity(projectile_entity).despawn();
                let rand: i32 = rand::thread_rng().gen_range(1..5);
                let sound_effect = asset_server.load(format!("sounds/impactSoft_medium_00{rand}.ogg"));
                audio.play(sound_effect);
            }
        }
    }
}


pub fn player_fire(
    mut commands: Commands,
    time: Res<Time>,
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&Transform, &Name, &mut FireSpeedTimer), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
){
    for (transform, name, mut timer) in player_query.iter_mut(){
        timer.tick(time.delta());
        if timer.just_finished(){
            if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::Space){
                let force = transform.rotation * Vec3::new(SHOT_FORCE, 0.0, 0.0);
                // Use only the subset of sprites in the sheet that make up the run animation
                commands.spawn((
                        SpriteBundle {
                            texture: asset_server.load("PNG/Bullets/shot.png"),
                            transform: transform.clone()
                                .with_translation(Vec3::new(
                                    transform.translation.x, 
                                    transform.translation.y + 6.0, 
                                    transform.translation.z - 1.0)),
                            ..default()
                        },
                    ))
                    .insert(Sensor)
                    .insert(ActiveEvents::COLLISION_EVENTS)
                    .insert(RigidBody::Dynamic) // 
                    .insert(Velocity{
                        linvel: force.truncate(),
                        angvel: 0.0,
                    })
                    .insert(Damping { // friction
                        linear_damping: 0.3,
                        angular_damping: 5.0,
                    })
                    .insert(Collider::capsule_x(1.0, 1.0)) // shape of collider
                    .insert(GravityScale(25.0))
                    .insert(Restitution::coefficient(0.0))
                    .insert(Projectile)
                    .insert(CollisionGroups::new(Group::GROUP_1,Group::GROUP_2));

                let rand: i32 = rand::thread_rng().gen_range(1..5);
                let sound_effect = asset_server.load(format!("sounds/footstep_carpet_00{rand}.ogg"));
                audio.play(sound_effect);
            }

        }
    }
    
}