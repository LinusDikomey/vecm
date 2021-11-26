use std::array::IntoIter;
use std::cmp;
use std::ops;
use std::fmt;

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

macro_rules! impl_ops2 {
    ($t: ident: $($op: ident = $f: ident; $assign_op: ident = $assign_f: ident),*) => {
        $(
            impl<T> $op<$t<T>> for $t<T>
            where T: $op<T> {
                type Output = $t<<T as $op>::Output>;

                #[inline]
                fn $f(self, b: $t<T>) -> Self::Output {
                    Self::Output {
                        x: self.x.$f(b.x),
                        y: self.y.$f(b.y),
                    }
                }
            }
            impl<T> $op<T> for $t<T>
            where T: $op<T> + Clone {
                type Output = $t<<T as $op>::Output>;

                #[inline]
                fn $f(self, b: T) -> Self::Output {
                    Self::Output {
                        x: self.x.$f(b.clone()),
                        y: self.y.$f(b)
                    }
                }
            }

            impl<T> $assign_op<$t<T>> for $t<T>
            where T: $assign_op<T> {
                #[inline]
                fn $assign_f(&mut self, b: $t<T>) {
                    self.x.$assign_f(b.x);
                    self.y.$assign_f(b.y);
                }
            }
            impl<T> $assign_op<T> for $t<T>
            where T: $assign_op<T> + Clone{
                #[inline]
                fn $assign_f(&mut self, b: T) {
                    self.x.$assign_f(b.clone());
                    self.y.$assign_f(b);
                }
           }
        )*
    };
}
/*

*/
macro_rules! impl_ops3 {
    ($t: ident: $($op: ident = $f: ident; $assign_op: ident = $assign_f: ident),*) => {
        $(
            impl<T> $op<$t<T>> for $t<T>
            where T: $op<T> {
                type Output = $t<<T as $op>::Output>;

                #[inline]
                fn $f(self, b: $t<T>) -> Self::Output {
                    Self::Output {
                        x: self.x.$f(b.x),
                        y: self.y.$f(b.y),
                        z: self.z.$f(b.z),
                    }
                }
            }
            impl<T> $op<T> for $t<T>
            where T: $op<T> + Clone {
                type Output = $t<<T as $op>::Output>;

                #[inline]
                fn $f(self, b: T) -> Self::Output {
                    Self::Output {
                        x: self.x.$f(b.clone()),
                        y: self.y.$f(b.clone()),
                        z: self.z.$f(b),
                    }
                }
            }

            impl<T> $assign_op<$t<T>> for $t<T>
            where T: $assign_op<T> {
                #[inline]
                fn $assign_f(&mut self, b: $t<T>) {
                    self.x.$assign_f(b.x);
                    self.y.$assign_f(b.y);
                    self.z.$assign_f(b.z);
                }
            }
            impl<T> $assign_op<T> for $t<T>
            where T: $assign_op<T> + Clone{
                #[inline]
                fn $assign_f(&mut self, b: T) {
                    self.x.$assign_f(b.clone());
                    self.y.$assign_f(b.clone());
                    self.z.$assign_f(b);
                }
            }
        )*
    };
}
macro_rules! impl_ops4 {
    ($t: ident: $($op: ident = $f: ident; $assign_op: ident = $assign_f: ident),*) => {
        $(
            impl<T> $op<$t<T>> for $t<T>
            where T: $op<T> {
                type Output = $t<<T as $op>::Output>;

                #[inline]
                fn $f(self, b: $t<T>) -> Self::Output {
                    Self::Output {
                        x: self.x.$f(b.x),
                        y: self.y.$f(b.y),
                        z: self.z.$f(b.z),
                        w: self.w.$f(b.w),
                    }
                }
            }
            impl<T> $op<T> for $t<T>
            where T: $op<T> + Clone {
                type Output = $t<<T as $op>::Output>;

                #[inline]
                fn $f(self, b: T) -> Self::Output {
                    Self::Output {
                        x: self.x.$f(b.clone()),
                        y: self.y.$f(b.clone()),
                        z: self.z.$f(b.clone()),
                        w: self.w.$f(b)
                    }
                }
            }

            impl<T> $assign_op<$t<T>> for $t<T>
            where T: $assign_op<T> {
                #[inline]
                fn $assign_f(&mut self, b: $t<T>) {
                    self.x.$assign_f(b.x);
                    self.y.$assign_f(b.y);
                    self.z.$assign_f(b.z);
                    self.w.$assign_f(b.w);
                }
            }
            impl<T> $assign_op<T> for $t<T>
            where T: $assign_op<T> + Clone{
                #[inline]
                fn $assign_f(&mut self, b: T) {
                    self.x.$assign_f(b.clone());
                    self.y.$assign_f(b.clone());
                    self.z.$assign_f(b.clone());
                    self.w.$assign_f(b.clone());
                }
            }
        )*
    };
}

