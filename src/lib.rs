#![feature(const_fn_trait_bound)]

extern crate num_traits;
#[cfg(binverse_impls)]
extern crate binverse;

use std::ops;

pub mod mat;
pub mod vec;
pub mod swizzle;

pub const RADIANS_TO_DEGREES: f32 = 180.0 / std::f32::consts::PI;

#[inline(always)]
pub fn min<T>(a: T, b: T) -> T
where T: std::cmp::Ord {
    if a < b { a } else {b}
}
#[inline(always)]
pub fn max<T>(a: T, b: T) -> T
where T: std::cmp::Ord {
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


#[cfg(test)]
mod test {
    #[test]
    fn swizzle() {
        use crate::{swizzle::*, vec::*};
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(Vec4i::vec_from(v.xxxx()), Vec4i::fill(1));
        assert_eq!(v.zyxx(), Vec4::new(3.0, 2.0, 1.0, 1.0));
    }

    #[test]
    fn vec_operations() {
        use crate::vec::*;
        let mut v1 = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v1.magnitude(), 14.0_f32.sqrt());
        v1 += 1.0;
        assert_eq!(v1.y, 3.0);
        v1 -= Vec3::new(2.0, 1.0, 0.0);
        assert_eq!(v1, Vec3::new(0.0, 2.0, 4.0));
        v1 += v1;
    }
}