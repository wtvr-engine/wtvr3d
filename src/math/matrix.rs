//! # Matrix
//! 4x4 matrix implementation for 3D math

use std::ops::{Index, IndexMut, Mul, MulAssign};
use super::quaternion::Quaternion;
use super::vector::Vector3;
use std::fmt;

/// # Matrix4
/// 4x4 matrix implementation for 3D math, as a 16 element f32 array.
#[derive(Clone)]
pub struct Matrix4 {

    /// Internal data of the matrix as a 16 element array
    data : [f32; 16]
}

impl Matrix4 {

    /// Creates a matrix from its translation, rotation and scale
    ///
    /// # Examples
    ///
    /// ```
    /// let t = Vector3 { x: 1.0, y : 2.0, z : -1.0};
    /// let r = Quaternion::identity();
    /// let s =  Vector3 { x: 5.0, y : 5.0, z : 5.0};
    /// let mat = Matrix4::new(&t, &r, &s);
    /// ```
    pub fn new(translation : &Vector3, rotation : &Quaternion, scale : &Vector3) -> Matrix4 {
        let mut res = Matrix4::from_quaternion(rotation);
        res.scale(scale);
        res[12] = translation.x;
        res[13] = translation.y;
        res[14] = translation.z;
        res
    }

    /// Returns the identity matrix
    ///
    /// # Examples
    ///
    /// ```
    /// let id = Matrix4::identity();
    /// ```
    pub fn identity() -> Matrix4 {
        let mut res = Matrix4 { data : [0.0; 16]};
        for i in 0..4 {
            res[i + i*4] = 1.0;
        }
        res
    }

    /// Returns a zero-filled matrix.
    ///
    /// # Examples
    ///
    /// ```
    /// let zero = Matrix4::zero();
    /// ```
    pub fn zero() -> Matrix4 {
        Matrix4 { data : [0.0; 16]}
    }

    /// Returns the matrix for perspective camera, given its parameters
    ///
    /// # Examples
    ///
    /// ```
    /// let camera_mat = Matrix4::perspective(1.0, 16.0/10.0, 10.0, 500.0);
    /// ```
    pub fn perspective(fov : f32, aspect_ratio : f32, nearz : f32, farz : f32) -> Matrix4 {
        let f = 1.0 / (fov/2.0).tan();
        let nf = 1.0 / (nearz - farz);
        Matrix4 {
            data : [
            f/ aspect_ratio, 0.0 , 0.0 , 0.0,
            0.0, f, 0.0, 0.0,
            0.0, 0.0, (farz + nearz) * nf, -1.0,
            0.0, 0.0, 2.0 * farz * nearz * nf, 0.0,
            ]
        }
    }

    /// Tests whether a matrix is equal to another one.
    ///
    /// # Examples
    ///
    /// ```
    /// let mut mat1 = Matrix4::identity();
    /// let mut mat2 = Matrix4::identity();
    /// mat1[3] = 3.0;
    /// assert!(!mat1.equals(&mat2));
    /// mat2[3] = 3.0;
    /// assert!(mat1.equals(&mat2))
    /// ```
    fn equals(&self, mat : &Matrix4) -> bool {
        let mut res = true;
        for i in 0..16 {
            if  self[i] != mat[i] {
                res = false;
                break;
            }
        }
        res
    }

    /// Computes the determinant of the matrix
    ///
    /// # Examples
    ///
    /// ```
    /// let mat = Matrix4::identity();
    /// assert_eq!(mat.determinant(),1.0);
    /// ```
    pub fn determinant(&self) -> f32 {
        let s = self.sub_determinants();
        s[0]*s[11] - s[1]*s[10] + s[2]*s[9] + s[3]*s[8] - s[4]*s[7] + s[5]*s[6]
    }

    /// Comutes the inverse of a matrix
    ///
    /// # Examples
    ///
    /// ```
    /// let mat = Matrix4::identity();
    /// ```
    pub fn inverse(&self) -> Matrix4 {
        let s = self.sub_determinants();
        let det = s[0]*s[11] - s[1]*s[10] + s[2]*s[9] + s[3]*s[8] - s[4]*s[7] + s[5]*s[6];
        if det == 0.0 {
            panic!("Given matrix is not invertible!");
        }
        Matrix4 {
            data : [
                (self[5]*s[11] - self[6]*s[10] + self[7]*s[9])  * det,
                (self[2]*s[10] - self[1]*s[11] - self[3]*s[9])  * det,
                (self[13]*s[5] - self[14]*s[4] + self[15]*s[3]) * det,
                (self[10]*s[4] - self[9]*s[5]  - self[11]*s[3]) * det,
                (self[6]*s[8]  - self[4]*s[11] - self[7]*s[7])  * det,
                (self[0]*s[11] - self[2]*s[8]  + self[3]*s[7])  * det,
                (self[14]*s[2] - self[12]*s[5] - self[15]*s[1]) * det,
                (self[8]*s[5]  - self[10]*s[2] + self[11]*s[1]) * det,
                (self[4]*s[10] - self[9]*s[8]  + self[7]*s[6])  * det,
                (self[1]*s[8]  - self[0]*s[10] - self[3]*s[6])  * det,
                (self[12]*s[4] - self[13]*s[2] + self[15]*s[0]) * det,
                (self[9]*s[2]  - self[8]*s[4]  - self[11]*s[0]) * det,
                (self[5]*s[7]  - self[6]*s[9]  - self[7]*s[6])  * det,
                (self[0]*s[9]  - self[1]*s[7]  + self[2]*s[6])  * det,
                (self[13]*s[1] - self[12]*s[3] - self[14]*s[0]) * det,
                (self[8]*s[3]  - self[9]*s[1]  + self[10]*s[0]) * det,
            ]
        }
    }