use std::ops::*;

macro_rules! impl_ops {
    ($($op: ident = $f: ident; $assign_op: ident = $assign_f: ident),*) => {
        impl_ops2!{PolyVec2:
            $($op = $f; $assign_op = $assign_f),*
        }
        impl_ops3!{PolyVec3:
            $($op = $f; $assign_op = $assign_f),*
        }
        impl_ops4!{PolyVec4:
            $($op = $f; $assign_op = $assign_f),*
        }
    };
}

impl_ops!{
    Add = add; AddAssign = add_assign,
    Sub = sub; SubAssign = sub_assign,
    Mul = mul; MulAssign = mul_assign,
    Div = div; DivAssign = div_assign,
    Rem = rem; RemAssign = rem_assign,
    BitAnd = bitand; BitAndAssign = bitand_assign,
    BitOr = bitor; BitOrAssign = bitor_assign,
    BitXor = bitxor; BitXorAssign = bitxor_assign,
    Shl = shl; ShlAssign = shl_assign,
    Shr = shr; ShrAssign = shr_assign
}

// ---------- PolyVec2 ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec2<T> {
    pub x: T,
    pub y: T
}

// Component traits

impl<T> X<T> for PolyVec2<T> {
    #[inline(always)]
    fn x(&self) -> &T { &self.x }
    #[inline(always)]
    fn set_x(&mut self, x: T) { self.x = x; }
}
impl<T> Y<T> for PolyVec2<T> {
    #[inline(always)]
    fn y(&self) -> &T { &self.y }
    #[inline(always)]
    fn set_y(&mut self, y: T) { self.y = y; }
}

// Constructors

impl<T> PolyVec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> PolyVec2<T> {
        PolyVec2 {x, y}
    }
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Copy {
        Self {x: val, y: val}
    }
}

// zero/one

impl<T> PolyVec2<T>
where T : num_traits::Zero {
    #[inline]
    pub fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() }
    #[inline]
    pub fn zero() -> Self { Self {x: T::zero(), y: T::zero() }}
}

impl<T> PolyVec2<T>
where T : num_traits::One + PartialEq {
    #[inline]
    pub fn is_one(&self) -> bool { self.x.is_one() && self.y.is_one() }
    #[inline]
    pub fn one() -> Self { Self {x: T::one(), y: T::one() }}
}

