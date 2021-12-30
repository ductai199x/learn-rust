use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;

use crate::test_bevy::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "game_setup_player",
            SystemStage::single(player_spawn.system()),
        )
        .add_system(player_movement.system().label("player_movement_system"))
        .add_system(
            player_collision
                .system()
                .label("player_collision_system")
                .after("player_movement_system"),
        )
        .add_system(
            player_update_hitbox
                .system()
                .label("player_update_hitbox_system")
                .after("player_collision_system"),
        );
    }
}

fn player_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<GameWindowSize>) {
    let (player_w, player_h): (f32, f32) = (35., 35.);
    // sp = spawn point - in the top-left origin coordinate
    let mut sp = Point::new((100., 100.));
    // convert sp to bevy's sp
    let mut bevy_sp = topleft_to_mid_origin(&sp, &win_size);
    // since bevy spawns's center matches its sp, we need our sp to be its top-left corner
    bevy_sp = bevy_sp + Point::new((player_w / 2., -player_h / 2.));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.player_materials.clone(),
            sprite: Sprite::new(Vec2::new(player_w, player_h)),
            transform: Transform::from_xyz(bevy_sp.x(), bevy_sp.y(), 1.),
            ..Default::default()
        })
        .insert(PlayerEntity)
        .insert(NameCmp(STR("ItsMe")))
        .insert(MovementCmp {
            position: sp.0,
            speed: Vec2::new(800., 800.),
            direction: Vec2::new(0., 0.),
            delta: Vec2::new(0., 0.),
        })
        .insert(RectangleHitboxCmp {
            rect: collision::Rect::new((sp, player_w, player_h)),
        });

    println!("Spawned Player at ({} -> {})", sp, bevy_sp);
}

fn player_movement(
    keyboard_input: Res<Input<KeyCode>>,
    mut player_query: Query<(&mut MovementCmp, &mut Transform), With<PlayerEntity>>,
) {
    if let Ok((mut player_mov, mut player_tf)) = player_query.single_mut() {
        let up = keyboard_input.pressed(KeyCode::W) as u32;
        let down = keyboard_input.pressed(KeyCode::S) as u32;
        let left = keyboard_input.pressed(KeyCode::A) as u32;
        let right = keyboard_input.pressed(KeyCode::D) as u32;

        let mut direction = match (up << 3) | (down << 2) | (left << 1) | right {
            8 => Vec2::new(0., -1.),
            4 => Vec2::new(0., 1.),
            2 => Vec2::new(-1., 0.),
            1 => Vec2::new(1., 0.),
            10 => Vec2::new(-1., -1.),
            9 => Vec2::new(1., -1.),
            6 => Vec2::new(-1., 1.),
            5 => Vec2::new(1., 1.),
            _ => Vec2::new(0., 0.),
        };

        player_mov.direction = direction;
    }
}

fn player_collision(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mut player_query: Query<
        (&mut MovementCmp, &RectangleHitboxCmp, &mut Transform),
        With<PlayerEntity>,
    >,
    mut wall_query: Query<
        (
            Entity,
            &RectangleHitboxCmp,
            &CollideCmp,
            &Handle<ColorMaterial>,
        ),
        With<WallEntity>,
    >,
) {
    if let Ok((mut player_mov, player_hitbox, mut player_tf)) = player_query.single_mut() {
        let mut delta = player_mov.direction * player_mov.speed * TIME_STEP;

        let mut contact_normals = 0_u32;

        for (wall_entity, wall_hitbox, wall_collide, wall_color) in wall_query.iter_mut() {
            let color = &mut materials.get_mut(wall_color).unwrap().color;

            let (collided, contact_point, contact_normal, contact_time) =
                wall_hitbox.rect.is_rect_collide(
                    &player_hitbox.rect,
                    &player_mov.direction,
                    &player_mov.speed,
                    TIME_STEP,
                );

            if collided {
                set_bevy_color_rgba(color, 0., 1., 1., 1.);

                // resolve the collision
                // Only add to delta unique normals since the moving block can collide with multiple
                // blocks resulting in duplication of the delta += .. statement.
                let contact_normals_new: u32 = contact_normals | contact_normal.unwrap() as u32;
                if contact_normals != contact_normals_new {
                    let resolving_side =
                        u32_to_rect_collision_side(contact_normals ^ contact_normals_new);

                    if (wall_collide.property | CollideType::Stop as u32) == wall_collide.property {
                        delta += resolve_collision_stop(
                            resolving_side,
                            &player_mov.speed,
                            contact_time.unwrap(),
                            TIME_STEP,
                        );
                    }
                }
                contact_normals = contact_normals_new;
            } else {
                set_bevy_color_rgba(color, 1., 1., 1., 1.);
            }
        }
        player_mov.delta = delta;
        player_mov.position += delta;
        player_tf.translation.x += delta.x;
        player_tf.translation.y += -delta.y;
        // reset player's direction vector
        player_mov.direction = Vec2::ZERO;
    }
}

fn player_update_hitbox(
    mut player_query: Query<(&MovementCmp, &mut RectangleHitboxCmp), With<PlayerEntity>>,
) {
    if let Ok((player_mov, mut player_hitbox)) = player_query.single_mut() {
        player_hitbox
            .rect
            .r#move(player_mov.delta.x, player_mov.delta.y);
    }
}
