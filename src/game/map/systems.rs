use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::*;

use crate::animation::components::CyclicImage;

use super::{components::Environment, environment_maps::*};

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
                    speed: Vec3::new(-2.0, 0.0, 0.0),
                    zone: Some(Rect::new(x, y, x + width, y + height))
                }
            ));
    }
}

pub fn spawn_tiles (
    mut commands: Commands,
    asset_server: Res<AssetServer>,
)
{
    let width = 64.0;
    let height = 64.0;
    let max_height = 12;
    let mut x = 0;
    let mut y = 0;
    for c in MAP_1.chars() {
        if c == '\n' {            
            y = 0;
            x += 1;
            continue;
        } else if let Some(image) = match c {
            '^' => Some("tiles/yellow/tileYellow_16.png"),
            'v' => Some("tiles/yellow/tileYellow_17.png"),
            '\\' => Some("tiles/yellow/tileYellow_10.png"),
            '/' => Some("tiles/yellow/tileYellow_11.png"),
            '|' => Some("tiles/yellow/tileYellow_06.png"),
            'o' => Some("tiles/yellow/tileYellow_04.png"),
            '`' => Some("tiles/yellow/tileYellow_20.png"),
            ',' => Some("tiles/yellow/tileYellow_19.png"),
            _ => None,
        }{  
            let transform = Transform::from_xyz(x as f32 * width,y as f32 * height, 10.0);
    
            commands.spawn(
                    SpriteBundle {
                        texture: asset_server.load(image),
                        transform: transform,
                        ..default()
                    }
                ).insert(CyclicImage{                    
                    speed: Vec3::new(-8.0, 0.0, 0.0),
                    zone: None
                })
                .insert(get_collider(c, width, height))
                .insert(ActiveEvents::COLLISION_EVENTS)
                .insert(Environment);
        }
        y += 1;  
    }
}

fn get_collider(c: char, width: f32, height: f32) -> Collider {
    return match c {
        // '(' => ,
        //     ')' => ,
            // this triangle works as expected
            '\\' => Collider::triangle(
                Vec2::new(width / 2.0,-height / 2.0),
                Vec2::new(width / 2.0, height / 2.0),  
                Vec2::new(-width / 2.0,-height / 2.0)
            ),
            // this triangle is unreliable
            '/' => Collider::triangle(
                Vec2::new(width / 2.0,-height / 2.0), 
                Vec2::new(-width / 2.0, height / 2.0), 
                Vec2::new(-width / 2.0,-height / 2.0)
            ),
            // '-' => ,
            // 'o' => ,
            // '\'' => ,
            // '`' => ,
            _ => Collider::cuboid(width / 2.0, height / 2.0),
    }
}