// From/Into array
impl<T> From<[T; 2]> for PolyVec2<T> {
    fn from(a: [T; 2]) -> Self {
        let mut iter = IntoIter::new(a);
        Self::new(iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> Into<[T; 2]> for PolyVec2<T> {
    fn into(self) -> [T; 2] {
        [self.x, self.y]  
    }
}
// From/Into tuple
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

// magnitude mathematics

impl<T> PolyVec2<T> 
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Copy {
    #[inline]
    pub fn square_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y
    }
}

impl<T> PolyVec2<T>
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + num_traits::Float + Copy {
    #[inline]
    pub fn magnitude(&self) -> T {
        self.square_magnitude().sqrt()
    }
}

impl<T> PolyVec2<T>
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + num_traits::Float + ops::Div<T, Output = T> + cmp::PartialEq + Copy {
    #[inline]
    pub fn normalized(&self) -> Self {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            *self / m
        }
    }
    #[inline]
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl<T> PolyVec2<T>
where T : num_traits::Float {
    #[inline]
    pub fn sin(&self) -> Self { Self { x: self.x.sin(), y: self.y.sin() } }
    #[inline]
    pub fn asin(&self) -> Self { Self { x: self.x.asin(), y: self.y.asin() } }
    #[inline]
    pub fn sinh(&self) -> Self { Self { x: self.x.sinh(), y: self.y.sinh() } }
    #[inline]
    pub fn asinh(&self) -> Self { Self { x: self.x.asinh(), y: self.y.asinh() } }

    #[inline]
    pub fn cos(&self) -> Self { Self { x: self.x.cos(), y: self.y.cos() } }
    #[inline]
    pub fn acos(&self) -> Self { Self { x: self.x.acos(), y: self.y.acos() } }
    #[inline]
    pub fn cosh(&self) -> Self { Self { x: self.x.cosh(), y: self.y.cosh() } }
    #[inline]
    pub fn acosh(&self) -> Self { Self { x: self.x.acosh(), y: self.y.acosh() } }

    #[inline]
    pub fn tan(&self) -> Self { Self { x: self.x.tan(), y: self.y.tan() } }
    #[inline]
    pub fn atan(&self) -> Self { Self { x: self.x.atan(), y: self.y.atan() } }
    #[inline]
    pub fn tanh(&self) -> Self { Self { x: self.x.tanh(), y: self.y.tanh() } }
    #[inline]
    pub fn atanh(&self) -> Self { Self { x: self.x.atanh(), y: self.y.atanh() } }   
}

// angle mathematics

impl<T> PolyVec2<T> 
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + ops::Div<T, Output = T> + num_traits::float::Float + Copy {
    /// returns the angle between two vectors in radians
    #[inline]
    pub fn angle(&self, other: &Self) -> T {
        ((self.x * other.x + self.y * other.y) / (self.magnitude() * other.magnitude())).acos()
    }

    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    #[inline]
    pub fn angle_normalized(&self, other: &Self) -> T {
        (self.x * other.x + self.y * other.y).acos()
    }
}

// display

impl<T> fmt::Display for PolyVec2<T>
    where T: fmt::Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

// negate

impl<T> ops::Neg for PolyVec2<T> 
    where T: ops::Neg {
    type Output = PolyVec2<<T as ops::Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y}
    }
}

// From/Into conversions

impl<T, U> Convertible<PolyVec2<U>> for PolyVec2<T>
where T: Into<U> {
    fn convert(self) -> PolyVec2<U> {
        PolyVec2 {
            x: self.x.into(),
            y: self.y.into()
        }
    }
}

// float operations

impl<T> PolyVec2<T> 
where T : num_traits::Float {
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
        }
    }
    
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
        }
    }
}

// binverse serialization

#[cfg(feature = "binverse_impl")]
impl<T, W: std::io::Write> binverse::serialize::Serialize<W> for PolyVec2<T>
where T: binverse::serialize::Serialize<W> {
    #[inline]
    fn serialize(&self, s: &mut binverse::streams::Serializer<W>) -> binverse::error::BinverseResult<()> {
        self.x.serialize(s)?;
        self.y.serialize(s)?;
        Ok(())
    }
}
#[cfg(feature = "binverse_impl")]
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

// ---------- PolyVec3 ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

// Component traits

impl<T> X<T> for PolyVec3<T> {
    #[inline(always)]
    fn x(&self) -> &T { &self.x }
    #[inline(always)]
    fn set_x(&mut self, x: T) { self.x = x; }
}
impl<T> Y<T> for PolyVec3<T> {
    #[inline(always)]
    fn y(&self) -> &T { &self.y }
    #[inline(always)]
    fn set_y(&mut self, y: T) { self.y = y; }
}
impl<T> Z<T> for PolyVec3<T> {
    #[inline(always)]
    fn z(&self) -> &T { &self.z }
    #[inline(always)]
    fn set_z(&mut self, z: T) { self.z = z; }
}

// Constructors

impl<T> PolyVec3<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> PolyVec3<T> {
        PolyVec3 {x, y, z}
    }
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Copy {
        Self {x: val, y: val, z: val}
    }
}

// zero/one

impl<T> PolyVec3<T>
where T : num_traits::Zero {
    #[inline]
    pub fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() && self.z.is_zero() }
    #[inline]
    pub fn zero() -> Self { Self {x: T::zero(), y: T::zero(), z: T::zero() }}
}

