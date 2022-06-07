use crate::ray::Ray;
use crate::canvas::*;
use crate::scene::*;
use cglinalg::{ 
    Magnitude,
    Vector3,
};
use rand::prelude::*;


#[derive(Copy, Clone)]
pub struct RendererSettings {
    samples_per_pixel: usize,
    max_path_depth: usize,
}

impl RendererSettings {
    pub fn new(samples_per_pixel: usize, max_path_depth: usize) -> Self {
        Self { 
            samples_per_pixel, max_path_depth,
        }
    }
}

pub struct Renderer {
    samples_per_pixel: usize,
    max_path_depth: usize,
}

impl Renderer {
    pub fn new(settings: RendererSettings) -> Self {
        Self { 
            samples_per_pixel: settings.samples_per_pixel, 
            max_path_depth: settings.max_path_depth,
        }
    }

    fn path_trace(&self, ray: Ray, scene: &Scene, rng: &mut ThreadRng, depth: usize) -> Vector3<f32> {
        if let Some(hit) = scene.intersect(&ray, 0.001, std::f32::MAX) {
            if depth < self.max_path_depth {
                let scattered_ray = hit.object.sample_bsdf(ray, &hit, rng);
                let color = self.path_trace(scattered_ray.ray, scene, rng, depth + 1);
    
                scattered_ray.attenuation.component_mul(&color)
            } else {
                Vector3::new(0_f32, 0_f32, 0_f32)
            }
        } else {
            let unit_direction = ray.direction.normalize();
            let t = (unit_direction.y + 1_f32) * 0.5;
    
            Vector3::new(1_f32, 1_f32, 1_f32) * (1_f32 - t) + Vector3::new(0.5, 0.7, 1.0) * t
        }
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

            color += self.path_trace(ray, scene, rng, 0);
        }
        
        color / self.samples_per_pixel as f32
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

