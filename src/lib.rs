#![allow(incomplete_features)]
#![feature(const_fn_trait_bound)]
#![feature(generic_const_exprs)]

extern crate num_traits;
#[cfg(binverse_impls)]
extern crate binverse;

use std::ops;

pub mod mat;
pub mod vec;
pub mod bin;

pub const RADIANS_TO_DEGREES: f32 = 180.0 / std::f32::consts::PI;

#[inline(always)]
pub fn min(a: f32, b: f32) -> f32 {
    if a < b { a } else {b}
}
#[inline(always)]
pub fn max(a: f32, b: f32) -> f32 {
    if a > b { a } else {b}
}

#[inline(always)]
pub fn lerp<T>(a: T, b: T, t: T) -> T 
where T : ops::Add<Output = T> + ops::Sub<Output = T> + ops::Mul<Output = T> + Copy {
    a + (b - a) * t
}

#[inline(always)]
pub fn cubic<T>(x: T) -> T 
where T : ops::Mul<Output = T> + Copy {
    x * x * x
}