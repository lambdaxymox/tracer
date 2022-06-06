use rand::prelude::*;
use cglinalg::{
    Magnitude, 
    Vector3
};


pub fn random_in_unit_sphere(rng: &mut ThreadRng) -> Vector3<f32> {
    loop {
        let a = rng.gen::<f32>();
        let b = rng.gen::<f32>();
        let c = rng.gen::<f32>();
        let p = Vector3::new(a, b, c) * 2_f32 - Vector3::new(1_f32, 1_f32, 1_f32);

        // If the sample falls inside the unit sphere, we can return.
        if p.magnitude() < 1.0 {
            return p;
        }
    }
}

pub fn random_in_unit_disk(rng: &mut ThreadRng) -> Vector3<f32> {
    loop {
        let p = Vector3::new(
            2_f32 * rng.gen::<f32>() - 1_f32,
            2_f32 * rng.gen::<f32>() - 1_f32,
            0_f32,
        );

        // If the sample falls inside the unit disk, we can return.
        if p.magnitude_squared() < 1.0 {
            return p;
        }
    }
}
