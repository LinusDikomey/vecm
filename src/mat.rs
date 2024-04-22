//! NOTE: matrices are in column-major order.

use crate::{PolyVec3, Quaternion, Vec2, Vec3};
use core::{
    mem::MaybeUninit,
    ops::{Add, AddAssign, Index, IndexMut, Mul},
};
use num_traits::{One, Zero};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
#[repr(C)]
pub struct Mat<T, const M: usize, const N: usize> {
    data: [[T; M]; N],
}

impl<T, const M: usize, const N: usize> Mat<T, M, N> {
    #[inline]
    pub fn new(input: [[T; N]; M]) -> Self {
        Mat { data: input }.transpose()
    }

    #[inline]
    pub const fn data_ptr(&self) -> *const T {
        &self.data[0][0]
    }

    pub fn transpose(self) -> Mat<T, N, M> {
        let mut data: MaybeUninit<[[T; N]; M]> = MaybeUninit::uninit();
        for (x, row) in self.data.into_iter().enumerate() {
            for (y, item) in row.into_iter().enumerate() {
                // this can be rewritten safely with `MaybeUninit::transpose` once it is stabilized
                let row_ptr: *mut [T; N] = data.as_mut_ptr().cast();
                let elem_ptr: *mut T = unsafe { row_ptr.add(y) }.cast();
                let ptr = unsafe { elem_ptr.add(x) };
                unsafe { core::ptr::write(ptr, item) };
            }
        }
        Mat {
            data: unsafe { data.assume_init() },
        }
    }
}

impl<T, const M: usize, const N: usize> Index<usize> for Mat<T, M, N> {
    type Output = [T; M];
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
impl<T, const M: usize, const N: usize> IndexMut<usize> for Mat<T, M, N> {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

impl<T: Zero + Copy, const M: usize, const N: usize> Mat<T, M, N> {
    #[inline]
    pub fn zero() -> Self {
        Self {
            data: [[T::zero(); M]; N],
        }
    }
}

// NxN (quadratic) matrices

impl<T: Zero + Copy + One, const N: usize> Mat<T, N, N> {
    #[inline]
    pub fn identity() -> Self {
        let mut data = [[T::zero(); N]; N];
        for i in 0..N {
            data[i][i] = T::one();
        }
        Self { data }
    }

