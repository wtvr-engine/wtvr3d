//! # Quaternion
//! Quaternion implementation with useful methods

use super::vector::Vector3;
use std::ops::{Mul,MulAssign};

#[derive(Clone)]
pub struct Quaternion{
    
    /// x coordinate of the quaternion
    pub x : f32,

    /// y coordinate of the quaternion
    pub y : f32,

    /// z coordinate of the quaternion
    pub z : f32,

    /// w coordinate of the quaternion
    pub w : f32,
}

impl Quaternion {
    /// Returns the identity quaternion
    ///
    /// # Examples
    ///
    /// ```
    /// let id = Quaternion::identity();
    /// ```
    pub fn identity() -> Quaternion {
        Quaternion { x : 0.0, y : 0.0, z : 0.0, w : 1.0 }
    }

    /// Returns a zero-filled Quaternion
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = Quaternion::zero();
    /// ```
    pub fn zero() -> Quaternion {
        Quaternion { x : 0.0, y : 0.0, z : 0.0, w : 0.0 }
    }

    /// Returns a quaternion from an axis and an angle (in radians)
    ///
    /// # Examples
    ///
    /// ```
    /// let quat = Quaternion::from_axis_angle(Vector3::identity(), math::PI/3.0)
    /// ```
    pub fn from_axis_angle(axis : Vector3, angle : f32) -> Quaternion {
        let sin_half = (angle/2.0).sin();
        let mut res = Quaternion {
            x : sin_half * axis.x,
            y : sin_half * axis.y,
            z : sin_half * axis.z,
            w : (angle/2.0).cos()
        };
        res *= 1.0/res.magnitude();
        res
    }

    /// Returns a quaternion obtained by converting a set of Euler angles
    ///
    /// # Examples
    /// ```
    /// let quat = Quaternion::from_euler(Vector3 { x: 0.0, y : PI/2.0, z : 0.0 });
    /// ```
    pub fn from_euler(v : &Vector3) -> Quaternion {
        let (x,y,z) = (v.x/2.0,v.y/2.0,v.z/2.0);
        let (c1,c2,c3) = (x.cos(),y.cos(),z.cos());
        let (s1,s2,s3) = (x.sin(),y.sin(),z.sin());
        Quaternion {
            x : s1 * c2 * c3 + c1 * s2 * s3,
            y : c1 * s2 * c3 - s1 * c2 * s3,
            z : c1 * c2 * s3 + s1 * s2 * c3,
            w : c1 * c2 * c3 - s1 * s2 * s3
        }
    }

    /// Tests whether two Quaternions are equal.
    ///
    /// # Examples
    ///
    /// ```
    /// let quat1 = Quaternion::from_axis_angle(Vector3(1.0,1.0,1.0), PI);
    /// let quat2 = Quaternion::from_axis_angle(Vector3(1.0,1.0,1.0), PI);
    /// assert!(quat1.equals(&quat2));
    /// ```
    pub fn equals(&self, quat : &Quaternion) -> bool {
        self.x == quat.x && self.y == quat.y && self.w == quat.w && self.z == quat.z
    }

    /// Normalizes a quaternion so that its magnitude is one.
    ///
    /// # Examples
    ///
    /// ```
    /// let quat = Quaternion::from_axis_angle(Vector { x: 1.0, y : 1.0, z : 0.0}, 1.0/(2.0*PI));
    /// quat.normalize();
    /// ```
    pub fn normalize(&mut self) {
        *self *= 1.0/self.magnitude();
    }

    /// Return a new Vector3, rotated from the input by the quaternion's rotation.
    ///
    /// # Examples
    ///
    /// ```
    /// let quat = Quaternion::from_axis_angle(Vector { x: 1.0, y : 1.0, z : 0.0}, 1.0/(2.0*PI));
    /// let new_vec = quat.rotate(Vector3{x: 0.0, y : 1.0, z : 0.0};
    /// ```
    pub fn rotate(&self, vec : Vector3) -> Vector3 {
        let vec2 = Vector3 { x : self.x, y : self.y, z : self.z };
        let mut ret = &vec2 * 2.0* vec.dot_product(&vec2);
        ret += &vec * (self.w * self.w - vec2.dot_product(&vec2));
        ret += vec2.cross_product(&vec);
        ret *= 2.0 *self.w;
        ret
    }

    /// Performs a spherical interpolation between 2 Quaternions
    ///
    /// # Examples
    ///
    /// ```
    /// let quat1 = Quaternion::identity();
    /// let quat2 = Quaternion { x: 1.0, y : 0.5, z : 0.0, w : 1.0};
    /// let quat3 = quat1.slerp(quat2, 0.4);
    /// ```
    pub fn slerp(&self,quat : Quaternion, t : f32) -> Quaternion {
        let (ax, ay, az, aw) = (self.x, self.y, self.z, self.w);
        let (mut bx, mut by, mut bz, mut bw) = (quat.x, quat.y, quat.z, quat.w);
        let mut cosom = ax * bx + ay * by + az * bz + aw * bw;
        let (mut scale0, mut scale1) = (1.0 - t,t);
        if cosom < 0.0 {
            cosom = - cosom;
            bx = -bx;
            by = -by;
            bz = -bz;
            bw = -bw;
        }
        if (1.0 - cosom) > 0.000001 {
            let omega = cosom.acos();
            let sinom = omega.sin();
            scale0 = ((1.0 - t)* omega).sin() / sinom;
            scale1 = (t * omega).sin()/sinom;
        }
        Quaternion {
            x : scale0 * ax + scale1 * bx,
            y : scale0 * ay + scale1 * by,
            z : scale0 * az + scale1 * bz,
            w : scale0 * aw + scale1 * bw,
        }
    }

