use rand::prelude::*;
use cglinalg::{
    Magnitude, 
    Vector3
};

pub struct SphereSampler {
    pub rng: ThreadRng,
}

impl SphereSampler {
    pub fn new(rng: ThreadRng) -> Self {
        Self { rng, }
    }

    pub fn sample_unit_sphere(&mut self) -> Vector3<f32> {
        loop {
            let a = self.rng.gen::<f32>();
            let b = self.rng.gen::<f32>();
            let c = self.rng.gen::<f32>();
            let p = Vector3::new(a, b, c) * 2_f32 - Vector3::new(1_f32, 1_f32, 1_f32);
    
            // If the sample falls inside the unit sphere, we can return.
            if p.magnitude() < 1.0 {
                return p;
            }
        }
    }

    pub fn sample_unit_disk(&mut self) -> Vector3<f32> {
        loop {
            let p = Vector3::new(
                2_f32 * self.rng.gen::<f32>() - 1_f32,
                2_f32 * self.rng.gen::<f32>() - 1_f32,
                0_f32,
            );
    
            // If the sample falls inside the unit disk, we can return.
            if p.magnitude_squared() < 1.0 {
                return p;
            }
        }
    }
}

