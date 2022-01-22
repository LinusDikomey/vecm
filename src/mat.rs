use std::ops::*;

use crate::vec::*;

#[derive(Copy, Clone)]
pub struct Mat4x4 {
    data: [[f32; 4]; 4]
}
impl Mat4x4 {
    #[inline]
    pub fn new(input: [[f32; 4]; 4]) -> Mat4x4 {
        let mut data: [[f32; 4]; 4] = [[0.0; 4]; 4];
        for x in 0..4 {
            for y in 0..4 {
                data[x][y] = input[y][x];
            }
        }
        Mat4x4{data}
    }
    #[inline]
    pub fn identity() -> Mat4x4 {
        Mat4x4 {data: [
            [1.0, 0.0, 0.0, 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ]}
    }
    #[inline]
    pub fn zero() -> Mat4x4 {
        Mat4x4 {data: [
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0],
            [0.0, 0.0, 0.0, 0.0]
        ]}
    }
    #[inline]
    pub fn projection_matrix(viewport: Vec2u, near_plane: f32, far_plane: f32, fov: f32) -> Mat4x4 {
        let aspect_ratio = viewport.x as f32 / viewport.y as f32;
        let y_scale = 1.0 / (fov / 2.0).to_radians().tan();
        let x_scale = y_scale / aspect_ratio;
        let frustrum_length = far_plane - near_plane;
        Mat4x4::new([
            [x_scale, 0.0    , 0.0                                                , 0.0],
            [0.0    , y_scale, 0.0                                                , 0.0],
            [0.0    , 0.0    , -((far_plane + near_plane) / frustrum_length)      , -((2.0 * far_plane * near_plane) / frustrum_length)],
            [0.0    , 0.0    , -1.0, 0.0],
        ])
    }
    #[inline]
    pub fn ortho(left: f32, right: f32, bottom: f32, top: f32, near: f32, far: f32) -> Self {
        Mat4x4::new([
            [2.0 / (right - left), 0.0, 0.0, -((right + left) / (right - left))],
            [0.0, 2.0 / (top - bottom), 0.0, (top + bottom) / (top - bottom)],
            [0.0, 0.0, -2.0 / (far - near), (far + near) / (far - near)],
            [0.0, 0.0, 0.0, 1.0]
        ])
    }
    #[inline]
    pub fn transformation_matrix(translation: Vec3, rot: Vec3, scale: Vec3) -> Mat4x4 {
        let mut mat = Mat4x4::identity();
        mat.scale(scale);
        mat.rotate(rot);
        mat.translate(translation);
        mat
    }
    #[inline]
    pub fn view_matrix(position: Vec3, pitch: f32, yaw: f32, roll: f32) -> Mat4x4 {
        let mut mat = Mat4x4::identity();
        mat.translate(-position);
        mat.rotate(Vec3::new(-pitch, -yaw, -roll));
        mat
    }
    #[inline]
    pub fn scale(&mut self, scale: Vec3) {
        self[0][0] *= scale.x;
        self[1][1] *= scale.y;
        self[2][2] *= scale.z;
    }
    #[inline]
    pub fn rx(r: f32) -> Mat4x4 {
        [
            [1.0, 0.0 , 0.0, 0.0],
            [0.0, r.cos(), r.sin(), 0.0],
            [0.0, -r.sin(), r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ].into()
    }
    #[inline]
    pub fn ry(r: f32) -> Mat4x4 {
        [
            [r.cos(), 0.0 , -r.sin(), 0.0],
            [0.0, 1.0, 0.0, 0.0],
            [r.sin(), 0.0, r.cos(), 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ].into()
    }
    #[inline]
    pub fn rz(r: f32) -> Mat4x4 {
        [
            [r.cos(), -r.sin() , 0.0, 0.0],
            [r.sin(), r.cos(), 0.0, 0.0],
            [0.0, 0.0, 1.0, 0.0],
            [0.0, 0.0, 0.0, 1.0]
        ].into()
    }
    #[inline]
    pub fn rotate(&mut self, r: Vec3) {
        *self = Mat4x4::rx(r.x) * Mat4x4::ry(r.y) * Mat4x4::rz(r.z) * *self;
    }
    #[inline]
    pub fn translate(&mut self, t: Vec3) {
        self[3][0] += t.x;
        self[3][1] += t.y;
        self[3][2] += t.z;
    }

    #[inline]
    pub unsafe fn data_ptr<'a>(&self) -> *const f32 {
        //#[allow(unaligned_references)]
        &self.data[0][0]
    }
}

impl std::fmt::Display for Mat4x4 {
    #[inline]
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut out = "\n".to_owned();
        for y in 0..4 {
            out.push('[');
            for x in 0..4 {
                out.push_str(&format!("{}, ", self[x][y]));
            }
            out.push_str("]\n");
        }
        write!(f, "{}", out)
    }

}

impl Add<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;
    #[inline]
    fn add(self, b: Mat4x4) -> Self::Output {
        let mut c = Mat4x4::zero();

        for x in 0..4 {
            for y in 0..4 {
                c[x][y] = self[x][y] + b[x][y];
            }
        }
        c
    }
}

impl AddAssign<Mat4x4> for Mat4x4 {
    #[inline]
    fn add_assign(&mut self, b: Mat4x4) {
        for x in 0..4 {
            for y in 0..4 {
                self[x][y] += b[x][y];
            }
        }
    }
}


impl Mul<Mat4x4> for Mat4x4 {
    type Output = Mat4x4;
    #[inline]
    fn mul(self, b: Mat4x4) -> Self::Output {
        let mut c = Mat4x4::zero();

        for y in 0..4 {
            for x in 0..4 {
                for i in 0..4 {
                    c[x][y] += self[i][y] * b[x][i];
                }
            }
        }
        c
    }
}

impl From<[[f32; 4]; 4]> for Mat4x4 {
    #[inline]
    fn from(data: [[f32; 4]; 4]) -> Mat4x4 {
        Mat4x4::new(data)
    }

}

impl Index<usize> for Mat4x4 {
    type Output = [f32; 4];
    #[inline]
    fn index(&self, i: usize) -> &Self::Output {
        &self.data[i]
    }
}
impl IndexMut<usize> for Mat4x4 {
    #[inline]
    fn index_mut(&mut self, i: usize) -> &mut Self::Output {
        &mut self.data[i]
    }
}

impl AddAssign<Vec3> for Mat4x4 {
    #[inline]
    fn add_assign(&mut self, v: Vec3) {
        self[0][3] += v.x;
        self[1][3] += v.y;
        self[2][3] += v.z;
    }
}



// binverse serialization

#[cfg(feature = "binverse_impls")]
impl<W: std::io::Write> binverse::serialize::Serialize<W> for Mat4x4 {
    #[inline]
    fn serialize(&self, s: &mut binverse::streams::Serializer<W>) -> binverse::error::BinverseResult<()> {
        self.data.serialize(s)
    }
}
#[cfg(feature = "binverse_impls")]
impl<R: std::io::Read> binverse::serialize::Deserialize<R> for Mat4x4 {
    #[inline]
    fn deserialize(d: &mut binverse::streams::Deserializer<R>) -> binverse::error::BinverseResult<Self> {
        Ok(Self { data: d.deserialize()? })
    }
}