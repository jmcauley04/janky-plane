use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile {
    pub speed: usize,
    pub mass: usize,
}