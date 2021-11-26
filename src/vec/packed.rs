use super::{PolyVec2, PolyVec3, PolyVec4};

macro_rules! packed_vec {
    ($name2: ident, $name3: ident, $name4: ident, $t: ty) => {
        #[repr(C, packed)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name2 {
            pub x: $t,
            pub y: $t
        }
        impl From<PolyVec2<$t>> for $name2 {
            #[inline]
            fn from(vec: PolyVec2<$t>) -> Self {
                Self { x: vec.x, y: vec.y }
            }
        }

        #[repr(C, packed)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name3 {
            pub x: $t,
            pub y: $t,
            pub z: $t
        }
        impl From<PolyVec3<$t>> for $name3 {
            #[inline]
            fn from(vec: PolyVec3<$t>) -> Self {
                Self { x: vec.x, y: vec.y, z: vec.z }
            }
        }

        #[repr(C, packed)]
        #[derive(Copy, Clone, Debug)]
        pub struct $name4 {
            pub x: $t,
            pub y: $t,
            pub z: $t,
            pub w: $t
        }
        impl From<PolyVec4<$t>> for $name4 {
            #[inline]
            fn from(vec: PolyVec4<$t>) -> Self {
                Self { x: vec.x, y: vec.y, z: vec.z, w: vec.w }
            }
        }
    };
}

packed_vec!(Vec2Packed, Vec3Packed, Vec4Packed, f32);
packed_vec!(Vec2iPacked, Vec3iPacked, Vec4iPacked, i32);
packed_vec!(Vec2uPacked, Vec3uPacked, Vec4uPacked, u32);