    /// Creates a matrix from a normalized quaternion. (Made to be pre-multiplied)
    ///
    /// # Examples
    /// ```
    /// let q = Matrix4::from_quaternion(Quaternion::identity());
    /// ```
    pub fn from_quaternion(q : &Quaternion) -> Matrix4 {
        let (x2,y2,z2) = (q.x + q.x, q.y + q.y, q.z + q.z);
        let (xx,xy,xz) = (q.x * x2, q.x * y2, q.x * z2);
        let (yy,yz,zz) = (q.y * y2, q.y * z2, q.z * z2);
        let (wx,wy,wz) = (q.w * x2, q.w * y2, q.w * z2);
        Matrix4 {
            data : [
            1.0 - (yy + zz),
            xy + wz,
            xz - wy,
            0.0,
            xy - wz,
            1.0 - (xx + zz),
            yz + wx,
            0.0,
            xz + wy,
            yz - wx,
            1.0 - (xx + yy),
            0.0,
            0.0,0.0,0.0,1.0
            ]
        }
    }

    /// Scales a matrix with a vector (internal use)
    fn scale(&mut self, v : &Vector3) {
        for i in 0..12 {
            self[i] *= match i {
                0...3 => v.x,
                4...7 => v.y,
                _ => v.z
            }
        }
    }

    /// Utility function to help with calculating determinant.
    fn sub_determinants(&self) -> [f32; 12] {
        [
            self[0]*self[5] - self[1]*self[4],
            self[0]*self[6] - self[2]*self[4],
            self[0]*self[7] - self[3]*self[4],
            self[1]*self[6] - self[2]*self[5],
            self[1]*self[7] - self[3]*self[5],
            self[2]*self[7] - self[3]*self[6],
            self[8]*self[13] - self[9]*self[12],
            self[8]*self[14] - self[10]*self[12],
            self[8]*self[15] - self[11]*self[12],
            self[9]*self[14] - self[10]*self[13],
            self[9]*self[15] - self[11]*self[13],
            self[10]*self[15] - self[11]*self[14]
        ]
    }
}

impl Index<usize> for Matrix4 {
    type Output = f32;
    fn index(&self, index : usize) -> &f32 {
        &self.data[index]
    }
}

impl IndexMut<usize> for Matrix4 {
    fn index_mut(& mut self, index : usize) -> &mut f32 {
        &mut self.data[index]
    }
}

impl Index<(usize,usize)> for Matrix4 {
    type Output = f32;
    fn index(&self, index : (usize,usize)) -> &f32 {
        &self.data[index.0 + 4*index.1]
    }
}

impl IndexMut<(usize,usize)> for Matrix4 {
    fn index_mut(& mut self, index : (usize,usize)) -> &mut f32 {
        &mut self.data[index.0 + 4*index.1]
    }
}

impl Mul<f32> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, f : f32) -> Matrix4 {
        let mut res = self.clone();
        for i in 0..16 {
            res[i] *= f
        }
        res
    }
}

impl<'a> Mul<f32> for &'a Matrix4 {
    type Output = Matrix4;

    fn mul(self, f : f32) -> Matrix4 {
        let mut res = self.clone();
        for i in 0..16 {
            res[i] *= f
        }
        res
    }
}

impl Mul<Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(self, mat : Matrix4) -> Matrix4 {
        let mut res = mat.clone();
        for i in 0..16 {
            res[i] *= self
        }
        res
    }
}

impl<'a> Mul<&'a Matrix4> for f32 {
    type Output = Matrix4;

    fn mul(self, mat : &'a Matrix4) -> Matrix4 {
        let mut res = mat.clone();
        for i in 0..16 {
            res[i] *= self
        }
        res
    }
}

impl MulAssign<f32> for Matrix4 {
    fn mul_assign(&mut self, f : f32){
        for i in 0..16 {
            self[i] *= f
        }
    }
}

impl Mul<Matrix4> for Matrix4 {
    type Output = Matrix4;

