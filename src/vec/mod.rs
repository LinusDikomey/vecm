use std::fmt;

pub mod packed;
pub mod ops;
pub use ops::*;

pub trait X<T> {
    fn x(&self) -> &T;
    fn set_x(&mut self, x: T);
}
pub trait Y<T> {
    fn y(&self) -> &T;
    fn set_y(&mut self, y: T);
}
pub trait Z<T> {
    fn z(&self) -> &T;
    fn set_z(&mut self, z: T);
}
pub trait W<T> {
    fn w(&self) -> &T;
    fn set_w(&mut self, w: T);
}

/// Used for From/Into conversions of Vector components. For primitive conversions, look at [`VecFrom`]/[`VecInto`] 
pub trait Convertible<T> {
    fn convert(self) -> T;
}

// ---------- Struct definitions ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec2<T> {
    pub x: T,
    pub y: T
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

// ---------- Component traits ----------

macro_rules! impl_comps {
    ($name: ident, $comp: ident, $set: ident,  $($t: ident),*) => {
        $(
            impl<T> $name<T> for $t<T> {
                #[inline(always)]
                fn $comp(&self) -> &T { &self.$comp }
                fn $set(&mut self, v: T) { self.$comp = v; }
            }  
        )*
    };
}
impl_comps!(X, x, set_x, PolyVec2, PolyVec3, PolyVec4);
impl_comps!(Y, y, set_y, PolyVec2, PolyVec3, PolyVec4);
impl_comps!(Z, z, set_z, PolyVec3, PolyVec4);
impl_comps!(W, w, set_w, PolyVec4);

// ---------- Constructors ----------

impl<T> PolyVec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> PolyVec2<T> {
        PolyVec2 { x, y }
    }
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Copy {
        Self { x: val, y: val }
    }
}

impl<T> PolyVec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> PolyVec3<T> {
        PolyVec3 {x, y, z}
    }
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Copy {
        Self { x: val, y: val, z: val }
    }
}

impl<T> PolyVec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> PolyVec4<T> {
        PolyVec4 {x, y, z, w}
    }
    #[inline]
    pub const fn fill(val: T) -> Self where
    T: Copy {
        Self { x: val, y: val, z: val, w: val }
    }
}

// ---------- convenience macros ----------

#[macro_export]
macro_rules! vec2 {
    ($v: expr) => { $crate::vec::PolyVec2 { x: $v, y: $v } };
    ($x: expr, ..) => { $crate::vec::PolyVec2 { x: $x, y: std::default::Default::default() } };
    ($x: expr, $y: expr) => { $crate::vec::PolyVec2 { x: $x, y: $y } };
}
pub use vec2;

#[macro_export]
macro_rules! vec3 {
    ($v: expr) => { $crate::vec::PolyVec3 { x: $v, y: $v, z: $v } };
    ($x: expr, ..) => { $crate::vec::PolyVec3 { x: $x, y: std::default::Default::default(), z: std::default::Default::default() } };
    ($x: expr, $y: expr, ..) => { $crate::vec::PolyVec3 { x: $x, y: $y, z: std::default::Default::default() } };
    ($x: expr, $y: expr, $z: expr) => { $crate::vec::PolyVec3 { x: $x, y: $y, z: $z } };
}
pub use vec3;

#[macro_export]
macro_rules! vec4 {
    ($v: expr) => { $crate::vec::PolyVec4 { x: $v, y: $v, z: $v } };
    ($x: expr, ..) => { $crate::vec::PolyVec4 { x: $x, y: std::default::Default::default(), z: std::default::Default::default(), w: std::default::Default::default() } };
    ($x: expr, $y: expr, ..) => { $crate::vec::PolyVec4 { x: $x, y: $y, z: std::default::Default::default(), w: std::default::Default::default() } };
    ($x: expr, $y: expr, $z: expr, ..) => { $crate::vec::PolyVec4 { x: $x, y: $y, z: $z, w: std::default::Default::default() } };
    ($x: expr, $y: expr, $z: expr, $w: expr) => { $crate::vec::PolyVec4 { x: $x, y: $y, z: $z, w: $w } };
}
pub use vec4;

// ---------- zero/one ----------

