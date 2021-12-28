#![allow(unused)]

mod collision;

use collision::*;
use bevy;
use bevy::prelude::*;

#[allow(non_snake_case)]
fn STR(s: &str) -> String {
    s.to_string()
}

const TIME_STEP: f32 = 1. / 60.;

// Shared Resources
struct Materials {
    player_materials: Handle<ColorMaterial>,
}
struct GameWindowSize {
    w: f32,
    h: f32,
}
// Shared Resources

// Components
struct PositionComponent {
    x: f32,
    y: f32,
}
impl Default for PositionComponent {
    fn default() -> Self {
        Self { x: 0., y: 0. }
    }
}

struct NameComponent(String);
impl Default for NameComponent {
    fn default() -> Self {
        Self(STR("None"))
    }
}

struct SpeedComponent(f32);
impl Default for SpeedComponent {
    fn default() -> Self {
        Self(0.)
    }
}

struct CollisionRectComponent {
    
}
// Components

// Entities
struct PlayerEntity;
struct WallEntity;
// Entities

pub fn bevy_main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: STR("Example Breakout"),
            width: 480.,
            height: 640.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_startup_system(setup.system())
        .add_startup_stage(
            "game_setup_actors",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system())
        .run();
}

fn setup(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut windows: ResMut<Windows>,
) {
    // camera
    commands.spawn_bundle(OrthographicCameraBundle::new_2d());

    // position window
    let mut window = windows.get_primary_mut().unwrap();
    window.set_position(IVec2::new(0, 0));

    // create main resources
    commands.insert_resource(Materials {
        player_materials: materials.add(Color::rgb(1., 0.7, 0.7).into()),
    });
    commands.insert_resource(GameWindowSize {
        w: window.width(),
        h: window.height(),
    });
}

fn player_spawn(
    mut commands: Commands,
    materials: Res<Materials>,
    game_win_size: Res<GameWindowSize>,
) {
    // spawn a sprite
    let bottom = -game_win_size.h / 2.;
    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            sprite: Sprite::new(Vec2::new(50., 10.)),
            transform: Transform {
                translation: Vec3::new(0., bottom + 10., 0.),
                ..Default::default()
            },
            ..Default::default()
        })
        .insert(PlayerEntity)
        .insert(NameComponent(STR("ItsMe")))
        .insert(SpeedComponent(100.))
        .insert(PositionComponent::default());
}

fn player_movement (
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&SpeedComponent, &mut PositionComponent, &mut Transform, With<PlayerEntity>)>
) {
    if let Ok((speed, mut pos, mut transform, _)) = query.single_mut() {
        let mut direction = 
        if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else if keyboard_input.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };
        pos.x += direction * speed.0 * TIME_STEP;
        transform.translation.x = pos.x;
    }
}

fn player_collision (
    keyboard_input: Res<Input<KeyCode>>,
    mut query: Query<(&SpeedComponent, &mut PositionComponent, &mut Transform, With<PlayerEntity>)>
) {
    if let Ok((speed, mut pos, mut transform, _)) = query.single_mut() {
        let mut direction = 
        if keyboard_input.pressed(KeyCode::A) {
            -1.
        } else if keyboard_input.pressed(KeyCode::D) {
            1.
        } else {
            0.
        };
        pos.x += direction * speed.0 * TIME_STEP;
        transform.translation.x = pos.x;
    }
}