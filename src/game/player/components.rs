use bevy::prelude::*;
use leafwing_input_manager::Actionlike;

#[derive(Component)]
pub struct Glider;

#[derive(Component)]
pub struct Player;

#[derive(Actionlike, PartialEq, Eq, Hash, Clone, Copy, Debug)]
pub enum Action {
    Move,
    Fire,
}