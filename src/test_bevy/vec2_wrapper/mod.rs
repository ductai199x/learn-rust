use bevy::math::{BVec2, Vec2};
use fstrings::*;
use std::{fmt, ops};

#[macro_export]
macro_rules! implement_struct_of_vec2_methods {
    ($iden:ident, $constructor_trait:ident, $update_trait:ident) => {
        impl $iden {
            pub fn new<A>(args: A) -> $iden
            where
                A: $constructor_trait,
            {
                args.construct()
            }

            pub fn x(&self) -> f32 {
                self.0.x
            }

            pub fn y(&self) -> f32 {
                self.0.y
            }

            pub fn get(&self) -> (f32, f32) {
                (self.0.x, self.0.y)
            }

            pub fn get_vec2(&self) -> Vec2 {
                self.0
            }

            pub fn update<A>(&mut self, args: A)
            where
                $iden: $update_trait<A>,
            {
                self.update_self(args)
            }

            pub fn dot(&self, other: $iden) -> f32 {
                self.0.dot(other.0)
            }

            pub fn abs(&self) -> $iden {
                $iden::new(self.0.abs())
            }

            pub fn is_finite(&self) -> bool {
                self.0.is_finite()
            }

            pub fn is_nan(&self) -> bool {
                self.0.is_nan()
            }

            pub fn perp(&self) -> $iden {
                $iden::new(self.0.perp())
            }

            /// The perpendicular dot product of `self` and `other`.
            pub fn perp_dot(&self, other: &$iden) -> f32 {
                self.0.perp_dot(other.0)
            }

            pub fn length(&self) -> f32 {
                self.0.length()
            }

            pub fn length_squared(&self) -> f32 {
                self.0.length_squared()
            }

            pub fn distance(&self, other: &$iden) -> f32 {
                self.0.distance(other.0)
            }

            pub fn distance_squared(&self, other: &$iden) -> f32 {
                self.0.distance_squared(other.0)
            }

            pub fn try_normalize(&self) -> Option<$iden> {
                if let Some(norm) = self.0.try_normalize() {
                    Some(Self(norm.into()))
                } else {
                    None
                }
            }

            pub fn recip(&self) -> $iden {
                $iden::new(self.0.recip())
            }

            pub fn powf(&self, n: f32) -> $iden {
                $iden::new(self.0.powf(n))
            }

            pub fn exp(&self) -> $iden {
                $iden::new(self.0.exp())
            }

            pub fn angle_between(&self, other: &$iden) -> f32 {
                self.0.angle_between(other.0)
            }

            pub fn cmpeq(&self, other: &$iden) -> BVec2 {
                self.0.cmpeq(other.0)
            }

            pub fn cmpne(&self, other: &$iden) -> BVec2 {
                self.0.cmpne(other.0)
            }

            pub fn cmpge(&self, other: &$iden) -> BVec2 {
                self.0.cmpge(other.0)
            }

            pub fn cmpgt(&self, other: &$iden) -> BVec2 {
                self.0.cmpgt(other.0)
            }

            pub fn cmple(&self, other: &$iden) -> BVec2 {
                self.0.cmple(other.0)
            }

            pub fn cmplt(&self, other: &$iden) -> BVec2 {
                self.0.cmplt(other.0)
            }
        }
    };
}

#[macro_export]
macro_rules! implement_struct_of_vec2_traits {
    ($iden:ident, $constructor_trait:ident, $update_trait:ident) => {
        pub trait $constructor_trait {
            fn construct(self) -> $iden;
        }

        impl $constructor_trait for ($iden) {
            fn construct(self) -> $iden {
                self.clone()
            }
        }
        impl $constructor_trait for (Vec2) {
            fn construct(self) -> $iden {
                $iden(self)
            }
        }
        impl $constructor_trait for (f32, f32) {
            fn construct(self) -> $iden {
                $iden(Vec2::new(self.0, self.1))
            }
        }

        pub trait $update_trait<A> {
            fn update_self(&mut self, args: A);
        }

        impl $update_trait<(f32, f32)> for $iden {
            fn update_self(&mut self, args: (f32, f32)) {
                self.0.x = args.0;
                self.0.y = args.1;
            }
        }

        impl $update_trait<$iden> for $iden {
            fn update_self(&mut self, args: $iden) {
                self.0.x = args.0.x;
                self.0.y = args.0.y;
            }
        }

        impl $update_trait<Vec2> for $iden {
            fn update_self(&mut self, args: Vec2) {
                self.0.x = args.x;
                self.0.y = args.y;
            }
        }

        impl Default for $iden {
            fn default() -> Self {
                Self(Vec2::ZERO) // middle of the screen
            }
        }

        impl fmt::Display for $iden {
            fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
                write_f!(f, "({x} {y})", x = self.x(), y = self.y())
            }
        }

        impl ops::Add<$iden> for $iden {
            type Output = $iden;

            fn add(self, other: $iden) -> $iden {
                $iden(self.0 + other.0)
            }
        }

        impl ops::Add<f32> for $iden {
            type Output = $iden;

            fn add(self, other: f32) -> $iden {
                $iden(self.0 + Vec2::splat(other))
            }
        }

        impl ops::Add<Vec2> for $iden {
            type Output = $iden;

            fn add(self, other: Vec2) -> $iden {
                $iden(self.0 + other)
            }
        }

        impl ops::Sub<$iden> for $iden {
            type Output = $iden;

            fn sub(self, other: $iden) -> $iden {
                $iden(self.0 - other.0)
            }
        }

        impl ops::Sub<f32> for $iden {
            type Output = $iden;

            fn sub(self, other: f32) -> $iden {
                $iden(self.0 - Vec2::splat(other))
            }
        }

        impl ops::Sub<Vec2> for $iden {
            type Output = $iden;

            fn sub(self, other: Vec2) -> $iden {
                $iden(self.0 - other)
            }
        }

        impl ops::Mul<$iden> for $iden {
            type Output = $iden;

            fn mul(self, other: $iden) -> $iden {
                $iden(self.0 * other.0)
            }
        }

        impl ops::Mul<f32> for $iden {
            type Output = $iden;

            fn mul(self, other: f32) -> $iden {
                $iden(self.0 * other)
            }
        }

        impl ops::Mul<Vec2> for $iden {
            type Output = $iden;

            fn mul(self, other: Vec2) -> $iden {
                $iden(self.0 * other)
            }
        }

        impl ops::Div<$iden> for $iden {
            type Output = $iden;

            fn div(self, other: $iden) -> $iden {
                $iden(self.0 / other.0)
            }
        }

        impl ops::Div<f32> for $iden {
            type Output = $iden;

            fn div(self, other: f32) -> $iden {
                $iden(self.0 / other)
            }
        }

        impl ops::Div<Vec2> for $iden {
            type Output = $iden;

            fn div(self, other: Vec2) -> $iden {
                $iden(self.0 / other)
            }
        }
    };
}
