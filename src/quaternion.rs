use crate::{Mat4x4, Vec3};

#[derive(Clone, Copy, Debug, PartialEq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Quaternion {
    pub w: f32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
}
impl Default for Quaternion {
    fn default() -> Self {
        Self {
            x: 0.0,
            y: 0.0,
            z: 0.0,
            w: 1.0,
        }
    }
}
impl Quaternion {
    /// Creates a Quaternion representing a rotation around the specified axis by an angle (in radians).
    pub fn from_angle_around_axis(axis: Vec3, angle: f32) -> Self {
        let a_2 = angle * 0.5;
        let v = axis * a_2.sin();
        Self {
            w: a_2.cos(),
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }

    /// Creates a rotation Quaternion from euler angles in 3-2-1 order.
    pub fn euler(x: f32, y: f32, z: f32) -> Self {
        // Source: https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Source_code
        let cr = (x * 0.5).cos();
        let sr = (x * 0.5).sin();
        let cp = (y * 0.5).cos();
        let sp = (y * 0.5).sin();
        let cy = (z * 0.5).cos();
        let sy = (z * 0.5).sin();

        Self {
            w: cr * cp * cy + sr * sp * sy,
            x: sr * cp * cy - cr * sp * sy,
            y: cr * sp * cy + sr * cp * sy,
            z: cr * cp * sy - sr * sp * cy,
        }
    }

    #[must_use = "only calculates the euler vector"]
    pub fn to_euler(self) -> Vec3 {
        // Source: https://en.wikipedia.org/wiki/Conversion_between_quaternions_and_Euler_angles#Source_code_2

        // roll (x-axis rotation)
        let sinr_cosp = 2.0 * (self.w * self.x + self.y * self.z);
        let cosr_cosp = 1.0 - 2.0 * (self.x * self.x + self.y * self.y);
        let x = f32::atan2(sinr_cosp, cosr_cosp);

        // pitch (y-axis rotation)
        let sinp = (1.0 + 2.0 * (self.w * self.y - self.x * self.z)).sqrt();
        let cosp = (1.0 - 2.0 * (self.w * self.y - self.x * self.z)).sqrt();
        let y = 2.0 * f32::atan2(sinp, cosp) - core::f32::consts::FRAC_PI_2;

        // yaw (z-axis rotation)
        let siny_cosp = 2.0 * (self.w * self.z + self.x * self.y);
        let cosy_cosp = 1.0 - 2.0 * (self.y * self.y + self.z * self.z);
        let z = f32::atan2(siny_cosp, cosy_cosp);

        Vec3 { x, y, z }
    }

    /// Retrieves the vector-part of the Quaternion: [x, y, z]
    #[must_use = "only retrieves the v-vector"]
    pub const fn v(self) -> Vec3 {
        Vec3::new(self.x, self.y, self.z)
    }

    /// Retrieves the equivalent rotation matrix.
    #[must_use = "only calculates the rotation matrix"]
    pub fn matrix(&self) -> Mat4x4 {
        let &Quaternion { x, y, z, w } = self;
        Mat4x4::new([
            [
                1.0 - 2.0 * y * y - 2.0 * z * z,
                2.0 * x * y - 2.0 * w * z,
                2.0 * x * z + 2.0 * w * y,
                0.0,
            ],
            [
                2.0 * x * y + 2.0 * w * z,
                1.0 - 2.0 * x * x - 2.0 * z * z,
                2.0 * y * z - 2.0 * w * x,
                0.0,
            ],
            [
                2.0 * x * z - 2.0 * w * y,
                2.0 * y * z + 2.0 * w * x,
                1.0 - 2.0 * x * x - 2.0 * y * y,
                0.0,
            ],
            [0.0, 0.0, 0.0, 1.0],
        ])
    }

    #[must_use = "only calculates the dot product"]
    pub fn dot(self, other: Self) -> f32 {
        self.w * other.w + self.x * other.x + self.y * other.y + self.z * other.z
    }

    #[must_use = "only calculates the length"]
    pub fn length(self) -> f32 {
        (self.w * self.w + self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&mut self) {
        let l = self.length();
        self.w /= l;
        self.x /= l;
        self.y /= l;
        self.z /= l;
    }

    #[must_use = "returns the new, normalized quaternion. If you want to normalize in-place, use .normalize()"]
    pub fn normalized(self) -> Self {
        let l = self.length();
        Self {
            w: self.w / l,
            x: self.x / l,
            y: self.y / l,
            z: self.z / l,
        }
    }

    #[must_use = "returns the new, conjugated quaternion"]
    pub fn conjugate(self) -> Self {
        Self {
            w: self.w,
            x: -self.x,
            y: -self.y,
            z: -self.z,
        }
    }

    pub fn slerp(self, other: Self, t: f32) -> Self {
        let o = (self.dot(other)).acos();
        ((1.0 - t).sin() * o) / o.sin() * self + (t * o).sin() / o.sin() * other
    }
}

impl core::ops::Mul<Self> for Quaternion {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        let v = rhs.v() * self.w + self.v() * rhs.w + self.v().cross(rhs.v());
        Self {
            w: self.w * rhs.w - (self.x * rhs.x + self.y * rhs.y + self.z * rhs.z),
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}
impl core::ops::Add<Self> for Quaternion {
    type Output = Self;

    fn add(mut self, rhs: Self) -> Self::Output {
        self.w += rhs.w;
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self
    }
}
impl core::ops::AddAssign<Self> for Quaternion {
    fn add_assign(&mut self, rhs: Self) {
        self.w += rhs.w;
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}
impl core::ops::Sub<Self> for Quaternion {
    type Output = Self;

    fn sub(mut self, rhs: Self) -> Self::Output {
        self.w -= rhs.w;
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self
    }
}
impl core::ops::SubAssign<Self> for Quaternion {
    fn sub_assign(&mut self, rhs: Self) {
        self.w -= rhs.w;
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}
impl core::ops::Mul<f32> for Quaternion {
    type Output = Self;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.w *= rhs;
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self
    }
}
impl core::ops::Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, rhs: Quaternion) -> Self::Output {
        rhs * self
    }
}
impl core::ops::MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, rhs: f32) {
        self.w *= rhs;
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}
impl core::ops::Mul<Quaternion> for Vec3 {
    type Output = Vec3;
    /// NOTE: Assumes a unit-length Quaternion.
    fn mul(self, rhs: Quaternion) -> Self::Output {
        #[cfg(debug_assertions)]
        {
            // Checks for approximate unit-length since chained calculations might make Quaternions
            // slightly non-normalized. This should catch cases where the Quaternion isn't
            // normalized at all.
            let len = rhs.length();
            debug_assert!(
                (0.9..=1.1).contains(&len),
                "Quaternion-Vector multiplication should be used with a normalized Quaternion.",
            );
            debug_assert!(len <= 1.1);
        }
        let p = Quaternion {
            w: 0.0,
            x: self.x,
            y: self.y,
            z: self.z,
        };
        // This is where the normalized Quaternion is assumed since an inversion would be needed
        // instead of a conjugation otherwise.
        (rhs * p * rhs.conjugate()).v()
    }
}

#[cfg(feature = "binverse")]
impl<W: std::io::Write> binverse::serialize::Serialize<W> for Quaternion {
    #[inline]
    fn serialize(
        &self,
        s: &mut binverse::streams::Serializer<W>,
    ) -> binverse::error::BinverseResult<()> {
        self.w.serialize(s)?;
        self.x.serialize(s)?;
        self.y.serialize(s)?;
        self.z.serialize(s)
    }
}
#[cfg(feature = "binverse")]
impl<R: std::io::Read> binverse::serialize::Deserialize<R> for Quaternion {
    #[inline]
    fn deserialize(
        d: &mut binverse::streams::Deserializer<R>,
    ) -> binverse::error::BinverseResult<Self> {
        Ok(Self {
            w: d.deserialize()?,
            x: d.deserialize()?,
            y: d.deserialize()?,
            z: d.deserialize()?,
        })
    }
}
