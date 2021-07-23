use std::array::IntoIter;
use std::cmp;
use std::ops;
use std::fmt;

// ---------- PolyVec2 ----------

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Hash)]
pub struct PolyVec2<T> {
    pub x: T,
    pub y: T
}

// constructors

impl<T> PolyVec2<T> {
    #[inline]
    pub const fn new(x: T, y: T) -> PolyVec2<T> {
        PolyVec2 {x, y}
    }
}

impl<T> PolyVec2<T>
where T : Copy {
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Clone {
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

impl<T> ops::Add<Self> for PolyVec2<T>
    where T: ops::Add<T> {
    type Output = PolyVec2<<T as std::ops::Add>::Output>;

    #[inline]
    fn add(self, b: PolyVec2<T>) -> Self::Output {
        Self::Output {x: self.x + b.x, y: self.y + b.y}
    }
}

impl<T> ops::Sub<Self> for PolyVec2<T>
    where T: ops::Sub<T> {
    type Output = PolyVec2<<T as std::ops::Sub>::Output>;

    #[inline]
    fn sub(self, b: PolyVec2<T>) -> Self::Output {
        Self::Output {x: self.x - b.x, y: self.y - b.y}
    }
}
impl<T> ops::Mul<Self> for PolyVec2<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Mul>::Output>;
    #[inline]
    fn mul(self, b: Self) -> Self::Output {
        PolyVec2 {x: self.x * b.x, y: self.y * b.y}
    }

}
impl<T> ops::Mul<T> for PolyVec2<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Mul>::Output>;
    #[inline]
    fn mul(self, b: T) -> Self::Output {
        Self::Output {x: self.x * b, y: self.y * b}
    }
}
impl<T> ops::Div<T> for PolyVec2<T>
    where T: ops::Div<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Div>::Output>;
    #[inline]
    fn div(self, b: T) -> Self::Output {
        Self::Output {x: self.x / b, y: self.y / b}
    }   
}
impl<T> ops::Rem<T> for PolyVec2<T>
    where T: ops::Rem<T> + Copy {
    type Output = PolyVec2<<T as std::ops::Rem>::Output>;
    #[inline]
    fn rem(self, b: T) -> Self::Output {
        Self::Output {x: self.x % b, y: self.y % b}
    }   
}
impl<T> ops::MulAssign<T> for PolyVec2<T>
    where T: ops::MulAssign<T> + Copy {
    #[inline]
    fn mul_assign(&mut self, b: T) {
        self.x *= b;
        self.y *= b;
    }
}
impl<T> ops::DivAssign<T> for PolyVec2<T>
    where T: ops::DivAssign<T> + Copy {
    #[inline]
    fn div_assign(&mut self, b: T) {
        self.x /= b;
        self.y /= b;
    }
}
impl<T> ops::AddAssign<PolyVec2<T>> for PolyVec2<T>
    where T: ops::AddAssign<T> {
    #[inline]
    fn add_assign(&mut self, b: PolyVec2<T>) {
        self.x += b.x;
        self.y += b.y;
    }
}
impl<T> ops::SubAssign<PolyVec2<T>> for PolyVec2<T>
    where T: ops::SubAssign<T> {
    #[inline]
    fn sub_assign(&mut self, b: PolyVec2<T>) {
        self.x -= b.x;
        self.y -= b.y;
    }
}
impl<T> ops::Neg for PolyVec2<T> 
    where T: ops::Neg {
    type Output = PolyVec2<<T as ops::Neg>::Output>;
    #[inline]
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
    #[inline]
    pub const fn new(x: T, y: T, z: T) -> PolyVec3<T> {
        PolyVec3 {x, y, z}
    }
}