impl<T> PolyVec3<T>
where T : num_traits::One + PartialEq {
    #[inline]
    pub fn is_one(&self) -> bool { self.x.is_one() && self.y.is_one() && self.z.is_one() }
    #[inline]
    pub fn one() -> Self { Self {x: T::one(), y: T::one(), z: T::one() }}
}

// magnitude mathematics

impl<T> PolyVec3<T> 
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Copy {
    #[inline]
    pub fn square_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl<T> PolyVec3<T>
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + num_traits::Float + Copy {
    #[inline]
    pub fn magnitude(&self) -> T {
        self.square_magnitude().sqrt()
    }
}

impl<T> PolyVec3<T>
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + num_traits::Float + ops::Div<T, Output = T> + cmp::PartialEq + num_traits::Zero + Copy {
    #[inline]
    pub fn normalized(&self) -> Self {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            *self / m
        }
    }

    #[inline]
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl<T> PolyVec3<T>
where T : num_traits::Float {
    #[inline]
    pub fn sin(&self) -> Self { Self { x: self.x.sin(), y: self.y.sin(), z: self.z.sin() } }
    #[inline]
    pub fn asin(&self) -> Self { Self { x: self.x.asin(), y: self.y.asin(), z: self.z.asin() } }
    #[inline]
    pub fn sinh(&self) -> Self { Self { x: self.x.sinh(), y: self.y.sinh(), z: self.z.sinh() } }
    #[inline]
    pub fn asinh(&self) -> Self { Self { x: self.x.asinh(), y: self.y.asinh(), z: self.z.asinh() } }

    #[inline]
    pub fn cos(&self) -> Self { Self { x: self.x.cos(), y: self.y.cos(), z: self.z.cos() } }
    #[inline]
    pub fn acos(&self) -> Self { Self { x: self.x.acos(), y: self.y.acos(), z: self.z.acos() } }
    #[inline]
    pub fn cosh(&self) -> Self { Self { x: self.x.cosh(), y: self.y.cosh(), z: self.z.cosh() } }
    #[inline]
    pub fn acosh(&self) -> Self { Self { x: self.x.acosh(), y: self.y.acosh(), z: self.z.acosh() } }

    #[inline]
    pub fn tan(&self) -> Self { Self { x: self.x.tan(), y: self.y.tan(), z: self.z.tan() } }
    #[inline]
    pub fn atan(&self) -> Self { Self { x: self.x.atan(), y: self.y.atan(), z: self.z.atan() } }
    #[inline]
    pub fn tanh(&self) -> Self { Self { x: self.x.tanh(), y: self.y.tanh(), z: self.z.tanh() } }
    #[inline]
    pub fn atanh(&self) -> Self { Self { x: self.x.atanh(), y: self.y.atanh(), z: self.z.atanh() } }   
}

// From/Into array
impl<T> From<[T; 3]> for PolyVec3<T> {
    fn from(a: [T; 3]) -> Self {
        let mut iter = IntoIter::new(a);
        Self::new(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> Into<[T; 3]> for PolyVec3<T> {
    fn into(self) -> [T; 3] {
        [self.x, self.y, self.z]
    }
}
// From/Into tuple
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

// angle mathematics

impl<T> PolyVec3<T> 
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + ops::Div<T, Output = T> + num_traits::Float + Copy {
    /// returns the angle between two vectors in radians
    #[inline]
    pub fn angle(&self, other: &Self) -> T {
        ((self.x * other.x + self.y * other.y + self.z * other.z) / (self.magnitude() * other.magnitude())).acos()
    }

    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    #[inline]
    pub fn angle_normalized(&self, other: &Self) -> T {
        (self.x * other.x + self.y * other.y + self.z * other.z).acos()
    }
}

// display

impl<T> fmt::Display for PolyVec3<T>
    where T: fmt::Display {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

// negate

impl<T> ops::Neg for PolyVec3<T> 
    where T: ops::Neg {
    type Output = PolyVec3<<T as ops::Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y, z: -self.z}
    }
}

// From/Into conversions

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

// float operations

impl<T> PolyVec3<T> 
where T : num_traits::Float {
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round()
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor()
        }
    }
    
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil()
        }
    }
}

