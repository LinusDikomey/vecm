use std::ops;
use std::fmt;

// ---------- PolyVec2 ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec2<T> {
    pub x: T,
    pub y: T
}

impl<T> PolyVec2<T> {

    pub const fn new(x: T, y: T) -> PolyVec2<T> {
        PolyVec2 {x, y}
    }

    pub fn fill(val: T) -> Self
        where T : Clone {
        Self {x: val.clone(), y: val}
    }
}

impl<T> fmt::Display for PolyVec2<T>
    where T: fmt::Display {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}]", self.x, self.y)
    }
}

impl<T> From<(T, T)> for PolyVec2<T> {
    fn from(other: (T, T)) -> Self {
        PolyVec2 {x: other.0, y: other.1}
    }
}

impl<T> ops::Add<Self> for PolyVec2<T>
    where T: ops::Add<T> {
    type Output = PolyVec2<<T as std::ops::Add>::Output>;

    fn add(self, b: PolyVec2<T>) -> Self::Output {
        Self::Output {x: self.x + b.x, y: self.y + b.y}
    }
}

impl<T> ops::Sub<Self> for PolyVec2<T>
    where T: ops::Sub<T> {
    type Output = PolyVec2<<T as std::ops::Sub>::Output>;

    fn sub(self, b: PolyVec2<T>) -> Self::Output {
        Self::Output {x: self.x - b.x, y: self.y - b.y}
    }
}
impl<T> ops::Mul<Self> for PolyVec2<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Mul>::Output>;
    fn mul(self, b: Self) -> Self::Output {
        PolyVec2 {x: self.x * b.x, y: self.y * b.y}
    }

}
impl<T> ops::Mul<T> for PolyVec2<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Mul>::Output>;

    fn mul(self, b: T) -> Self::Output {
        Self::Output {x: self.x * b, y: self.y * b}
    }
}
impl<T> ops::Div<T> for PolyVec2<T>
    where T: ops::Div<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Div>::Output>;

    fn div(self, b: T) -> Self::Output {
        Self::Output {x: self.x / b, y: self.y / b}
    }   
}
impl<T> ops::Rem<T> for PolyVec2<T>
    where T: ops::Rem<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Rem>::Output>;

    fn rem(self, b: T) -> Self::Output {
        Self::Output {x: self.x % b, y: self.y % b}
    }   
}
impl<T> ops::MulAssign<T> for PolyVec2<T>
    where T: ops::MulAssign<T> + Copy {
   
    fn mul_assign(&mut self, b: T) {
        self.x *= b;
        self.y *= b;
    }
}
impl<T> ops::DivAssign<T> for PolyVec2<T>
    where T: ops::DivAssign<T> + Copy {
   
    fn div_assign(&mut self, b: T) {
        self.x /= b;
        self.y /= b;
    }
}
impl<T> ops::AddAssign<PolyVec2<T>> for PolyVec2<T>
    where T: ops::AddAssign<T> {
   
    fn add_assign(&mut self, b: PolyVec2<T>) {
        self.x += b.x;
        self.y += b.y;
    }
}
impl<T> ops::SubAssign<PolyVec2<T>> for PolyVec2<T>
    where T: ops::SubAssign<T> {
   
    fn sub_assign(&mut self, b: PolyVec2<T>) {
        self.x -= b.x;
        self.y -= b.y;
    }
}
impl<T> ops::Neg for PolyVec2<T> 
    where T: ops::Neg {
    type Output = PolyVec2<<T as ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y}
    }
}

// ---------- PolyVec3 ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec3<T> {
    pub x: T,
    pub y: T,
    pub z: T
}

impl<T> PolyVec3<T> {

    pub const fn new(x: T, y: T, z: T) -> PolyVec3<T> {
        PolyVec3 {x, y, z}
    }

    pub fn fill(val: T) -> Self
        where T: Clone {
        Self {x: val.clone(), y: val.clone(), z: val}
    }
}

impl<T> fmt::Display for PolyVec3<T>
    where T: fmt::Display {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}]", self.x, self.y, self.z)
    }
}

