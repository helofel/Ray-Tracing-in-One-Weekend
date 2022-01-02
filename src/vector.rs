use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
use std::fmt::{self, Formatter, Debug, Display};

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self(x, y, z)
    }

    pub fn x(&self) -> f64 {
        self.0
    }

    pub fn y(&self) -> f64 {
        self.1
    }

    pub fn z(&self) -> f64 {
        self.2
    }

    pub fn length(&self) -> f64 {
        (self.0 * self.0 + self.1 * self.1 + self.2 * self.2).sqrt()
    }
}

impl Add for Vec3 {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self(self.0 + other.0, self.1 + other.1, self.2 + other.2)
    }
}

impl AddAssign for Vec3 {
    fn add_assign(&mut self, other: Self) {
        *self = Self(self.0 + other.0, self.1 + other.1, self.2 + other.2);
    }
}

impl Sub for Vec3 {
    type Output = Self;

    fn sub(self, other: Self) -> Self {
        Self(self.0 - other.0, self.1 - other.1, self.2 - other.2)
    }
}

impl SubAssign for Vec3 {
    fn sub_assign(&mut self, other: Self) {
        *self = Self(self.0 - other.0, self.1 - other.1, self.2 - other.2);
    }
}

impl Mul for Vec3 {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        Self(self.0 * other.0, self.1 * other.1, self.2 * other.2)
    }
}

impl MulAssign for Vec3 {
    fn mul_assign(&mut self, other: Self) {
        *self = Self(self.0 * other.0, self.1 * other.1, self.2 * other.2);
    }
}

impl Div for Vec3 {
    type Output = Self;

    fn div(self, other: Self) -> Self {
        Self(self.0 / other.0, self.1 / other.1, self.2 / other.2)
    }
}

impl DivAssign for Vec3 {
    fn div_assign(&mut self, other: Self) {
        *self = Self(self.0 / other.0, self.1 / other.1, self.2 / other.2);
    }
}

impl Display for Vec3 {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        write!(f, "{:.3} {:.3} {:.3}", self.0, self.1, self.2)
    }
}

pub fn add<'a>(u:&'a Vec3, v:&'a Vec3) -> Vec3 {
    Vec3(u.x() + v.x(), u.y() + v.y() , u.z() + v.z())
}

pub fn sub(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.x() - v.x(), u.y() - v.y(), u.z() - v.z())
}

pub fn mult(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.x() * v.x(), u.y() * v.y(), u.z() * v.z())
}

pub fn scale<'a>(t: f64, v: &'a Vec3) -> Vec3 {
    Vec3(t * v.x(), t * v.y(), t * v.z())
}
/*
pub fn mult_by_const<'a>(t: f64, v: Vec3) -> &'a Vec3 {
    &(t * v)
}
pub fn div_by_const<'a>(t: f64, v: Vec3) -> &'a Vec3 {
    &((1/t) * v)
}
pub fn unit_vector<'a>(v: Vec3) -> &'a Vec3{
    &(v / v.length())
}
*/
pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() + v.z() 
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.y() * v.z() - u.z() * v.y(),
          u.z() * v.x() - u.x() * v.z(),
          u.x() * v.y() - u.y() * v.x())
}