impl<T> PolyVec2<T>
where T : num_traits::Zero {
    #[inline]
    pub fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() }
    #[inline]
    pub fn zero() -> Self { Self { x: T::zero(), y: T::zero() } }
}
impl<T> PolyVec2<T>
where T : num_traits::One + PartialEq {
    #[inline]
    pub fn is_one(&self) -> bool { self.x.is_one() && self.y.is_one() }
    #[inline]
    pub fn one() -> Self { Self { x: T::one(), y: T::one() }}
}

impl<T> PolyVec3<T>
where T : num_traits::Zero {
    #[inline]
    pub fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() && self.z.is_zero() }
    #[inline]
    pub fn zero() -> Self { Self { x: T::zero(), y: T::zero(), z: T::zero() } }
}
impl<T> PolyVec3<T>
where T : num_traits::One + PartialEq {
    #[inline]
    pub fn is_one(&self) -> bool { self.x.is_one() && self.y.is_one() && self.z.is_one() }
    #[inline]
    pub fn
    
    one() -> Self { Self { x: T::one(), y: T::one(), z: T::one() } }
}

impl<T> PolyVec4<T>
where T : num_traits::Zero {
    #[inline]
    pub fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero() }
    #[inline]
    pub fn zero() -> Self { Self { x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() } }
}

impl<T> PolyVec4<T>
where T : num_traits::One + PartialEq {
    #[inline]
    pub fn is_one(&self) -> bool { self.x.is_one() && self.y.is_one() && self.z.is_one() && self.w.is_one() }
    #[inline]
    pub fn one() -> Self { Self { x: T::one(), y: T::one(), z: T::one(), w: T::one() } }
}

// ---------- From/Into array ----------

impl<T> From<[T; 2]> for PolyVec2<T> {
    fn from(a: [T; 2]) -> Self {
        let [x, y] = a;
        Self { x, y }
    }
}
impl<T> Into<[T; 2]> for PolyVec2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]  
    }
}

impl<T> From<[T; 3]> for PolyVec3<T> {
    fn from(a: [T; 3]) -> Self {
        let [x, y, z] = a;
        Self { x, y, z }
    }
}
impl<T> Into<[T; 3]> for PolyVec3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}