impl<T> From<(T, T, T)> for PolyVec3<T> {
    fn from(other: (T, T, T)) -> Self {
        PolyVec3 {x: other.0, y: other.1, z: other.2}
    }
}

impl<T> ops::Add<Self> for PolyVec3<T>
    where T: ops::Add<T> {
    type Output = PolyVec3<<T as std::ops::Add>::Output>;

    fn add(self, b: PolyVec3<T>) -> Self::Output {
        Self::Output {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
}

impl<T> ops::Sub<Self> for PolyVec3<T>
    where T: ops::Sub<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Sub>::Output>;

    fn sub(self, b: PolyVec3<T>) -> Self::Output {
        Self::Output {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
}
impl<T> ops::Mul<Self> for PolyVec3<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Mul>::Output>;
    fn mul(self, b: Self) -> Self::Output {
        PolyVec3 {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
    }

}
impl<T> ops::Mul<T> for PolyVec3<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Mul>::Output>;

    fn mul(self, b: T) -> Self::Output {
        Self::Output {x: self.x * b, y: self.y * b, z: self.z * b}
    }
}
impl<T> ops::Div<T> for PolyVec3<T>
    where T: ops::Div<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Div>::Output>;

    fn div(self, b: T) -> Self::Output {
        Self::Output {x: self.x / b, y: self.y / b, z: self.z / b}
    }   
}
impl<T> ops::Rem<T> for PolyVec3<T>
    where T: ops::Rem<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Rem>::Output>;

    fn rem(self, b: T) -> Self::Output {
        Self::Output {x: self.x % b, y: self.y % b, z: self.z % b}
    }   
}
impl<T> ops::MulAssign<T> for PolyVec3<T>
    where T: ops::MulAssign<T> + Copy {
   
    fn mul_assign(&mut self, b: T) {
        self.x *= b;
        self.y *= b;
        self.z *= b;
    }
}
impl<T> ops::DivAssign<T> for PolyVec3<T>
    where T: ops::DivAssign<T> + Copy {
   
    fn div_assign(&mut self, b: T) {
        self.x /= b;
        self.y /= b;
        self.z /= b;
    }
}
impl<T> ops::AddAssign<PolyVec3<T>> for PolyVec3<T>
    where T: ops::AddAssign<T> {
   
    fn add_assign(&mut self, b: PolyVec3<T>) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }
}
impl<T> ops::SubAssign<PolyVec3<T>> for PolyVec3<T>
    where T: ops::SubAssign<T> {
   
    fn sub_assign(&mut self, b: PolyVec3<T>) {
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
    }
}
impl<T> ops::Neg for PolyVec3<T> 
    where T: ops::Neg {
    type Output = PolyVec3<<T as ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y, z: -self.z}
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

impl<T> PolyVec4<T> {

    pub const fn new(x: T, y: T, z: T, w: T) -> PolyVec4<T> {
        PolyVec4 {x, y, z, w}
    }

    pub fn fill(val: T) -> Self
        where T: Clone {
        Self {x: val.clone(), y: val.clone(), z: val.clone(), w: val}
    }
}

impl<T> fmt::Display for PolyVec4<T>
    where T: fmt::Display {
    
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "[{}, {}, {}, {}]", self.x, self.y, self.z, self.w)
    }
}

impl<T> From<(T, T, T, T)> for PolyVec4<T> {
    fn from(other: (T, T, T, T)) -> Self {
        PolyVec4 {x: other.0, y: other.1, z: other.2, w: other.3}
    }
}

impl<T> ops::Add<Self> for PolyVec4<T>
    where T: ops::Add<T> {
    type Output = PolyVec4<<T as std::ops::Add>::Output>;

    fn add(self, b: PolyVec4<T>) -> Self::Output {
        Self::Output {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z, w: self.w + b.w}
    }
}

