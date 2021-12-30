#[macro_use]
use bevy;
use bevy::prelude::*;
use fstrings::*;
use std::{fmt, mem::swap, ops};

use crate::{implement_struct_of_vec2_methods, implement_struct_of_vec2_traits};

#[derive(Debug, Clone, Copy)]
pub struct Point(pub Vec2);
implement_struct_of_vec2_traits!(Point, ConstructorPoint, UpdatePoint);
implement_struct_of_vec2_methods!(Point, ConstructorPoint, UpdatePoint);

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    pub start: Point,
    pub end: Point,
}
#[derive(Debug, Clone, Copy)]
pub enum RectCollisionSide {
    None = 0,
    Left = 1,
    Right = 2,
    Top = 4,
    Bottom = 8,
    TopLeft = 5,
    TopRight = 6,
    BottomLeft = 9,
    BottomRight = 10,
}

pub fn u32_to_rect_collision_side(num: u32) -> RectCollisionSide {
    match num {
        0 => RectCollisionSide::None,
        1 => RectCollisionSide::Left,
        2 => RectCollisionSide::Right,
        4 => RectCollisionSide::Top,
        8 => RectCollisionSide::Bottom,
        5 => RectCollisionSide::TopLeft,
        6 => RectCollisionSide::TopRight,
        9 => RectCollisionSide::BottomLeft,
        10 => RectCollisionSide::BottomRight,
        _ => RectCollisionSide::None,
    }
}

pub fn rect_collision_side_to_vec2(side: RectCollisionSide) -> Vec2 {
    match side {
        RectCollisionSide::None => Vec2::new(0., 0.),
        RectCollisionSide::Left => Vec2::new(-1., 0.),
        RectCollisionSide::Right => Vec2::new(1., 0.),
        RectCollisionSide::Top => Vec2::new(0., -1.),
        RectCollisionSide::Bottom => Vec2::new(0., 1.),
        RectCollisionSide::TopLeft => Vec2::new(-1., -1.),
        RectCollisionSide::TopRight => Vec2::new(1., -1.),
        RectCollisionSide::BottomLeft => Vec2::new(-1., 1.),
        RectCollisionSide::BottomRight => Vec2::new(1., 1.),
    }
}

pub fn resolve_collision_stop(
    side: RectCollisionSide,
    speed: &Vec2,
    contact_time: f32,
    time_step: f32,
) -> Vec2 {
    rect_collision_side_to_vec2(side) * (*speed) * (1_f32 - contact_time) * time_step
}

pub fn resolve_collision_reflect(
    side: RectCollisionSide,
    speed: &Vec2,
    contact_time: f32,
    time_step: f32,
    reflect_speed_ratio: f32,
) -> Vec2 {
    rect_collision_side_to_vec2(side)
        * (*speed)
        * (1_f32 - contact_time)
        * time_step
        * (1_f32 + reflect_speed_ratio)
}

#[derive(Debug, Clone, Copy)]
pub struct Rect {
    pub upper_left: Point,
    pub lower_right: Point,
    pub width: f32,
    pub height: f32,
}
pub trait IntoRect {
    fn into(self) -> Rect;
}
impl IntoRect for (Point, Point) {
    fn into(self) -> Rect {
        let (x1, x2) = self;
        let ul = Point::new((x1.x().min(x2.x()), x1.y().max(x2.y())));
        let lr = Point::new((x1.x().max(x2.x()), x1.y().min(x2.y())));
        let w = lr.x() - ul.x();
        let h = ul.y() - lr.y();
        Rect {
            upper_left: ul,
            lower_right: lr,
            width: w,
            height: h,
        }
    }
}
impl IntoRect for (Point, f32, f32) {
    fn into(self) -> Rect {
        let (ul, w, h) = self;
        let lr = Point::new((ul.x() + w, ul.y() + h));
        Rect {
            upper_left: ul,
            lower_right: lr,
            width: w,
            height: h,
        }
    }
}
impl Rect {
    pub fn new<A>(args: A) -> Rect
    where
        A: IntoRect,
    {
        args.into()
    }

    pub fn area(&self) -> f32 {
        self.width * self.height
    }

