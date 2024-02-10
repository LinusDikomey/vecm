pub mod mat;
pub mod vec;
pub mod swizzle;

mod quaternion;

pub use quaternion::Quaternion;

use std::ops;

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
    use crate::{swizzle::*, vec::*};
    use crate::{vec2, vec3, vec4};

    #[test]
    fn swizzle() {
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
        assert_eq!(v1, Vec3::new(0.0, 4.0, 8.0));

        let v2: Vec3b = vec3![-1_i8].vec_into();
        assert_eq!(v2, vec3![255]);
    }

    #[test]
    fn macros() {
        let v1 = vec2![1.0, 2.0];
        assert_eq!(v1, (vec3![1.0] + vec3![0.0, 1.0, 0.0]).xy());

        assert_eq!(
            Vec3i::new(1, 2, 3),
            vec3![1, 2, 3]
        );
        assert_eq!(
            PolyVec3::<u16>::fill(12),
            vec3![12]
        );
        assert_eq!(
            vec4![12.0, 4.0, ..],
            Vec4::new(12.0, 4.0, 0.0, 0.0)
        );

        assert_eq!(vec4![1.0, 2.0, 3.0, 4.0].yzw().square_magnitude(), 4.0 + 9.0 + 16.0);
    }

    #[cfg(feature = "binverse")]
    mod binverse_tests {
        use binverse::{serialize::Serialize, streams::{Deserializer, Serializer}};

        use crate::{*, vec::*};

        #[test]
        fn binverse_vecs() {
            let mut s = Serializer::new(Vec::<u8>::new(), 0).unwrap();
            let v: Vec4i = vec4![12, 7, 42, 8];
            v.serialize(&mut s).unwrap();
            let mut d = Deserializer::new(std::io::Cursor::new(s.finish())).unwrap();
            assert_eq!(v, d.deserialize().unwrap());
        }
    }
}

