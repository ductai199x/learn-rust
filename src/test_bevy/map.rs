use fstrings::*;
use ascii::AsciiString;

use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;

use crate::test_bevy::*;

pub struct Map {
    map_size: Vec2,
    block_size: Vec2,
    topleft: Vec2,
    map_string: AsciiString,
}

pub fn get_map() -> Map {
    Map {
        map_size: Vec2::new(20., 20.),
        block_size: Vec2::new(20., 20.),
        topleft: Vec2::new(20., 20.),
        map_string: AsciiString::from_ascii(
"********************
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
*                  *
********  **********").unwrap(),
    }
} 

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "game_setup_walls", 
            SystemStage::single(map_spawn.system())
        );
    }
}


fn map_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<GameWindowSize>) {
    let map = get_map();

    let mut wall_spawn = |sp: Vec2, block_size: Vec2| {
        let (wall_w, wall_h): (f32, f32) = (block_size.x, block_size.y);
        let mut sp = Point::new(sp);
        let mut bevy_sp = topleft_to_mid_origin(&sp, &win_size);
        bevy_sp = bevy_sp + Point::new((wall_w / 2., -wall_h / 2.));
    
        commands
            .spawn_bundle(SpriteBundle {
                material: materials.wall_materials.clone(),
                sprite: Sprite::new(Vec2::new(wall_w, wall_h)),
                transform: Transform::from_xyz(bevy_sp.x(), bevy_sp.y(), 1.),
                ..Default::default()
            })
            .insert(WallEntity)
            .insert(NameCmp(STR("Wall1")))
            .insert(RectangleHitboxCmp {
                rect: collision::Rect::new((sp, wall_w, wall_h)),
            });
        println!("Spawned Wall at ({} -> {})", sp, bevy_sp);
    };
    
    let mut row = 0_f32;
    let mut col = 0_f32;
    for char in map.map_string.into_iter() {
        if *char == '\n' {
            row += 1_f32;
            col = 0_f32;
        }
        if *char == '*' {
            let sp = map.topleft + Vec2::new(col * map.block_size.x, row * map.block_size.y);
            wall_spawn(sp, map.block_size);
        }
        col += 1_f32;
    }
}