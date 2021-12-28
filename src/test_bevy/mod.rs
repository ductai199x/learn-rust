#![allow(unused)]

mod collision;
use collision::*;

mod player;
use player::PlayerPlugin;

mod map;
use map::MapPlugin;

mod helper;
use helper::*;

use fstrings::*;

use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};

const TIME_STEP: f32 = 1. / 60.;

// Shared Resources

pub struct Materials {
    player_materials: Handle<ColorMaterial>,
    wall_materials: Handle<ColorMaterial>,
    ray_materials: Handle<ColorMaterial>,
}
#[derive(Debug)]
pub struct GameWindowSize {
    w: f32,
    h: f32,
}
#[derive(Debug)]
pub struct CursorPosition {
    x: Option<f32>,
    y: Option<f32>,
}
// Shared Resources

// Components
#[derive(Default)]
pub struct PositionCmp(Vec2);

#[derive(Default)]
pub struct SpeedCmp(Vec2);

#[derive(Default)]
pub struct MovementCmp {
    position: Vec2,
    speed: Vec2,
    direction: Vec2,
    delta: Vec2,
}

pub struct NameCmp(String);
impl Default for NameCmp {
    fn default() -> Self {
        Self(STR("None"))
    }
}
#[derive(Debug)]
pub struct RectangleHitboxCmp {
    rect: collision::Rect,
}
#[derive(Debug)]
pub struct RayCmp {
    ray: collision::Ray,
    thickness: f32,
    length: f32,
}
// Components

// Entities
pub struct PlayerEntity;
pub struct WallEntity;
pub struct RayEntity;
// Entities

pub fn bevy_main() {
    App::build()
        .insert_resource(ClearColor(Color::rgb(0., 0., 0.)))
        .insert_resource(WindowDescriptor {
            title: STR("Example Breakout"),
            width: 600.,
            height: 600.,
            ..Default::default()
        })
        .add_plugins(DefaultPlugins)
        .add_plugin(PlayerPlugin)
        .add_plugin(MapPlugin)
        .add_plugin(FrameTimeDiagnosticsPlugin)
        // .add_plugin(LogDiagnosticsPlugin::default())
        .add_startup_system(setup.system())
        .add_startup_stage("game_setup_rays", SystemStage::single(ray_spawn.system()))
        .add_system(get_window_size.system())
        .add_system(get_cursor_pos.system())
        .add_system(ray_move.system())
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
        wall_materials: materials.add(Color::rgb(1., 1., 1.).into()),
        ray_materials: materials.add(Color::rgb(1., 1., 1.).into()),
    });
    commands.insert_resource(GameWindowSize {
        w: window.width(),
        h: window.height(),
    });
    commands.insert_resource(CursorPosition { x: None, y: None });
}

fn get_window_size(mut win_size: ResMut<GameWindowSize>, mut events: EventReader<WindowResized>) {
    for event in events.iter() {
        win_size.w = event.width;
        win_size.h = event.height;
    }
}

fn get_cursor_pos(
    windows: Res<Windows>,
    win_size: Res<GameWindowSize>,
    mut cursor_pos: ResMut<CursorPosition>,
    mut events: EventReader<CursorMoved>,
) {
    let window = windows.get_primary().unwrap();
    if let Some(cursor_pos_) = window.cursor_position() {
        let converted_pos = botleft_to_toplef_origin(&Point(cursor_pos_), &win_size);
        cursor_pos.x = Some(converted_pos.x());
        cursor_pos.y = Some(converted_pos.y());
        // dbg!((cursor_pos.x, cursor_pos.y));
    } else {
        cursor_pos.x = None;
        cursor_pos.y = None;
    }
}

pub fn get_ray_tf_mat(
    ray: &collision::Ray,
    thickness: f32,
    win_size: &Res<GameWindowSize>,
) -> (Point, Mat4) {
    let sp = ray.start; // start point
    let ep = ray.end; // end point

    let ray_l = sp.0.distance(ep.0);
    let ray_t = thickness;
    let ray_z = 10.;
    let ray_angle = (ep.0 - sp.0).angle_between(Vec2::new(1., 0.));

    let mut bevy_sp = topleft_to_mid_origin(&sp, win_size);
    bevy_sp = bevy_sp
        + Point::new((
            (ray_l / 2.) * ray_angle.cos(),
            (ray_l / 2.) * ray_angle.sin() - ray_t,
        ));

    let tf_mat = Mat4::from_rotation_translation(
        Quat::from_rotation_z(ray_angle),
        Vec3::new(bevy_sp.x(), bevy_sp.y(), ray_z),
    );

    return (bevy_sp, tf_mat);
}

fn ray_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<GameWindowSize>) {
    let sp = Point::new((150., 150.)); // start point
    let ep = Point::new((210., 400.)); // end point
    let length = sp.0.distance(ep.0);
    let thickness = 2.;
    let ray = collision::Ray { start: sp, end: ep };
    let (bevy_sp, tf_mat) = get_ray_tf_mat(&ray, thickness, &win_size);

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.ray_materials.clone(),
            sprite: Sprite::new(Vec2::new(length, thickness)),
            transform: Transform::from_matrix(tf_mat),
            ..Default::default()
        })
        .insert(RayEntity)
        .insert(NameCmp(STR("Ray1")))
        .insert(RayCmp {
            ray,
            thickness,
            length,
        });

    println_f!("Spawned Ray at ({sp} -> {bevy_sp}) with length {length}");
}

fn ray_move(
    win_size: Res<GameWindowSize>,
    cursor_pos: Res<CursorPosition>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    mut ray_query: Query<(
        &mut RayCmp,
        &mut Transform,
        &Handle<ColorMaterial>,
        With<RayEntity>,
    )>,
    wall_query: Query<(
        Entity,
        &RectangleHitboxCmp,
        &Handle<ColorMaterial>,
        &WallEntity,
    )>,
) {
    if let Ok((mut ray_comp, mut ray_tf, mut ray_color, _)) = ray_query.single_mut() {
        if cursor_pos.x != None && cursor_pos.y != None {
            ray_comp.ray.end = Point::new((cursor_pos.x.unwrap(), cursor_pos.y.unwrap()));
            let new_length = ray_comp.ray.start.0.distance(ray_comp.ray.end.0);
            let (bevy_sp, tf_mat) = get_ray_tf_mat(&ray_comp.ray, ray_comp.thickness, &win_size);
            let (_, rot_mat, tran_mat) = tf_mat.to_scale_rotation_translation();
            ray_tf.scale = Vec3::new(new_length / ray_comp.length, 1., 1.);
            ray_tf.rotation = rot_mat;
            ray_tf.translation = tran_mat;

            

            for (wall_entity, wall_rect, wall_color, _) in wall_query.iter() {
                let wall_color = &mut materials.get_mut(wall_color).unwrap().color;
                let (collided, _, _, contact_time) = wall_rect.rect.is_ray_intersect(&ray_comp.ray);
                if collided && contact_time.unwrap() < 1_f32 {
                    set_bevy_color_rgba(wall_color, 0., 1., 1., 1.);
                } else {
                    set_bevy_color_rgba(wall_color, 1., 1., 1., 1.);
                }
            }
        }
    }
}
