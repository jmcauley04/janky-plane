use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;
use rand::Rng;

use crate::game::{player::components::Player, components::Health};

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
    mut health_colliders: Query<(Entity, &mut Health), With<Health>>,
    rapier_context: Res<RapierContext>,
    audio: Res<Audio>,
    asset_server: Res<AssetServer>,
){

    for projectile_entity in projectile_colliders.iter(){    
        for (e1, e2, intersecting) in rapier_context.intersections_with(projectile_entity){
                
            if intersecting {
                for (health_entity, mut health) in health_colliders.iter_mut(){
                    if e1 == health_entity || e2 == health_entity{
                        health.0 -= 10;
                        if health.0 <= 0{
                            commands.entity(health_entity).despawn();
                        }
                    }
                }

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
    mut player_query: Query<(&Transform, &Name, &mut FireSpeedTimer, &CollisionGroups), With<Player>>,
    asset_server: Res<AssetServer>,
    audio: Res<Audio>,
){
    for (transform, name, mut timer, collision_groups) in player_query.iter_mut(){
        timer.tick(time.delta());
        if timer.just_finished(){
            if name.as_str() == "Player 1" && keyboard_input.pressed(KeyCode::Space){
                spawn_bullet(&mut commands, &asset_server, &audio, &transform, collision_groups);
            }
        }
    }    
}

pub fn spawn_bullet(
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
    audio: &Res<Audio>,
    from: &Transform,
    collission_groups: &CollisionGroups,
)
{    
    let force = from.rotation * Vec3::new(SHOT_FORCE, 0.0, 0.0);
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("PNG/Bullets/shot.png"),
            transform: from.clone()
                .with_translation(Vec3::new(
                    from.translation.x, 
                    from.translation.y + 6.0, 
                    from.translation.z - 1.0)),
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
    .insert(collission_groups.clone());

    let rand: i32 = rand::thread_rng().gen_range(1..5);
    let sound_effect = asset_server.load(format!("sounds/footstep_carpet_00{rand}.ogg"));
    audio.play(sound_effect);
}