impl<T> ops::Sub<Self> for PolyVec4<T>
    where T: ops::Sub<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Sub>::Output>;

    fn sub(self, b: PolyVec4<T>) -> Self::Output {
        Self::Output {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z, w: self.w - b.w}
    }
}

impl<T> ops::Mul<Self> for PolyVec4<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Mul>::Output>;
    fn mul(self, b: Self) -> Self::Output {
        PolyVec4 {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z, w: self.w * b.w}
    }

}
impl<T> ops::Mul<T> for PolyVec4<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Mul>::Output>;

    fn mul(self, b: T) -> Self::Output {
        Self::Output {x: self.x * b, y: self.y * b, z: self.z * b, w: self.w * b}
    }
}
impl<T> ops::Div<T> for PolyVec4<T>
    where T: ops::Div<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Div>::Output>;

    fn div(self, b: T) -> Self::Output {
        Self::Output {x: self.x / b, y: self.y / b, z: self.z / b, w: self.w / b}
    }   
}
impl<T> ops::Rem<T> for PolyVec4<T>
    where T: ops::Rem<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Rem>::Output>;

    fn rem(self, b: T) -> Self::Output {
        Self::Output {x: self.x % b, y: self.y % b, z: self.z % b, w: self.w % b}
    }   
}
impl<T> ops::MulAssign<T> for PolyVec4<T>
    where T: ops::MulAssign<T> + Copy {
   
    fn mul_assign(&mut self, b: T) {
        self.x *= b;
        self.y *= b;
        self.z *= b;
        self.w *= b;
    }
}
impl<T> ops::DivAssign<T> for PolyVec4<T>
    where T: ops::DivAssign<T> + Copy {
   
    fn div_assign(&mut self, b: T) {
        self.x /= b;
        self.y /= b;
        self.z /= b;
        self.w /= b;
    }
}
impl<T> ops::AddAssign<PolyVec4<T>> for PolyVec4<T>
    where T: ops::AddAssign<T> {
   
    fn add_assign(&mut self, b: Self) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
        self.w += b.w;
    }
}
impl<T> ops::SubAssign<PolyVec4<T>> for PolyVec4<T>
    where T: ops::SubAssign<T> {
   
    fn sub_assign(&mut self, b: Self) {
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
        self.w -= b.w;
    }
}
impl<T> ops::Neg for PolyVec4<T> 
    where T: ops::Neg {
    type Output = PolyVec4<<T as ops::Neg>::Output>;

    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

// ---------- type definitions ----------

pub type Vec2 = PolyVec2<f32>;

impl Vec2 {

    pub const ZERO: Self = Self {x: 0.0, y: 0.0};
    pub const ONE: Self = Self {x: 1.0, y: 1.0};

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y).sqrt()
    }
    pub fn square_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y
    }

    pub fn normalize(&mut self) {
        let m = self.magnitude();
        if m != 0.0 {
            *self /= m;
        }
    }

    /// returns the angle between two vectors in radians
    pub fn angle(&self, other: &Self) -> f32 {
        ((self.x * other.x + self.y * other.y) / (self.magnitude() * other.magnitude())).acos()
    }

    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    pub fn angle_normalized(&self, other: &Self) -> f32 {
        (self.x * other.x + self.y * other.y).acos()
    }
}

pub type Vec3 = PolyVec3<f32>;

impl Vec3 {

    pub const ZERO: Self = Self {x: 0.0, y: 0.0, z: 0.0};
    pub const ONE: Self = Self {x: 1.0, y: 1.0, z: 1.0};
    
    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }
    pub fn square_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }

    pub fn normalize(mut self) -> Self {
        let m = self.magnitude();
        if m != 0.0 {
            self /= m;
        }
        self
    }

    /// returns the angle between two vectors in degrees
    pub fn angle(&self, other: &Self) -> f32 {
        ((self.x * other.x + self.y * other.y + self.z * other.z) / (self.magnitude() * other.magnitude())).acos()
    }

    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    pub fn angle_normalized(&self, other: &Self) -> f32 {
        (self.x * other.x + self.y * other.y + self.z * other.z).acos()
    }

    pub fn dot(&self, other: &Self) -> f32 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn lerp(a: &Vec3, b: &Vec3, t: f32) -> Vec3 {
        Vec3 {
            x: crate::lerp(a.x, b.x, t),
            y: crate::lerp(a.y, b.y, t),
            z: crate::lerp(a.z, b.z, t),
        }
    }
}