    pub fn pow(&self, n: usize) -> Self {
        let mut res = Mat::identity();
        for _ in 0..n {
            res = *self * res;
        }
        res
    }
}

impl Mat<f32, 4, 4> {
    #[inline]
    pub fn projection_matrix(viewport: Vec2, near_plane: f32, far_plane: f32, fov: f32) -> Self {
        let aspect_ratio = viewport.x / viewport.y;
        let y_scale = 1.0 / (fov / 2.0).to_radians().tan();
        let x_scale = y_scale / aspect_ratio;
        let frustrum_length = far_plane - near_plane;
        Self::new([
            [x_scale, 0.0, 0.0, 0.0],
            [0.0, y_scale, 0.0, 0.0],
            [
                0.0,
                0.0,
                -((far_plane + near_plane) / frustrum_length),
                -((2.0 * far_plane * near_plane) / frustrum_length),
            ],
            [0.0, 0.0, -1.0, 0.0],
        ])
    }
    #[inline]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Self::new([
            [
                2.0 / (right - left),
                0.0,
                0.0,
                -((right + left) / (right - left)),
            ],
            [
                0.0,
                2.0 / (top - bottom),
                0.0,
                (top + bottom) / (top - bottom),
            ],
            [0.0, 0.0, -2.0 / (far - near), (far + near) / (far - near)],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[inline]
    pub fn transformation_matrix(translation: Vec3, rotation: Quaternion, scale: Vec3) -> Self {
        let mut mat = Self::identity();
        mat.scale(scale);
        mat = rotation.matrix() * mat;
        mat.translate(translation);
        mat
    }
    #[inline]
    pub fn view_matrix(position: Vec3, rotation: Quaternion) -> Self {
        let mut mat = Self::identity();
        mat.translate(-position);
        rotation.conjugate().matrix() * mat
    }
    #[inline]
    pub fn scale(&mut self, scale: Vec3) {
        self[0][0] *= scale.x;
        self[1][1] *= scale.y;
        self[2][2] *= scale.z;
    }
    #[inline]
    pub fn rx(r: f32) -> Self {
        Self::new([
            [1.0, 0.0, 0.0, 0.0],
            [0.0, r.cos(), r.sin(), 0.0],
            [0.0, -r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[inline]
    pub fn ry(r: f32) -> Self {
        Self::new([
            [r.cos(), 0.0, -r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[inline]
    pub fn rz(r: f32) -> Self {
        Self::new([
            [r.cos(), -r.sin(), 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }
    #[inline]
    pub fn rotate(&mut self, r: Vec3) {
        *self = Self::rx(r.x) * Self::ry(r.y) * Self::rz(r.z) * *self;
    }
    #[inline]
    pub fn translate(&mut self, t: Vec3) {
        self[3][0] += t.x;
        self[3][1] += t.y;
        self[3][2] += t.z;
    }
}

impl<T: core::fmt::Display, const M: usize, const N: usize> core::fmt::Display for Mat<T, M, N> {
    #[inline]
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        writeln!(f)?;
        for m in 0..M {
            write!(f, "[")?;
            for n in 0..N {
                if n != 0 {
                    write!(f, ", ")?;
                }
                write!(f, "{}", self[n][m])?;
            }
            writeln!(f, "],")?;
        }
        Ok(())
    }
}

impl<T: Zero + Copy, const M: usize, const N: usize> Add<Self> for Mat<T, M, N> {
    type Output = Self;
    #[inline]
    fn add(self, b: Self) -> Self::Output {
        let mut c = Self::zero();

        for m in 0..M {
            for n in 0..N {
                c[n][m] = self[n][m] + b[n][m];
            }
        }
        c
    }
}

impl<T: AddAssign + Clone, const N: usize, const M: usize> AddAssign<Self> for Mat<T, N, M> {
    #[inline]
    fn add_assign(&mut self, b: Self) {
        for x in 0..M {
            for y in 0..N {
                self[x][y] += b[x][y].clone();
            }
        }
    }
}

impl<T: Zero + Copy + Add + Mul<Output = T>, const L: usize, const M: usize, const N: usize>
    Mul<Mat<T, M, N>> for Mat<T, L, M>
{
    type Output = Mat<T, L, N>;
    #[inline]
    fn mul(self, b: Mat<T, M, N>) -> Self::Output {
        let mut c = Mat::zero();

        for n in 0..N {
            for l in 0..L {
                for i in 0..M {
                    c[n][l] = c[n][l] + self[i][l] * b[n][i];
                }
            }
        }
        c
    }
}

impl<T: AddAssign> AddAssign<PolyVec3<T>> for Mat<T, 4, 4> {
    #[inline]
    fn add_assign(&mut self, v: PolyVec3<T>) {
        self[0][3] += v.x;
        self[1][3] += v.y;
        self[2][3] += v.z;
    }
}

// binverse serialization

#[cfg(feature = "binverse")]
impl<W, T, const M: usize, const N: usize> binverse::serialize::Serialize<W> for Mat<T, M, N>
where
    W: std::io::Write,
    T: binverse::serialize::Serialize<W>,
{
    #[inline]
    fn serialize(
        &self,
        s: &mut binverse::streams::Serializer<W>,
    ) -> binverse::error::BinverseResult<()> {
        self.data.serialize(s)
    }
}
#[cfg(feature = "binverse")]
impl<R, T, const M: usize, const N: usize> binverse::serialize::Deserialize<R> for Mat<T, M, N>
where
    R: std::io::Read,
    T: binverse::serialize::Deserialize<R>,
{
    #[inline]
    fn deserialize(
        d: &mut binverse::streams::Deserializer<R>,
    ) -> binverse::error::BinverseResult<Self> {
        Ok(Self {
            data: d.deserialize()?,
        })
    }
}

// Serde can't serialize arrays of arbitrary size so we have to implement each size seperately
// using the following macro.
// For now there are only quadratic matrices up to size 5.
// That should hopefully cover the most common use cases.

#[cfg(feature = "serde")]
macro_rules! serde_mat {
    ($($n: literal, $m: literal)*) => {
        $(
            impl<T: serde::Serialize> serde::Serialize for Mat<T, $n, $m> {
                fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where
                    S: serde::Serializer
                {
                    self.data.serialize(serializer)
                }
            }
            impl<'de, T: serde::Deserialize<'de>> serde::Deserialize<'de> for Mat<T, $n, $m> {
                fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
                where
                    D: serde::Deserializer<'de>
                {
                    Ok(Self {
                        data: serde::Deserialize::deserialize(deserializer)?,
                    })
                }
            }
        )*
    };
}

#[cfg(feature = "serde")]
serde_mat! {
    1, 1
    2, 2
    3, 3
    4, 4
    5, 5
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn matrix_multiplication() {
        let a = Mat::new([[1, 2, 3], [5, 6, 7]]);

        let b = Mat::new([[2, 3, 4, 5], [6, 7, 8, 9], [10, 11, 12, 13]]);

        assert_eq!(a * b, Mat::new([[44, 50, 56, 62], [116, 134, 152, 170],]));
    }

    #[test]
    fn power() {
        let a = Mat::new([[1, 2, 3], [4, 5, 6], [7, 8, 9]]);
        assert_eq!(a.pow(0), Mat::identity());
        assert_eq!(a.pow(1), a);
        assert_eq!(
            a.pow(2),
            Mat::new([[30, 36, 42], [66, 81, 96], [102, 126, 150],])
        );
        assert_eq!(
            a.pow(3),
            Mat::new([[468, 576, 684], [1062, 1305, 1548], [1656, 2034, 2412],])
        );
    }

    #[test]
    fn transpose() {
        let a = Mat::new([[1, 2, 3], [4, 5, 6]]);

        assert_eq!(a.transpose(), Mat::new([[1, 4], [2, 5], [3, 6]]));
    }

    #[test]
    fn correct_drop_counts() {
        use core::sync::atomic::{AtomicU8, Ordering};
        const ORDERING: Ordering = Ordering::Relaxed;
        static DROP_COUNT: AtomicU8 = AtomicU8::new(0);
        assert_eq!(DROP_COUNT.load(ORDERING), 0);

        struct CountDrop;
        impl Drop for CountDrop {
            fn drop(&mut self) {
                DROP_COUNT.fetch_add(1, ORDERING);
            }
        }

        assert_eq!(DROP_COUNT.load(ORDERING), 0);

        let mat1 = Mat::new([[CountDrop, CountDrop]]);
        let mat2 = mat1.transpose();

        assert_eq!(DROP_COUNT.load(ORDERING), 0);
        drop(mat2);
        assert_eq!(DROP_COUNT.load(ORDERING), 2);
    }
}
