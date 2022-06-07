use crate::ray::Ray;
use crate::canvas::*;
use crate::scene::*;
use crate::scene_object::*;
use crate::material::*;
use cglinalg::{ 
    Magnitude,
    Vector3,
};
use rand::prelude::*;


const MAX_DEPTH: u32 = 16;


// TODO: Move to cglinalg crate.
#[inline]
fn component_multiply(v1: Vector3<f32>, v2: Vector3<f32>) -> Vector3<f32> {
    Vector3::new(v1.x * v2.x, v1.y * v2.y, v1.z * v2.z)
}

fn path_trace<H: Intersect>(ray: Ray, scene: &H, rng: &mut ThreadRng, depth: u32) -> Vector3<f32> {
    if let Some(hit) = scene.intersect(&ray, 0.001, std::f32::MAX) {
        if depth < MAX_DEPTH {
            let scattered_ray = hit.object.sample_bsdf(ray, &hit, rng);
            let color = path_trace(scattered_ray.ray, scene, rng, depth + 1);

            return component_multiply(scattered_ray.attenuation, color);
        } else {
            return Vector3::new(0_f32, 0_f32, 0_f32);
        }
    } else {
        let unit_direction = ray.direction.normalize();
        let t = (unit_direction.y + 1_f32) * 0.5;

        return Vector3::new(1_f32, 1_f32, 1_f32) * (1_f32 - t) + Vector3::new(0.5, 0.7, 1.0) * t
    }
}

pub struct Renderer {
    samples_per_pixel: u32,
}

impl Renderer {
    pub fn new(samples_per_pixel: u32) -> Self {
        Self { samples_per_pixel }
    }

    #[inline]
    fn sample_pixel(&self, row: usize, column: usize, rng: &mut ThreadRng, scene: &mut Scene) -> Vector3<f32> {
        let height = scene.canvas.height;
        let width = scene.canvas.width;
        let mut color = Vector3::new(0_f32, 0_f32, 0_f32);
        for _ in 0..self.samples_per_pixel {
            let du = rng.gen::<f32>();
            let u = (column as f32 + du) / (width as f32);
            let dv = rng.gen::<f32>();
            let v = (((height - row) as f32) + dv) / (height as f32);
            let ray = scene.camera.get_ray(rng, u, v);

            color += path_trace(ray, scene, rng, 0);
        }
        let averaged_color = color / self.samples_per_pixel as f32;

        averaged_color
    }

    pub fn render(&self, scene: &mut Scene) {
        let height = scene.canvas.height;
        let width = scene.canvas.width;
        let mut rng = rand::prelude::thread_rng();
        for row in 0..height {
            println!("Rendering line {} of {}", row+1, height);
            for column in 0..width {
                let mut color = self.sample_pixel(row, column, &mut rng, scene);
                color = Vector3::new(
                    f32::sqrt(color[0]), 
                    f32::sqrt(color[1]), 
                    f32::sqrt(color[2])
                );

                let ir = (255.99 * color[0]) as u8;
                let ig = (255.99 * color[1]) as u8;
                let ib = (255.99 * color[2]) as u8;
            
                scene.canvas[row][column] = Rgba::new(ir, ig, ib);
            }
        }
    }
}