impl<T> PolyVec3<T>
where T : Copy {
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Clone {
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

impl<T> ops::Add<Self> for PolyVec3<T>
    where T: ops::Add<T> {
    type Output = PolyVec3<<T as std::ops::Add>::Output>;
    #[inline]
    fn add(self, b: PolyVec3<T>) -> Self::Output {
        Self::Output {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z}
    }
}

impl<T> ops::Sub<Self> for PolyVec3<T>
    where T: ops::Sub<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Sub>::Output>;
    #[inline]
    fn sub(self, b: PolyVec3<T>) -> Self::Output {
        Self::Output {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z}
    }
}
impl<T> ops::Mul<Self> for PolyVec3<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Mul>::Output>;
    #[inline]
    fn mul(self, b: Self) -> Self::Output {
        PolyVec3 {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z}
    }

}
impl<T> ops::Mul<T> for PolyVec3<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Mul>::Output>;
    #[inline]
    fn mul(self, b: T) -> Self::Output {
        Self::Output {x: self.x * b, y: self.y * b, z: self.z * b}
    }
}
impl<T> ops::Div<T> for PolyVec3<T>
    where T: ops::Div<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Div>::Output>;
    #[inline]
    fn div(self, b: T) -> Self::Output {
        Self::Output {x: self.x / b, y: self.y / b, z: self.z / b}
    }   
}
impl<T> ops::Rem<T> for PolyVec3<T>
    where T: ops::Rem<T> + Copy {
    type Output = PolyVec3<<T as std::ops::Rem>::Output>;
    #[inline]
    fn rem(self, b: T) -> Self::Output {
        Self::Output {x: self.x % b, y: self.y % b, z: self.z % b}
    }   
}
impl<T> ops::MulAssign<T> for PolyVec3<T>
    where T: ops::MulAssign<T> + Copy {
    #[inline]
    fn mul_assign(&mut self, b: T) {
        self.x *= b;
        self.y *= b;
        self.z *= b;
    }
}
impl<T> ops::DivAssign<T> for PolyVec3<T>
    where T: ops::DivAssign<T> + Copy {
    #[inline]
    fn div_assign(&mut self, b: T) {
        self.x /= b;
        self.y /= b;
        self.z /= b;
    }
}
impl<T> ops::AddAssign<PolyVec3<T>> for PolyVec3<T>
    where T: ops::AddAssign<T> {
    #[inline]
    fn add_assign(&mut self, b: PolyVec3<T>) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
    }
}
impl<T> ops::SubAssign<PolyVec3<T>> for PolyVec3<T>
    where T: ops::SubAssign<T> {
    #[inline]
    fn sub_assign(&mut self, b: PolyVec3<T>) {
        self.x -= b.x;
        self.y -= b.y;
        self.z -= b.z;
    }
}
impl<T> ops::Neg for PolyVec3<T> 
    where T: ops::Neg {
    type Output = PolyVec3<<T as ops::Neg>::Output>;
    #[inline]
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
    #[inline]
    pub const fn new(x: T, y: T, z: T, w: T) -> PolyVec4<T> {
        PolyVec4 {x, y, z, w}
    }
}

