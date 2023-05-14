use bevy::prelude::*;

#[derive(Component)]
pub struct Projectile;

#[derive(Component, Deref, DerefMut)]
pub struct FireSpeedTimer(pub Timer);