// binverse serialization

#[cfg(feature = "binverse_impl")]
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
#[cfg(feature = "binverse_impl")]
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

// ---------- PolyVec4 ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec4<T> {
    pub x: T,
    pub y: T,
    pub z: T,
    pub w: T
}

// Component traits

impl<T> X<T> for PolyVec4<T> {
    #[inline(always)]
    fn x(&self) -> &T { &self.x }
    #[inline(always)]
    fn set_x(&mut self, x: T) { self.x = x; }
}
impl<T> Y<T> for PolyVec4<T> {
    #[inline(always)]
    fn y(&self) -> &T { &self.y }
    #[inline(always)]
    fn set_y(&mut self, y: T) { self.y = y; }
}
impl<T> Z<T> for PolyVec4<T> {
    #[inline(always)]
    fn z(&self) -> &T { &self.z }
    #[inline(always)]
    fn set_z(&mut self, z: T) { self.z = z; }
}
impl<T> W<T> for PolyVec4<T> {
    #[inline(always)]
    fn w(&self) -> &T { &self.w }
    #[inline(always)]
    fn set_w(&mut self, w: T) { self.w = w; }
}

// Constructors

impl<T> PolyVec4<T> {
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> PolyVec4<T> {
        PolyVec4 {x, y, z, w}
    }
    #[inline]
    pub const fn fill(val: T) -> Self where
    T: Copy {
        Self {x: val, y: val, z: val, w: val}
    }
}

// zero/one

impl<T> PolyVec4<T>
where T : num_traits::Zero {
    #[inline]
    pub fn is_zero(&self) -> bool { self.x.is_zero() && self.y.is_zero() && self.z.is_zero() && self.w.is_zero() }
    #[inline]
    pub fn zero() -> Self { Self {x: T::zero(), y: T::zero(), z: T::zero(), w: T::zero() }}
}

impl<T> PolyVec4<T>
where T : num_traits::One + PartialEq {
    #[inline]
    pub fn is_one(&self) -> bool { self.x.is_one() && self.y.is_one() && self.z.is_one() && self.w.is_one() }
    #[inline]
    pub fn one() -> Self { Self {x: T::one(), y: T::one(), z: T::one(), w: T::one() }}
}

// From/Into array
impl<T> From<[T; 4]> for PolyVec4<T> {
    fn from(a: [T; 4]) -> Self {
        let mut iter = IntoIter::new(a);
        Self::new(iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap(), iter.next().unwrap())
    }
}
impl<T> Into<[T; 4]> for PolyVec4<T> {
    fn into(self) -> [T; 4] {
        [self.x, self.y, self.z, self.w]
    }
}
// From/Into tuple
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

// magnitude mathematics

impl<T> PolyVec4<T> 
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + Copy {
    #[inline]
    pub fn square_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
}

impl<T> PolyVec4<T>
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + num_traits::Float + Copy {
    #[inline]
    pub fn magnitude(&self) -> T {
        self.square_magnitude().sqrt()
    }
}

impl<T> PolyVec4<T>
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + num_traits::Float + ops::Div<T, Output = T> + cmp::PartialEq + Copy {
    #[inline]
    pub fn normalized(&self) -> Self {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            *self / m
        }
    }
    #[inline]
    pub fn normalize(&mut self) {
        *self = self.normalized();
    }
}