    fn mul(self, mat : Matrix4) -> Matrix4 {
        let mut res = Matrix4::zero();
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    res[col + row * 4] += mat[col + row * 4] * self[col + i * 4];
                }
            }
        }
        res
    }
}

impl<'a> Mul<&'a Matrix4> for &'a Matrix4 {
    type Output = Matrix4;

    fn mul(self, mat : &'a Matrix4) -> Matrix4 {
        let mut res = Matrix4::zero();
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    res[col + row * 4] += mat[col + row * 4] * self[col + i * 4];
                }
            }
        }
        res
    }
}

impl MulAssign<Matrix4> for Matrix4 {
    fn mul_assign(&mut self, mat : Matrix4){
        let mut res = Matrix4::zero();
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    res[col + row * 4] += mat[col + row * 4] * self[col + i * 4];
                }
            }
        }
        self.clone_from(&res);
    }
}

impl<'a> MulAssign<&'a Matrix4> for Matrix4 {
    fn mul_assign(&mut self, mat : &'a Matrix4){
        let mut res = Matrix4::zero();
        for row in 0..4 {
            for col in 0..4 {
                for i in 0..4 {
                    res[col + row * 4] += mat[col + row * 4] * self[col + i * 4];
                }
            }
        }
        self.clone_from(&res);
    }
}

impl fmt::Debug for Matrix4 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Matrix4\n[{},{},{},{}]\n[{},{},{},{}]\n[{},{},{},{}]\n[{},{},{},{}]", self[0],self[1],self[2],self[3],self[4],self[5],self[6],self[7],self[8],self[9],self[10],self[11],self[12],self[13],self[14],self[15])
    }
}

// ################################# //
// ########### TESTS ############### //
// ################################# //

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn identity() {
        let id = Matrix4::identity();
        for i in 0..16 {
            assert_eq!(id[i] ,if i % 5  == 0 { 1.0 } else { 0.0 });
        }
    }

    #[test]
    fn zero() {
        let zero = Matrix4::zero();
        for i in 0..16 {
            assert_eq!(zero[i] , 0.0 );
        }
    }

    #[test]
    fn equals() {
        let mut mat1 = Matrix4::identity();
        let mut mat2 = Matrix4::identity();
        mat1[3] = 3.0;
        assert!(!mat1.equals(&mat2));
        mat2[3] = 3.0;
        assert!(mat1.equals(&mat2));
    }

    #[test]
    fn index() {
        let mut id = Matrix4::identity();
        id[13] = 5.0;
        id[(2,1)] = 7.0;
        assert_eq!(id[13],5.0);
        assert_eq!(id[(2,1)],7.0);
        assert_eq!(id[13],id[(1,3)]);
        assert_eq!(id[6],id[(2,1)]);
    }

    #[test]
    fn from_quaternion() {
        let approx = 0.000002;
        let q1 = Quaternion::identity();
        let m1 = Matrix4::from_quaternion(&q1);
        let m1bis = Matrix4::identity();
        assert!(m1.equals(&m1bis));
        let mut q2 = Quaternion { w : 1.0 , x : 1.0, y : 2.0, z : 1.0};
        q2.normalize();
        let m2 = Matrix4::from_quaternion(&q2);
        println!("Matrix {:?}",&m2);
        let m2bisData = [-0.428571, 0.857142, -0.285715,0.0, 0.285715 , 0.428572, 0.857142, 0.0, 0.857142, 0.285715, -0.428572,0.0,0.0,0.0,0.0,1.0];
        for i in 0..16 {
            assert!((m2[i] - m2bisData[i]).abs() <= approx, "at {}, wanted : {}, got : {}",i,m2bisData[i],m2[i]);
        }
    }

    #[test]
    fn determinant() {
        let mut mat = Matrix4::identity();
        assert_eq!(mat.determinant(),1.0);
        mat *= 2.0;
        assert_eq!(mat.determinant(),16.0);
        mat[(1,0)] = 1.0;
        mat[(3,1)] = 5.0;
        mat[(0,3)] = -8.0;
        assert_eq!(mat.determinant(),-64.0);
    }

    #[test]
    fn inverse() {
        let mat = Matrix4::identity();
        let mut id = mat.inverse();
        assert_eq!(id.determinant(), 1.0);
        for i in 0..16 {
            assert_eq!(id[i] ,if i % 5  == 0 { 1.0 } else { 0.0 });
        }
        id[3] = 1.0;
        id[6] = 2.0;
        id[12] = 2.0;
        let inv = id.inverse();
        let res = Matrix4 { data : [-1.0, 0.0, 0.0, 1.0, 0.0, 1.0, -2.0, 0.0, 0.0, 0.0, 1.0, 0.0, 2.0, 0.0, 0.0, -1.0] };
        assert!(inv.equals(&res));
    }
}
