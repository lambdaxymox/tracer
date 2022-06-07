use crate::camera::Camera;
use crate::ray::Ray;
use crate::scene::*;
use crate::material::*;
use cglinalg::{ 
    Magnitude,
    Vector3,
};
use rand::prelude::*;


const MAX_DEPTH: u32 = 16;


#[inline]
fn component_multiply(v1: Vector3<f32>, v2: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

fn color<H: Intersect>(ray: Ray, world: &H, rng: &mut ThreadRng, depth: u32) -> Vector3<f32> {
    match world.intersect(&ray, 0.001, std::f32::MAX) {
        Some(hit) => {    
            if depth < MAX_DEPTH {
                let scatter = hit.material.scatter(ray, &hit, rng);
                let col = color(scatter.ray, world, rng, depth + 1);
                return component_multiply(scatter.attenuation, col);
            } else {
                return Vector3::new(0_f32, 0_f32, 0_f32);
            }
        }
        None => {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1_f32) * 0.5;
            return Vector3::new(1_f32, 1_f32, 1_f32) * (1_f32 - t) + Vector3::new(0.5, 0.7, 1.0) * t
        }
    }
}

pub fn render(width: usize, height: usize, samples_per_pixel: u32, camera: Camera, scene: &mut Scene) {
    let mut rng = rand::prelude::thread_rng();
    // let mut data = vec![];
    for row in 0..height {
        println!("Rendering line {} of {}", row+1, height);
        for column in 0..width {
            let mut col = Vector3::new(0_f32, 0_f32, 0_f32);
            for _ in 0..samples_per_pixel {
                let du = rng.gen::<f32>();
                let u = (column as f32 + du) / (width as f32);
                let dv = rng.gen::<f32>();
                let v = (((height - row) as f32) + dv) / (height as f32);
                let ray = camera.get_ray(&mut rng, u, v);
                col += color(ray, scene, &mut rng, 0);
            }
            col /= samples_per_pixel as f32;
            col = Vector3::new(f32::sqrt(col[0]), f32::sqrt(col[1]), f32::sqrt(col[2]));
            let ir = (255.99 * col[0]) as u8;
            let ig = (255.99 * col[1]) as u8;
            let ib = (255.99 * col[2]) as u8;
            
            // data.push(Rgba::new(ir, ig, ib));
            scene.canvas[row][column] = Rgba::new(ir, ig, ib);
        }
    }

    // scene.canvas.data = data
}