impl<T> PolyVec4<T>
where T : num_traits::Float {
    #[inline]
    pub fn sin(&self) -> Self { Self { x: self.x.sin(), y: self.y.sin(), z: self.z.sin(), w: self.w.sin() } }
    #[inline]
    pub fn asin(&self) -> Self { Self { x: self.x.asin(), y: self.y.asin(), z: self.z.asin(), w: self.w.asin() } }
    #[inline]
    pub fn sinh(&self) -> Self { Self { x: self.x.sinh(), y: self.y.sinh(), z: self.z.sinh(), w: self.w.sinh() } }
    #[inline]
    pub fn asinh(&self) -> Self { Self { x: self.x.asinh(), y: self.y.asinh(), z: self.z.asinh(), w: self.w.asinh() } }


    #[inline]
    pub fn cos(&self) -> Self { Self { x: self.x.cos(), y: self.y.cos(), z: self.z.cos(), w: self.w.cos() } }
    #[inline]
    pub fn acos(&self) -> Self { Self { x: self.x.acos(), y: self.y.acos(), z: self.z.acos(), w: self.w.acos() } }
    #[inline]
    pub fn cosh(&self) -> Self { Self { x: self.x.cosh(), y: self.y.cosh(), z: self.z.cosh(), w: self.w.cosh() } }
    #[inline]
    pub fn acosh(&self) -> Self { Self { x: self.x.acosh(), y: self.y.acosh(), z: self.z.acosh(), w: self.w.acosh() } }

    #[inline]
    pub fn tan(&self) -> Self { Self { x: self.x.tan(), y: self.y.tan(), z: self.z.tan(), w: self.w.tan() } }
    #[inline]
    pub fn atan(&self) -> Self { Self { x: self.x.atan(), y: self.y.atan(), z: self.z.atan(), w: self.w.atan() } }
    #[inline]
    pub fn tanh(&self) -> Self { Self { x: self.x.tanh(), y: self.y.tanh(), z: self.z.tanh(), w: self.w.tanh() } }
    #[inline]
    pub fn atanh(&self) -> Self { Self { x: self.x.atanh(), y: self.y.atanh(), z: self.z.atanh(), w: self.w.atanh() } }   
}

// angle mathematics

impl<T> PolyVec4<T> 
where T : ops::Add<T, Output = T> + ops::Mul<T, Output = T> + ops::Div<T, Output = T> + num_traits::Float + Copy {
    /// returns the angle between two vectors in radians
    #[inline]
    pub fn angle(&self, other: &Self) -> T {
        ((self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w) / (self.magnitude() * other.magnitude())).acos()
    }

    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    #[inline]
    pub fn angle_normalized(&self, other: &Self) -> T {
        (self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w).acos()
    }
}

// display

impl<T> fmt::Display for PolyVec4<T>
    where T: fmt::Display {
    
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

// negate

impl<T> ops::Neg for PolyVec4<T> 
    where T: ops::Neg {
    type Output = PolyVec4<<T as ops::Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

// From/Into conversions

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

// float operations

impl<T> PolyVec4<T> 
where T : num_traits::Float {
    pub fn round(&self) -> Self {
        Self {
            x: self.x.round(),
            y: self.y.round(),
            z: self.z.round(),
            w: self.w.round()
        }
    }

    pub fn floor(&self) -> Self {
        Self {
            x: self.x.floor(),
            y: self.y.floor(),
            z: self.z.floor(),
            w: self.w.floor()
        }
    }
    
    pub fn ceil(&self) -> Self {
        Self {
            x: self.x.ceil(),
            y: self.y.ceil(),
            z: self.z.ceil(),
            w: self.w.ceil()
        }
    }
}

// binverse serialization

#[cfg(feature = "binverse_impl")]
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
#[cfg(feature = "binverse_impl")]
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

// ---------- type aliases ----------

pub type Vec2 = PolyVec2<f32>;
pub type Vec2i = PolyVec2<i32>;
pub type Vec2u = PolyVec2<u32>;

pub type Vec3 = PolyVec3<f32>;
pub type Vec3i = PolyVec3<i32>;
pub type Vec3u = PolyVec3<u32>;

pub type Vec4 = PolyVec4<f32>;
pub type Vec4i = PolyVec4<i32>;
pub type Vec4u = PolyVec4<u32>;

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

// ---------- packed ----------

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
                Self {x: vec.x, y: vec.y}
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
                Self {x: vec.x, y: vec.y, z: vec.z }
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
                Self {x: vec.x, y: vec.y, z: vec.z, w: vec.w }
            }
        }
    };
}

packed_vec!(Vec2Packed, Vec3Packed, Vec4Packed, f32);
packed_vec!(Vec2iPacked, Vec3iPacked, Vec4iPacked, i32);
packed_vec!(Vec2uPacked, Vec3uPacked, Vec4uPacked, u32);