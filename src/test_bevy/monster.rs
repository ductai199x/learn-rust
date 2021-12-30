use bevy::core::FixedTimestep;
use rand::{thread_rng, Rng};

use bevy::input::*;
use bevy::prelude::*;
use bevy::window::*;

use crate::test_bevy::*;

pub struct MonsterPlugin;

impl Plugin for MonsterPlugin {
    fn build(&self, app: &mut AppBuilder) {
        app.add_startup_stage(
            "game_setup_monster",
            SystemStage::single(monster_spawn.system()),
        )
        .add_system_set(
            SystemSet::new()
                .label("monster_movement_system")
                .with_run_criteria(FixedTimestep::step(0.3))
                .with_system(monster_movement.system()),
        )
        .add_system(monster_collision.system().label("monster_collision_system"))
        .add_system(
            monster_update_hitbox
                .system()
                .label("monster_update_hitbox_system")
                .after("monster_collision_system"),
        );
    }
}

fn monster_spawn(mut commands: Commands, materials: Res<Materials>, win_size: Res<GameWindowSize>) {
    let (monster_w, monster_h): (f32, f32) = (35., 35.);
    // sp = spawn point - in the top-left origin coordinate
    let mut sp = Point::new((100., 100.));
    // convert sp to bevy's sp
    let mut bevy_sp = topleft_to_mid_origin(&sp, &win_size);
    // since bevy spawns's center matches its sp, we need our sp to be its top-left corner
    bevy_sp = bevy_sp + Point::new((monster_w / 2., -monster_h / 2.));

    commands
        .spawn_bundle(SpriteBundle {
            material: materials.monster_materials.clone(),
            sprite: Sprite::new(Vec2::new(monster_w, monster_h)),
            transform: Transform::from_xyz(bevy_sp.x(), bevy_sp.y(), 1.),
            ..Default::default()
        })
        .insert(MonsterEntity)
        .insert(NameCmp(STR("ItsMe")))
        .insert(MovementCmp {
            position: sp.0,
            speed: Vec2::new(600., 600.),
            direction: Vec2::new(1., 1.),
            delta: Vec2::new(0., 0.),
        })
        .insert(RectangleHitboxCmp {
            rect: collision::Rect::new((sp, monster_w, monster_h)),
        });

    println!("Spawned Monster at ({} -> {})", sp, bevy_sp);
}

fn monster_movement(
    mut monster_query: Query<(&mut MovementCmp, &mut Transform), With<MonsterEntity>>,
) {
    if let Ok((mut monster_mov, mut monster_tf)) = monster_query.single_mut() {
        let mut rng = thread_rng();
        let move_arr: [u32; 8] = [8, 4, 2, 1, 10, 9, 6, 5];
        let dir_choice: usize = rng.gen_range(0..8);

        // let move_arr: [u32; 4] = [8, 4, 2, 1];
        // let dir_choice: usize = rng.gen_range(0..4);

        let mut direction = match move_arr[dir_choice] {
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

        monster_mov.direction = direction;
    }
}

fn monster_collision(
    mut commands: Commands,
    mut materials: ResMut<Assets<ColorMaterial>>,
    windows: Res<Windows>,
    mut monster_query: Query<
        (&mut MovementCmp, &RectangleHitboxCmp, &mut Transform),
        With<MonsterEntity>,
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
    if let Ok((mut monster_mov, monster_hitbox, mut monster_tf)) = monster_query.single_mut() {
        let mut delta = monster_mov.direction * monster_mov.speed * TIME_STEP;

        // reset direction
        let mut next_direction = Vec2::ZERO;

        let mut contact_normals = 0_u32;

        for (wall_entity, wall_hitbox, wall_collide, wall_color) in wall_query.iter_mut() {
            let color = &mut materials.get_mut(wall_color).unwrap().color;

            let collided2 = wall_hitbox.rect.is_rect_intersect(&monster_hitbox.rect);

            let (collided, contact_point, contact_normal, contact_time) =
                wall_hitbox.rect.is_rect_collide(
                    &monster_hitbox.rect,
                    &monster_mov.direction,
                    &monster_mov.speed,
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

                    if (wall_collide.property | CollideType::Reflect as u32)
                        == wall_collide.property
                    {
                        delta += resolve_collision_reflect(
                            resolving_side,
                            &monster_mov.speed,
                            contact_time.unwrap(),
                            TIME_STEP,
                            0.5,
                        );
                        next_direction += rect_collision_side_to_vec2(resolving_side);
                    }
                }
                contact_normals = contact_normals_new;
            } else {
                set_bevy_color_rgba(color, 1., 1., 1., 1.);
            }
        }
        if next_direction.cmpne(Vec2::ZERO).all() {
            monster_mov.direction = next_direction;
        }

        monster_mov.delta = delta;
        monster_mov.position += delta;
        monster_tf.translation.x += delta.x;
        monster_tf.translation.y += -delta.y;
    }
}

fn monster_update_hitbox(
    mut monster_query: Query<(&MovementCmp, &mut RectangleHitboxCmp), With<MonsterEntity>>,
) {
    if let Ok((monster_mov, mut monster_hitbox)) = monster_query.single_mut() {
        monster_hitbox
            .rect
            .r#move(monster_mov.delta.x, monster_mov.delta.y);
    }
}
