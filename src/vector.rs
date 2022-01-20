// use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};
// use std::fmt::{self, Formatter, Debug, Display};

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

pub fn add(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.x() + v.x(), u.y() + v.y() , u.z() + v.z())
}

pub fn sub(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.x() - v.x(), u.y() - v.y(), u.z() - v.z())
}

pub fn mult(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.x() * v.x(), u.y() * v.y(), u.z() * v.z())
}

pub fn scale(t: f64, v: Vec3) -> Vec3 {
    Vec3(t * v.x(), t * v.y(), t * v.z())
}
/*
pub fn mult_by_const<'a>(t: f64, v: Vec3) -> &'a Vec3 {
    &(t * v)
}
pub fn div_by_const<'a>(t: f64, v: Vec3) -> &'a Vec3 {
    &((1/t) * v)
}*/
pub fn unit_vector(v: Vec3) -> Vec3{
    Vec3(v.x() / v.length(), v.y() / v.length(), v.z() / v.length())
}

pub fn dot(u: Vec3, v: Vec3) -> f64 {
    u.x() * v.x() + u.y() * v.y() + u.z() + v.z()
}

pub fn cross(u: Vec3, v: Vec3) -> Vec3 {
    Vec3(u.y() * v.z() - u.z() * v.y(),
          u.z() * v.x() - u.x() * v.z(),
          u.x() * v.y() - u.y() * v.x())
}