pub type Vec4 = PolyVec4<f32>;

impl Vec4 {

    pub const ZERO: Self = Self {x: 0.0, y: 0.0, z: 0.0, w: 0.0};
    pub const ONE: Self = Self {x: 1.0, y: 1.0, z: 1.0, w: 0.0};

    pub fn magnitude(&self) -> f32 {
        (self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w).sqrt()
    }
    pub fn square_magnitude(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }

    pub fn normalize(&mut self) {
        let m = self.magnitude();
        if m != 0.0 {
            *self /= m;
        }
    }

    /// returns the angle between two vectors in radians
    pub fn angle(&self, other: &Self) -> f32 {
        ((self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w) / (self.magnitude() * other.magnitude())).acos()
    }

    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    pub fn angle_normalized(&self, other: &Self) -> f32 {
        (self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w).acos()
    }

}

pub type Vec2i = PolyVec2<i32>;
impl From<Vec2> for Vec2i { 
    fn from(f: Vec2) -> Self { Self::new(f.x as i32, f.y as i32) }
}
impl From<Vec2i> for Vec2 { 
    fn from(f: Vec2i) -> Self { Self::new(f.x as f32, f.y as f32) }
}

pub type Vec2u = PolyVec2<u32>;
impl From<Vec2> for Vec2u { 
    fn from(f: Vec2) -> Self { Self::new(f.x as u32, f.y as u32) }
}
impl From<Vec2u> for Vec2 { 
    fn from(f: Vec2u) -> Self { Self::new(f.x as f32, f.y as f32) }
}

pub type Vec3i = PolyVec3<i32>;
impl Vec3i {
    pub const ZERO: Vec3i = Vec3i {x: 0, y: 0, z: 0};
    pub fn length(&self) -> f32 {
        ((self.x * self.x + self.y * self.y + self.z * self.z) as f32).sqrt()
    }
    pub fn square_magnitude(&self) -> i32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}
impl From<Vec3> for Vec3i { 
    fn from(f: Vec3) -> Self { Self::new(f.x as i32, f.y as i32, f.z as i32) }
}
impl From<Vec3i> for Vec3 { 
    fn from(f: Vec3i) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32) }
}

pub type Vec3u = PolyVec3<u32>;
impl From<Vec3> for Vec3u { 
    fn from(f: Vec3) -> Self { Self::new(f.x as u32, f.y as u32, f.z as u32) }
}
impl From<Vec3u> for Vec3 { 
    fn from(f: Vec3u) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32) }
}

pub type Vec4i = PolyVec4<i32>;
impl From<Vec4> for Vec4i { 
    fn from(f: Vec4) -> Self { Self::new(f.x as i32, f.y as i32, f.z as i32, f.w as i32) }
}
impl From<Vec4i> for Vec4 { 
    fn from(f: Vec4i) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32, f.w as f32) }
}

pub type Vec4u = PolyVec4<u32>;
impl From<Vec4> for Vec4u { 
    fn from(f: Vec4) -> Self { Self::new(f.x as u32, f.y as u32, f.z as u32, f.w as u32) }
}
impl From<Vec4u> for Vec4 { 
    fn from(f: Vec4u) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32, f.w as f32) }
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
            fn from(vec: PolyVec4<$t>) -> Self {
                Self {x: vec.x, y: vec.y, z: vec.z, w: vec.w }
            }
        }
    };
}

packed_vec!(Vec2Packed, Vec3Packed, Vec4Packed, f32);
packed_vec!(Vec2iPacked, Vec3iPacked, Vec4iPacked, i32);
packed_vec!(Vec2uPacked, Vec3uPacked, Vec4uPacked, u32);