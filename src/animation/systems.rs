use bevy::prelude::*;

use super::components::*;

pub fn animate_sprite(
    time: Res<Time>,
    mut query: Query<(
        &AnimationIndices,
        &mut AnimationTimer,
        &mut TextureAtlasSprite,
    )>,
) {
    for (indices, mut timer, mut sprite) in &mut query {
        timer.tick(time.delta());
        if timer.just_finished() {
            sprite.index = if sprite.index == indices.last {
                indices.first
            } else {
                sprite.index + 1
            };
        }
    }
}

pub fn cycle_images(
    mut bg_query: Query<(&mut Transform, &CyclicImage), With<CyclicImage>>,
){
    for (mut transform, cycle) in bg_query.iter_mut(){
        transform.translation += cycle.speed;
        
        if let Some(z) = cycle.zone {
            let current_zone = Rect::new(
                transform.translation.x, 
                transform.translation.y, 
                transform.translation.x + z.width(), 
                transform.translation.y + z.height()
            );
    
            if z.intersect(current_zone).is_empty() {
                transform.translation.x = z.min.x;
                transform.translation.y = z.min.y;
            }
        }
    }
}