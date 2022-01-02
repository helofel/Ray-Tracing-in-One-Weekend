use crate::vector::*;

pub struct Ray <'a> {
    b: &'a Vec3,
    m: &'a Vec3,
}
    impl<'a> Ray<'a>{
        pub fn new(&mut self, b: &'a Vec3, m: &'a Vec3) -> Self {
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