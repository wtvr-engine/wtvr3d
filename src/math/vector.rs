use std::ops::{Add, Sub, Mul, AddAssign, MulAssign, SubAssign};

pub struct Vector3 {
    pub x : f32,
    pub y : f32,
    pub z : f32,
}

impl Vector3 {
    pub fn zero() -> Vector3 {
        Vector3 {x : 0.0, y : 0.0, z : 0.0}
    }
    pub fn equals(&self, v : &Vector3) -> bool {
        self.x == v.x && self.y == v.y && self.z == v.z
    }
    pub fn is_zero(&self) -> bool{
        self.x == 0.0 && self.y == 0.0 && self.z == 0.0
    }
    pub fn length(&self) -> f32{
        (self.x*self.x + self.y*self.y + self.z*self.z).sqrt()
    }
    pub fn normal(&self) -> bool{
        self.length() == 1.0
    }
    pub fn dot_product(&self, v : &Vector3) -> f32{
        &self.x*v.x + &self.y*v.y + &self.z*v.z
    }
    pub fn cross_product(&self, v : &Vector3) -> Vector3{
        Vector3 {
            x : self.y*v.z - self.z*v.y,
            y : self.z*v.x - self.x*v.z,
            z : self.x*v.y - self.y*v.x,
        }
    }
}

impl Add for Vector3 {
    type Output = Vector3;

    fn add(self, v: Vector3) -> Vector3 {
        Vector3 {x : self.x + v.x, y: self.y + v.y, z : self.z + v.z}
    }
}

impl Sub for Vector3 {
    type Output = Vector3;

    fn sub(self, v: Vector3) -> Vector3 {
        Vector3 {x : self.x - v.x, y: self.y - v.y, z : self.z - v.z}
    }
}

impl Mul<f32> for Vector3 {
    type Output = Vector3;

    fn mul(self, f : f32) -> Vector3 {
        Vector3 {x : self.x * f, y: self.y * f, z : self.z * f}
    }
}

impl Mul<Vector3> for f32 {
    type Output = Vector3;

    fn mul(self, vec : Vector3) -> Vector3 {
        Vector3 {x : self * vec.x, y: self * vec.y, z : self * vec.z}
    }
}

impl AddAssign for Vector3 {
    fn add_assign(&mut self, v: Vector3){
        self.x += v.x;
        self.y += v.y;
        self.z += v.z;
    }
}

impl SubAssign for Vector3 {
    fn sub_assign(&mut self, v: Vector3){
        self.x -= v.x;
        self.y -= v.y;
        self.z -= v.z;
    }
}

impl MulAssign<f32> for Vector3 {
    fn mul_assign(&mut self, f : f32){
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}

// ################################# //
// ########### TESTS ############### //
// ################################# //

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn zero() {
        let vec = Vector3::zero();
        assert_eq!(vec.x, 0.0);
        assert_eq!(vec.y, 0.0);
        assert_eq!(vec.z, 0.0);
    }

    #[test]
    fn equals(){
        let v1 = Vector3 { x : 1.0, y : 3.56, z : 6.3};
        let mut v2 = Vector3 { x : 1.0, y : 1.56 + 2.0, z : 9.3 - 3.0};
        assert!(v1.equals(&v2));
        assert!(v2.equals(&v1));
        v2.x += 2.0;
        assert!(!v2.equals(&v1));
        assert!(!v1.equals(&v2));
    }

    #[test]
    fn is_zero(){
        let mut vec = Vector3::zero();
        assert!(vec.is_zero());
        vec.x = 1.0;
        assert!(!vec.is_zero());
    }

    #[test]
    fn length(){
        let mut vec = Vector3 {x : 1.0, y: 0.0, z : 0.0};
        assert_eq!(vec.length(),1.0);
        assert!(vec.normal());
        vec.y = 4.0;
        vec.x = 3.0;
        assert_eq!(vec.length(), 25.0_f32.sqrt());
    }

    #[test]
    fn dot_product(){
        let v1 = Vector3 { x : 1.0, y : 3.0, z : 2.0};
        let v2 = Vector3 { x : 0.0, y : 4.0, z : 5.0};
        assert_eq!(v1.dot_product(&v2),22.0);
    }

    #[test]
    fn cross_product(){
        let v1 = Vector3 { x : 1.0, y : 3.0, z : 2.0};
        let v2 = Vector3 { x : 0.0, y : 4.0, z : 5.0};
        let result = Vector3{ x : 7.0, y : -5.0, z : 4.0};
        assert!(v1.cross_product(&v2).equals(&result));
    }

    #[test]
    fn add(){
        let v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let v2 = Vector3{ x: 2.0, y : -5.0, z : -2.0};
        let result = Vector3{ x: 3.0, y : -2.0, z : -6.0};
        assert!((v1 + v2).equals(&result));
    }

    #[test]
    fn add_assign(){
        let mut v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let v2 = Vector3{ x: 2.0, y : -5.0, z : -2.0};
        v1 += v2;
        let result = Vector3{ x: 3.0, y : -2.0, z : -6.0};
        assert!(v1.equals(&result))
    }

    #[test]
    fn sub(){
        let v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let v2 = Vector3{ x: 2.0, y : -5.0, z : -2.0};
        let result = Vector3{ x: -1.0, y : 8.0, z : -2.0};
        assert!((v1 - v2).equals(&result));
    }

    #[test]
    fn sub_assign(){
        let mut v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let v2 = Vector3{ x: 2.0, y : -5.0, z : -2.0};
        v1 -= v2;
        let result = Vector3{ x: -1.0, y : 8.0, z : -2.0};
        assert!(v1.equals(&result));
    }

    #[test]
    fn mul(){
        let v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let f = 5.0;
        let result = Vector3{ x: 5.0, y : 15.0, z : -20.0};
        assert!((v1 * f).equals(&result));
    }

    #[test]
    fn mul_reverse(){
        let v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let f = 5.0;
        let result = Vector3{ x: 5.0, y : 15.0, z : -20.0};
        assert!((f * v1).equals(&result));
    }

    #[test]
    fn mul_assign(){
        let mut v1 = Vector3{ x: 1.0, y : 3.0, z : -4.0};
        let f = 5.0;
        v1 *= f;
        let result = Vector3{ x: 5.0, y : 15.0, z : -20.0};
        assert!(v1.equals(&result));
    }
}
