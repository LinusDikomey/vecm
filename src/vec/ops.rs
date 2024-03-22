use core::ops::{
    Add, AddAssign, BitAnd, BitAndAssign, BitOr, BitOrAssign, BitXor, BitXorAssign, Div, DivAssign,
    Mul, MulAssign, Neg, Rem, RemAssign, Shl, ShlAssign, Shr, ShrAssign, Sub, SubAssign,
};

use super::{PolyVec2, PolyVec3, PolyVec4};

// ---------- Basic math operations ----------

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

impl_ops! {
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

// ---------- magnitude mathematics ----------

impl<T> PolyVec2<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
{
    #[inline]
    pub fn square_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y
    }
    #[inline]
    pub fn magnitude(&self) -> T
    where
        T: num_traits::Float,
    {
        self.square_magnitude().sqrt()
    }
    #[inline]
    pub fn normalized(&self) -> Self
    where
        T: num_traits::Float + Div<T, Output = T>,
    {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            *self / m
        }
    }
    #[inline]
    pub fn normalize(&mut self)
    where
        T: num_traits::Float + Div<T, Output = T>,
    {
        *self = self.normalized();
    }
}

impl<T> PolyVec3<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
{
    #[inline]
    pub fn square_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    #[inline]
    pub fn magnitude(&self) -> T
    where
        T: num_traits::Float,
    {
        self.square_magnitude().sqrt()
    }
    #[inline]
    pub fn normalized(&self) -> Self
    where
        T: num_traits::Float + Div<T, Output = T>,
    {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            *self / m
        }
    }
    #[inline]
    pub fn normalize(&mut self)
    where
        T: num_traits::Float + Div<T, Output = T>,
    {
        *self = self.normalized();
    }
}

impl<T> PolyVec4<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Copy,
{
    #[inline]
    pub fn square_magnitude(&self) -> T {
        self.x * self.x + self.y * self.y + self.z * self.z + self.w * self.w
    }
    #[inline]
    pub fn magnitude(&self) -> T
    where
        T: num_traits::Float,
    {
        self.square_magnitude().sqrt()
    }
    #[inline]
    pub fn normalized(&self) -> Self
    where
        T: num_traits::Float + Div<T, Output = T>,
    {
        let m = self.magnitude();
        if m.is_zero() {
            Self::zero()
        } else {
            *self / m
        }
    }
    #[inline]
    pub fn normalize(&mut self)
    where
        T: num_traits::Float + Div<T, Output = T>,
    {
        *self = self.normalized();
    }
}

// ---------- Trigonometry/float operations ----------

macro_rules! trig {
    ($($f: ident)*) => {
        impl<T: num_traits::Float> PolyVec2<T> {
            $(
                #[inline]
                pub fn $f(&self) -> Self { Self { x: self.x.$f(), y: self.y.$f() } }
            )*
        }
        impl<T: num_traits::Float> PolyVec3<T> {
            $(
                #[inline]
                pub fn $f(&self) -> Self { Self { x: self.x.$f(), y: self.y.$f(), z: self.z.$f() } }
            )*
        }
        impl<T: num_traits::Float> PolyVec4<T> {
            $(
                #[inline]
                pub fn $f(&self) -> Self { Self { x: self.x.$f(), y: self.y.$f(), z: self.z.$f(), w: self.w.$f() } }
            )*
        }
    };
}

trig! {
    sin asin sinh asinh
    cos acos cosh acosh
    tan atan tanh atanh
    round floor ceil
}

// ---------- angle mathematics ----------

impl<T> PolyVec2<T>
where
    T: Add<T, Output = T>
        + Mul<T, Output = T>
        + Div<T, Output = T>
        + num_traits::float::Float
        + Copy,
{
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

impl<T> PolyVec3<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + num_traits::Float + Copy,
{
    /// returns the angle between two vectors in radians
    #[inline]
    pub fn angle(&self, other: &Self) -> T {
        ((self.x * other.x + self.y * other.y + self.z * other.z)
            / (self.magnitude() * other.magnitude()))
        .acos()
    }
    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    #[inline]
    pub fn angle_normalized(&self, other: &Self) -> T {
        (self.x * other.x + self.y * other.y + self.z * other.z).acos()
    }
}

impl<T> PolyVec4<T>
where
    T: Add<T, Output = T> + Mul<T, Output = T> + Div<T, Output = T> + num_traits::Float + Copy,
{
    /// returns the angle between two vectors in radians
    #[inline]
    pub fn angle(&self, other: &Self) -> T {
        ((self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w)
            / (self.magnitude() * other.magnitude()))
        .acos()
    }
    /// returns the angle between two vectors in degrees and assumes that both vectors have a length of 1 to simplify the calculation
    #[inline]
    pub fn angle_normalized(&self, other: &Self) -> T {
        (self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w).acos()
    }
}

// ---------- negate ----------

impl<T> Neg for PolyVec2<T>
where
    T: Neg,
{
    type Output = PolyVec2<<T as Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
        }
    }
}

impl<T> Neg for PolyVec3<T>
where
    T: Neg,
{
    type Output = PolyVec3<<T as Neg>::Output>;
    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }
}

impl<T> Neg for PolyVec4<T>
where
    T: Neg,
{
    type Output = PolyVec4<<T as Neg>::Output>;

    #[inline]
    fn neg(self) -> Self::Output {
        Self::Output {
            x: -self.x,
            y: -self.y,
            z: -self.z,
            w: -self.w,
        }
    }
}

// cross
impl<T> PolyVec3<T>
where
    T: Mul<T, Output = T> + Sub<T, Output = T> + Copy,
{
    #[inline]
    pub fn cross(self, other: Self) -> Self {
        Self {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }
}
