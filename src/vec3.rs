
struct vec3 {
    vector: Vec<f64>,
}

impl vec3{
    // pub fn new(&mut self) -> Self {
    //     Self {
    //         self.vector: vec![0, 3],
    //     }
    // }

    pub fn new(&self, e0: f64, e1: f64, e2: f64) -> Self {
        Self {
            vector: vec![e0, e1, e2],
        }
    }

    fn x(&self) -> f64 {
        self.vector[0]
    }

    fn y(&self) -> f64 {
        self.vector[1]        
    }

    fn z(&self) -> f64 {
        self.vector[2]        
    }

    fn negate_operator(&self) -> (f64, f64, f64){
        (-self.vector[0], -self.vector[1], -self.vector[2])
    }

    fn get_value_by_index(&self, i: i8) -> f64 {
        self.vector[i as usize]
    }    

    fn get_value_by_ref(&self, i: i8) -> &f64 {
        &self.vector[i as usize]
    }
    
    fn add(&mut self, v: &Self) -> &Self {
        self.vector[0 as usize] += v.x();
        self.vector[1 as usize] += v.y();
        self.vector[2 as usize] += v.z();

        self
    }

    fn mult(&mut self, t: f64) -> &Self {
        self.vector[0 as usize] *= t;
        self.vector[1 as usize] *= t;
        self.vector[2 as usize] *= t;

        self
    }

    fn div(&mut self, t: f64) -> &Self{
        self.vector *= 1/t
    }
}