    /// Returns the magnitude (or vector length) of the quaternion.
    fn magnitude(&self) -> f32 {
        (self.x*self.x + self.y*self.y + self.z*self.z + self.w*self.w).sqrt()
    }
}

impl Mul<f32> for Quaternion {
    type Output = Quaternion;

    fn mul(self, f : f32) -> Quaternion {
        &self*f
    }
}

impl<'a> Mul<f32> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, f : f32) -> Quaternion {
        let mut res = self.clone();
        res.x *= f;
        res.y *= f;
        res.z *= f;
        res.w *= f;
        res
    }
}

impl Mul<Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, quat : Quaternion) -> Quaternion {
        self*&quat
    }
}

impl<'a> Mul<&'a Quaternion> for f32 {
    type Output = Quaternion;

    fn mul(self, quat : &'a Quaternion) -> Quaternion {
        let mut res = quat.clone();
        res.x *= self;
        res.y *= self;
        res.z *= self;
        res.w *= self;
        res
    }
}

impl MulAssign<f32> for Quaternion {
    fn mul_assign(&mut self, f : f32) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
        self.w *= f;
    }
}

impl<'a> Mul<&'a Quaternion> for &'a Quaternion {
    type Output = Quaternion;

    fn mul(self, quat : &'a Quaternion ) -> Quaternion {
        Quaternion {
            x : self.x * quat.w + self.w * quat.x + self.y * quat.z - self.z * quat.y,
            y : self.y * quat.w + self.w * quat.y + self.z * quat.x - self.x * quat.z,
            z : self.z * quat.w + self.w * quat.z + self.x * quat.y - self.y * quat.x,
            w : self.w * quat.w - self.x * quat.x - self.y * quat.y - self.z * quat.z,
        }
    }
}

impl Mul<Quaternion> for Quaternion {
    type Output = Quaternion;

    fn mul(self, quat : Quaternion ) -> Quaternion {
        &self * &quat
    }
}

impl MulAssign<Quaternion> for Quaternion {
    fn mul_assign(&mut self, quat : Quaternion ) {
        self.x = self.x * quat.w + self.w * quat.x + self.y * quat.z - self.z * quat.y;
        self.y = self.y * quat.w + self.w * quat.y + self.z * quat.x - self.x * quat.z;
        self.z = self.z * quat.w + self.w * quat.z + self.x * quat.y - self.y * quat.x;
        self.w = self.w * quat.w - self.x * quat.x - self.y * quat.y - self.z * quat.z;
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::super::vector::Vector3;
    use super::super::PI;

    #[test]
    fn identity() {
        let id = Quaternion::identity();
        assert_eq!(id.x, 0.0);
        assert_eq!(id.y, 0.0);
        assert_eq!(id.z, 0.0);
        assert_eq!(id.w, 1.0);
    }

    #[test]
    fn zero() {
        let id = Quaternion::zero();
        assert_eq!(id.x, 0.0);
        assert_eq!(id.y, 0.0);
        assert_eq!(id.z, 0.0);
        assert_eq!(id.w, 0.0);
    }

    #[test]
    fn from_axis_angle() {
        let quat = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        println!("{}, {}, {}, {}",quat.x,quat.y,quat.z,quat.w);
        assert_eq!(quat.x,0.7071068);
        assert_eq!(quat.w,0.7071068);
        assert_eq!(quat.z,0.0);
        assert_eq!(quat.y,0.0);
    }
    #[test]
    fn equals() {
        let quat1 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        let mut quat2 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        assert!(quat1.equals(&quat2));
        quat2.z = 2.0;
        assert!(!quat2.equals(&quat1));
    }

    #[test]
    fn mul() {
        let quat1 = Quaternion::identity();
        let quat2 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        let quat3 = &quat1 * &quat2;
        assert!(quat2.equals(&quat3));
        let quat4 = Quaternion::from_axis_angle(Vector3 { x : 1.0, y : 0.0, z : 0.0 }, PI/2.0);
        let quat5 = &quat2 * &quat4;
        assert!((1.0 - quat5.x).abs() < 0.0001);
        assert_eq!(quat5.y, 0.0);
        assert_eq!(quat5.z, 0.0);
        assert_eq!(quat5.w, 0.0);
    }

    #[test]
    fn normalize() {
        let mut quat2 = Quaternion { x : 4.0, y : 0.0, z : 0.0, w : 0.0 };
        quat2.normalize();
        assert_eq!(quat2.x,1.0);
        assert_eq!(quat2.y,0.0);
        assert_eq!(quat2.z,0.0);
        assert_eq!(quat2.w,0.0);
    }

    #[test]
    fn rotate() {
        let quat = Quaternion::from_axis_angle(Vector3 { x: 0.0, y : 0.0, z : 1.0}, 0.5*PI);
        let new_vec = quat.rotate(Vector3{x: 1.0, y : 0.0, z : 0.0});
        assert!(new_vec.x.abs()  < 0.0001);
        assert!((1.0 - new_vec.y).abs()  < 0.0001);
        assert!(new_vec.z.abs()  < 0.0001);
    }
}