impl<T> From<[T; 4]> for PolyVec4<T> {
    fn from(a: [T; 4]) -> Self {
        let [x, y, z, w] = a;
        Self { x, y, z, w }
    }
}
impl<T> Into<[T; 4]> for PolyVec4<T> {
    fn into(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}

// ---------- From/Into tuple ----------

impl<T> From<(T, T)> for PolyVec2<T> {
    fn from(t: (T, T)) -> Self {
        Self::new(t.0, t.1)
    }
}
impl<T> Into<(T, T)> for PolyVec2<T> {
    fn into(self) -> (T, T) {
        (self.x, self.y)  
    }
}

impl<T> From<(T, T, T)> for PolyVec3<T> {
    fn from(t: (T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2)
    }
}
impl<T> Into<(T, T, T)> for PolyVec3<T> {
    fn into(self) -> (T, T, T) {
        (self.x, self.y, self.z) 
    }
}

impl<T> From<(T, T, T, T)> for PolyVec4<T> {
    fn from(t: (T, T, T, T)) -> Self {
        Self::new(t.0, t.1, t.2, t.3)
    }
}
impl<T> Into<(T, T, T, T)> for PolyVec4<T> {
    fn into(self) -> (T, T, T, T) {
        (self.x, self.y, self.z, self.w)
    }
}

// ---------- display ----------

impl<T> fmt::Display for PolyVec2<T>
where T: fmt::Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> fmt::Display for PolyVec3<T>
where T: fmt::Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T> fmt::Display for PolyVec4<T>
where T: fmt::Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

// ---------- From/Into conversions ----------

impl<T, U> Convertible<PolyVec2<U>> for PolyVec2<T>
where T: Into<U> {
    fn convert(self) -> PolyVec2<U> {
        PolyVec2 {
            x: self.x.into(),
            y: self.y.into()
        }
    }
}

impl<T, U> Convertible<PolyVec3<U>> for PolyVec3<T>
where T: Into<U> {
    fn convert(self) -> PolyVec3<U> {
        PolyVec3 {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into()
        }
    }
}


impl<T, U> Convertible<PolyVec4<U>> for PolyVec4<T>
where T: Into<U> {
    fn convert(self) -> PolyVec4<U> {
        PolyVec4 {
            x: self.x.into(),
            y: self.y.into(),
            z: self.z.into(),
            w: self.w.into()
        }
    }
}

// ---------- type aliases ----------

pub type Vec2 = PolyVec2<f32>;
pub type Vec2i = PolyVec2<i32>;
pub type Vec2u = PolyVec2<u32>;
pub type Vec2b = PolyVec2<u8>;

pub type Vec3 = PolyVec3<f32>;
pub type Vec3i = PolyVec3<i32>;
pub type Vec3u = PolyVec3<u32>;
pub type Vec3b = PolyVec3<u8>;

pub type Vec4 = PolyVec4<f32>;
pub type Vec4i = PolyVec4<i32>;
pub type Vec4u = PolyVec4<u32>;
pub type Vec4b = PolyVec4<u8>;

// ---------- type conversions ----------

/// Mirror trait for common `as` conversions for vectors not supported by From/Into implementations
pub trait VecFrom<F> {
    fn vec_from(f: F) -> Self;
}

/// Common `as` conversions for vectors not supported by From/Into implementations
pub trait VecInto<I> {
    fn vec_into(self) -> I;
}

impl<F, I> VecFrom<F> for I 
where I: From<F> {
    fn vec_from(f: F) -> Self {
        f.into()
    }
}

impl<F, I> VecInto<I> for F
where I: VecFrom<F> {
    fn vec_into(self) -> I {
        I::vec_from(self)
    }
}

macro_rules! impl_vec_as {
    ($($a: ty => $b: ty),*) => {
        $(
            impl VecFrom<PolyVec2<$a>> for PolyVec2<$b> {
                fn vec_from(a: PolyVec2<$a>) -> Self {
                    Self {
                        x: a.x as $b,
                        y: a.y as $b,
                    }
                }
            }
            impl VecFrom<PolyVec3<$a>> for PolyVec3<$b> {
                fn vec_from(a: PolyVec3<$a>) -> Self {
                    Self {
                        x: a.x as $b,
                        y: a.y as $b,
                        z: a.z as $b,
                    }
                }
            }
            impl VecFrom<PolyVec4<$a>> for PolyVec4<$b> {
                fn vec_from(a: PolyVec4<$a>) -> Self {
                    Self {
                        x: a.x as $b,
                        y: a.y as $b,
                        z: a.z as $b,
                        w: a.w as $b,
                    }
                }
            }
        )*
    };
}

impl_vec_as! {
    // as f32
    u8   => f32,
    u16  => f32,
    u32  => f32,
    u64  => f32,
    u128 => f32,
    i8   => f32,
    i16  => f32,
    i32  => f32,
    i64  => f32,
    i128 => f32,
    f64  => f32,
    // as f64
    u8   => f64,
    u16  => f64,
    u32  => f64,
    u64  => f64,
    u128 => f64,
    i8   => f64,
    i16  => f64,
    i32  => f64,
    i64  => f64,
    i128 => f64,
    f32  => f64,
    // as unsigned int
    u16  => u8,
    u32  => u8,
    u64  => u8,
    u128 => u8,
    i8   => u8,
    i16  => u8,
    i32  => u8,
    i64  => u8,
    i128 => u8,
    f32  => u8,
    f64  => u8,

    u8   => u16,
    u32  => u16,
    u64  => u16,
    u128 => u16,
    i8   => u16,
    i16  => u16,
    i32  => u16,
    i64  => u16,
    i128 => u16,
    f32  => u16,
    f64  => u16,

    u8   => u32,
    u16  => u32,
    u64  => u32,
    u128 => u32,
    i8   => u32,
    i16  => u32,
    i32  => u32,
    i64  => u32,
    i128 => u32,
    f32  => u32,
    f64  => u32,
    
    u8   => u64,
    u16  => u64,
    u32  => u64,
    u128 => u64,
    i8   => u64,
    i16  => u64,
    i32  => u64,
    i64  => u64,
    i128 => u64,
    f32  => u64,
    f64  => u64,

    u8   => u128,
    u16  => u128,
    u32  => u128,
    u64  => u128,
    i8   => u128,
    i16  => u128,
    i32  => u128,
    i64  => u128,
    i128 => u128,
    f32  => u128,
    f64  => u128,
    // as signed int
    u8   => i8,
    u16  => i8,
    u32  => i8,
    u64  => i8,
    u128 => i8,
    i16  => i8,
    i32  => i8,
    i64  => i8,
    i128 => i8,
    f32  => i8,
    f64  => i8,

    u8   => i16,
    u16  => i16,
    u32  => i16,
    u64  => i16,
    u128 => i16,
    i8   => i16,
    i32  => i16,
    i64  => i16,
    i128 => i16,
    f32  => i16,
    f64  => i16,

    u8   => i32,
    u16  => i32,
    u32  => i32,
    u64  => i32,
    u128 => i32,
    i8   => i32,
    i16  => i32,
    i64  => i32,
    i128 => i32,
    f32  => i32,
    f64  => i32,
    
    u8   => i64,
    u16  => i64,
    u32  => i64,
    u64  => i64,
    u128 => i64,
    i8   => i64,
    i16  => i64,
    i32  => i64,
    i128 => i64,
    f32  => i64,
    f64  => i64,

    u8   => i128,
    u16  => i128,
    u32  => i128,
    u64  => i128,
    u128 => i128,
    i8   => i128,
    i16  => i128,
    i32  => i128,
    i64  => i128,
    f32  => i128,
    f64  => i128
}


// ---------- binverse implementations ----------

#[cfg(feature = "binverse_impls")]
impl<T, W: std::io::Write> binverse::serialize::Serialize<W> for PolyVec2<T>
where T: binverse::serialize::Serialize<W> {
    #[inline]
    fn serialize(&self, s: &mut binverse::streams::Serializer<W>) -> binverse::error::BinverseResult<()> {
        self.x.serialize(s)?;
        self.y.serialize(s)?;
        Ok(())
    }
}
#[cfg(feature = "binverse_impls")]
impl<T, R: std::io::Read> binverse::serialize::Deserialize<R> for PolyVec2<T>
where T: binverse::serialize::Deserialize<R> {
    #[inline]
    fn deserialize(d: &mut binverse::streams::Deserializer<R>) -> binverse::error::BinverseResult<Self> {
        Ok(Self {
            x: d.deserialize()?,
            y: d.deserialize()?
        })
    }
}

#[cfg(feature = "binverse_impls")]
impl<T, W: std::io::Write> binverse::serialize::Serialize<W> for PolyVec3<T>
where T: binverse::serialize::Serialize<W> {
    #[inline]
    fn serialize(&self, s: &mut binverse::streams::Serializer<W>) -> binverse::error::BinverseResult<()> {
        self.x.serialize(s)?;
        self.y.serialize(s)?;
        self.z.serialize(s)?;
        Ok(())
    }
}
#[cfg(feature = "binverse_impls")]
impl<T, R: std::io::Read> binverse::serialize::Deserialize<R> for PolyVec3<T>
where T: binverse::serialize::Deserialize<R> {
    #[inline]
    fn deserialize(d: &mut binverse::streams::Deserializer<R>) -> binverse::error::BinverseResult<Self> {
        Ok(Self {
            x: d.deserialize()?,
            y: d.deserialize()?,
            z: d.deserialize()?,
        })
    }
}

#[cfg(feature = "binverse_impls")]
impl<T, W: std::io::Write> binverse::serialize::Serialize<W> for PolyVec4<T>
where T: binverse::serialize::Serialize<W> {
    #[inline]
    fn serialize(&self, s: &mut binverse::streams::Serializer<W>) -> binverse::error::BinverseResult<()> {
        self.x.serialize(s)?;
        self.y.serialize(s)?;
        self.z.serialize(s)?;
        self.w.serialize(s)?;
        Ok(())
    }
}
#[cfg(feature = "binverse_impls")]
impl<T, R: std::io::Read> binverse::serialize::Deserialize<R> for PolyVec4<T>
where T: binverse::serialize::Deserialize<R> {
    #[inline]
    fn deserialize(d: &mut binverse::streams::Deserializer<R>) -> binverse::error::BinverseResult<Self> {
        Ok(Self {
            x: d.deserialize()?,
            y: d.deserialize()?,
            z: d.deserialize()?,
            w: d.deserialize()?,
        })
    }
}