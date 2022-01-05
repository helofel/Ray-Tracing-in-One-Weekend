use crate::vector::*;
/*
pub struct Ray <'a> {
    b: &'a Vec3,
    m: &'a Vec3,
}
    impl<'a> Ray<'a>{
        pub fn new(b: &'a Vec3, m: &'a Vec3) -> Self {
            Self {
                b: b,
                m: m
            }

        }
    }
    trait At {
        fn at(&self, x: f64) -> Vec3;
    }

    impl<'a> At for Ray<'a> {
        fn at(&self, x: f64) -> Vec3{
            add(self.b, &scale(x, self.m))
        }
    }
*/
#[derive(Copy, Clone)]

pub struct Ray{
    pub b: Vec3,
    pub m: Vec3,
}
    impl Ray{
        pub fn new(b: Vec3, m: Vec3) -> Self {
            Self {
                b: b,
                m: m
            }

        }
    }
    trait At {
        fn at(&self, x: f64) -> Vec3;
    }

    impl At for Ray {
        fn at(&self, x: f64) -> Vec3{
            add(self.b, scale(x, self.m))
        }
    }