impl<T> PolyVec4<T>
where T : Copy {
    #[inline]
    pub const fn fill(val: T) -> Self
        where T: Clone {
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

impl<T> ops::Add<Self> for PolyVec4<T>
    where T: ops::Add<T> {
    type Output = PolyVec4<<T as std::ops::Add>::Output>;

    #[inline]
    fn add(self, b: PolyVec4<T>) -> Self::Output {
        Self::Output {x: self.x + b.x, y: self.y + b.y, z: self.z + b.z, w: self.w + b.w}
    }
}

impl<T> ops::Sub<Self> for PolyVec4<T>
    where T: ops::Sub<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Sub>::Output>;

    #[inline]
    fn sub(self, b: PolyVec4<T>) -> Self::Output {
        Self::Output {x: self.x - b.x, y: self.y - b.y, z: self.z - b.z, w: self.w - b.w}
    }
}

impl<T> ops::Mul<Self> for PolyVec4<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Mul>::Output>;
    #[inline]
    fn mul(self, b: Self) -> Self::Output {
        PolyVec4 {x: self.x * b.x, y: self.y * b.y, z: self.z * b.z, w: self.w * b.w}
    }

}
impl<T> ops::Mul<T> for PolyVec4<T>
    where T: ops::Mul<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Mul>::Output>;

    #[inline]
    fn mul(self, b: T) -> Self::Output {
        Self::Output {x: self.x * b, y: self.y * b, z: self.z * b, w: self.w * b}
    }
}
impl<T> ops::Div<T> for PolyVec4<T>
    where T: ops::Div<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Div>::Output>;

    #[inline]
    fn div(self, b: T) -> Self::Output {
        Self::Output {x: self.x / b, y: self.y / b, z: self.z / b, w: self.w / b}
    }   
}
impl<T> ops::Rem<T> for PolyVec4<T>
    where T: ops::Rem<T> + Copy {
    type Output = PolyVec4<<T as std::ops::Rem>::Output>;

    #[inline]
    fn rem(self, b: T) -> Self::Output {
        Self::Output {x: self.x % b, y: self.y % b, z: self.z % b, w: self.w % b}
    }   
}
impl<T> ops::MulAssign<T> for PolyVec4<T>
    where T: ops::MulAssign<T> + Copy {
   
    #[inline]
    fn mul_assign(&mut self, b: T) {
        self.x *= b;
        self.y *= b;
        self.z *= b;
        self.w *= b;
    }
}
impl<T> ops::DivAssign<T> for PolyVec4<T>
    where T: ops::DivAssign<T> + Copy {
   
    #[inline]
    fn div_assign(&mut self, b: T) {
        self.x /= b;
        self.y /= b;
        self.z /= b;
        self.w /= b;
    }
}
impl<T> ops::AddAssign<PolyVec4<T>> for PolyVec4<T>
    where T: ops::AddAssign<T> {
   
    #[inline]
    fn add_assign(&mut self, b: Self) {
        self.x += b.x;
        self.y += b.y;
        self.z += b.z;
        self.w += b.w;
    }
}
impl<T> ops::SubAssign<PolyVec4<T>> for PolyVec4<T>
    where T: ops::SubAssign<T> {
   
    #[inline]
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

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {x: -self.x, y: -self.y, z: -self.z, w: -self.w}
    }
}

// ---------- type definitions ----------

pub type Vec2 = PolyVec2<f32>;
pub type Vec3 = PolyVec3<f32>;
pub type Vec4 = PolyVec4<f32>;


pub type Vec2i = PolyVec2<i32>;
impl From<Vec2> for Vec2i {
    #[inline]
    fn from(f: Vec2) -> Self { Self::new(f.x as i32, f.y as i32) }
}
impl From<Vec2i> for Vec2 {
    #[inline]
    fn from(f: Vec2i) -> Self { Self::new(f.x as f32, f.y as f32) }
}

pub type Vec2u = PolyVec2<u32>;
impl From<Vec2> for Vec2u { 
    #[inline]
    fn from(f: Vec2) -> Self { Self::new(f.x as u32, f.y as u32) }
}
impl From<Vec2u> for Vec2 { 
    #[inline]
    fn from(f: Vec2u) -> Self { Self::new(f.x as f32, f.y as f32) }
}



pub type Vec3i = PolyVec3<i32>;
impl From<Vec3> for Vec3i { 
    #[inline]
    fn from(f: Vec3) -> Self { Self::new(f.x as i32, f.y as i32, f.z as i32) }
}
impl From<Vec3i> for Vec3 { 
    #[inline]
    fn from(f: Vec3i) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32) }
}

pub type Vec3u = PolyVec3<u32>;
impl From<Vec3> for Vec3u { 
    #[inline]
    fn from(f: Vec3) -> Self { Self::new(f.x as u32, f.y as u32, f.z as u32) }
}
impl From<Vec3u> for Vec3 { 
    #[inline]
    fn from(f: Vec3u) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32) }
}



pub type Vec4i = PolyVec4<i32>;
impl From<Vec4> for Vec4i { 
    #[inline]
    fn from(f: Vec4) -> Self { Self::new(f.x as i32, f.y as i32, f.z as i32, f.w as i32) }
}
impl From<Vec4i> for Vec4 { 
    #[inline]
    fn from(f: Vec4i) -> Self { Self::new(f.x as f32, f.y as f32, f.z as f32, f.w as f32) }
}

pub type Vec4u = PolyVec4<u32>;
impl From<Vec4> for Vec4u { 
    #[inline]
    fn from(f: Vec4) -> Self { Self::new(f.x as u32, f.y as u32, f.z as u32, f.w as u32) }
}
impl From<Vec4u> for Vec4 { 
    #[inline]
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