    pub fn r#move(&mut self, delta_x: f32, delta_y: f32) {
        let delta = Point::new((delta_x, delta_y));
        self.upper_left = self.upper_left + delta;
        self.lower_right = self.lower_right + delta;
    }

    pub fn is_rect_inside(&self, r2: &Rect) -> bool {
        self.upper_left.x() <= r2.upper_left.x()
            && self.upper_left.y() <= r2.upper_left.y()
            && self.lower_right.x() >= r2.lower_right.x()
            && self.lower_right.y() >= r2.lower_right.y()
    }

    pub fn is_point_inside(&self, p: &Point) -> bool {
        self.upper_left.x() <= p.x()
            && self.upper_left.y() <= p.y()
            && self.lower_right.x() >= p.x()
            && self.lower_right.y() >= p.y()
    }

    pub fn is_rect_intersect(&self, r2: &Rect) -> bool {
        let r2_ur = Point::new((r2.upper_left.x() + r2.width, r2.upper_left.y())); //upper-right
        let r2_ll = Point::new((r2.upper_left.x(), r2.upper_left.y() + r2.height)); //lower-left
        self.is_point_inside(&r2.upper_left)
            || self.is_point_inside(&r2.lower_right)
            || self.is_point_inside(&r2_ur)
            || self.is_point_inside(&r2_ll)
    }

    // (Arbitrary Rectangle Collision Detection & Resolution - Complete!)
    // https://www.youtube.com/watch?v=8JJ-4JgR7Dg
    pub fn is_ray_intersect(
        &self,
        ray: &Ray,
    ) -> (bool, Option<Point>, Option<RectCollisionSide>, Option<f32>) {
        let mut contact_point = Point::new((0., 0.));
        let mut contact_normal = RectCollisionSide::None;
        // Calculate intersections with rectangle bounding axes
        let ray_len_x = ray.end.x() - ray.start.x();
        let ray_len_y = ray.end.y() - ray.start.y();

        let mut near_x = (self.upper_left.x() - ray.start.x()) / ray_len_x;
        let mut near_y = (self.upper_left.y() - ray.start.y()) / ray_len_y;
        let mut far_x = (self.lower_right.x() - ray.start.x()) / ray_len_x;
        let mut far_y = (self.lower_right.y() - ray.start.y()) / ray_len_y;

        if near_x.is_nan() || near_y.is_nan() || far_x.is_nan() || far_y.is_nan() {
            return (false, None, None, None);
        }

        // Sort distances
        if near_x > far_x {
            swap(&mut near_x, &mut far_x);
        }
        if near_y > far_y {
            swap(&mut near_y, &mut far_y);
        }

        // Early rejection
        if near_x > far_y || near_y > far_x {
            return (false, None, None, None);
        }

        // dbg!((near_x, far_y, near_y, far_x));

        // let t_hit_near = near_x.max(near_y);
        let mut t_hit_near = 0_f32;
        if near_x > near_y {
            t_hit_near = near_x;
        } else {
            t_hit_near = near_y;
        }
        let t_hit_far = far_x.max(far_y);

        // Reject if ray direction is Pointing away from object
        if t_hit_far < 0. {
            return (false, None, None, None);
        }

        contact_point.update((ray.start + ray.end * t_hit_near));

        if near_x > near_y {
            if ray_len_x < 0. {
                contact_normal = RectCollisionSide::Right;
            } else if ray_len_x > 0. {
                contact_normal = RectCollisionSide::Left;
            } else {
                contact_normal = RectCollisionSide::None;
            }
        } else if near_x < near_y {
            if ray_len_y < 0. {
                contact_normal = RectCollisionSide::Bottom;
            } else if ray_len_y > 0. {
                contact_normal = RectCollisionSide::Top;
            } else {
                contact_normal = RectCollisionSide::None;
            }
        } else {
            if ray_len_x > 0. && ray_len_y > 0. {
                contact_normal = RectCollisionSide::TopLeft;
            } else if ray_len_x < 0. && ray_len_y < 0. {
                contact_normal = RectCollisionSide::BottomRight;
            } else if ray_len_x > 0. && ray_len_y < 0. {
                contact_normal = RectCollisionSide::BottomLeft;
            } else if ray_len_x < 0. && ray_len_y > 0. {
                contact_normal = RectCollisionSide::TopRight;
            } else {
                contact_normal = RectCollisionSide::None;
            }
            // dbg!((near_x, near_y, far_x, far_y, ray_len_x, ray_len_y));
        }

        return (
            true,
            Some(contact_point),
            Some(contact_normal),
            Some(t_hit_near),
        );
    }

    pub fn is_rect_collide(
        &self,
        r2: &Rect,
        r2_direction: &Vec2,
        r2_speed: &Vec2,
        time_step: f32,
    ) -> (bool, Option<Point>, Option<RectCollisionSide>, Option<f32>) {
        if r2_direction.x == 0. && r2_direction.y == 0. {
            return (false, None, None, None);
        }

        // Expand target rectangle by source dimensions
        let expanded_r1 = Rect::new((
            self.upper_left - Point::new((r2.width / 2., r2.height / 2.)),
            self.width + r2.width,
            self.height + r2.height,
        ));

        // ray's origin is the center of r2
        let ray_origin = r2.upper_left + Vec2::new(r2.width / 2., r2.height / 2.);
        // ray's destination is orgin + displacement
        let ray_dest = ray_origin + (*r2_direction) * (*r2_speed) * time_step;
        let r2_ray = Ray {
            start: ray_origin,
            end: ray_dest,
        };

        let (collided, contact_point, contact_normal, contact_time) =
            expanded_r1.is_ray_intersect(&r2_ray);

        let res = collided && (contact_time.unwrap() >= 0.0 && contact_time.unwrap() <= 1.0);

        if res {
            return (true, contact_point, contact_normal, contact_time);
        } else {
            return (false, None, None, None);
        }
    }
}
impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write_f!(f, "{{ ul: {self.upper_left}, lr: {self.lower_right}, w: {self.width}, h: {self.height} }}")
    }
}
