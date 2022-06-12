use crate::query::*;
use crate::canvas::*;
use crate::scene::*;
use crate::scene_object::*;
use cglinalg::{ 
    Vector3,
    Magnitude,
};
use rand::prelude::*;


#[derive(Copy, Clone)]
pub struct RendererSettings {
    samples_per_pixel: usize,
    max_path_depth: usize,
    t_min: f32,
    t_max: f32,
}

impl RendererSettings {
    pub fn new(samples_per_pixel: usize, max_path_depth: usize) -> Self {
        Self { 
            samples_per_pixel, 
            max_path_depth,
            t_min: 0.0001,
            t_max: f32::MAX
        }
    }
}

pub struct Renderer {
    samples_per_pixel: usize,
    max_path_depth: usize,
    t_min: f32,
    t_max: f32,
}

impl Renderer {
    pub fn new(settings: RendererSettings) -> Self {
        Self { 
            samples_per_pixel: settings.samples_per_pixel, 
            max_path_depth: settings.max_path_depth,
            t_min: settings.t_min,
            t_max: settings.t_max,
        }
    }

    fn estimate(&self, scene: &mut Scene, query: &IntersectionQuery, rng: &mut ThreadRng, depth: usize) -> Vector3<f32> {
        // TODO: Include ability to sample emissions for scene objects that are lights.
        if let Some(hit) = scene.ray_cast(query) {
            if depth < self.max_path_depth {
                let scattering_query = ScatteringQuery::new(
                    query.ray.direction,
                    Vector3::zero(),
                    hit.intersection_result.unwrap_hit_or_tangent().point
                );
                // WARNING: This is really really unsafe. Do this more safely. This is a temporary hack to get 
                // the algorithm and abstractions straightened out first. cf. UnsafeCell or Cell or RefCell.
                #[allow(mutable_transmutes)]
                let unsafe_hit_object = unsafe {
                    std::mem::transmute::<&SceneObject, &mut SceneObject>(hit.object)
                };
                if let Some(scattering_result) = unsafe_hit_object.scatter(&scattering_query, rng) {
                    let next_origin = scattering_result.point;
                    let next_direction = scattering_result.ray_outgoing;
                    let next_incoming_ray = Ray::new(next_origin, next_direction);
                    let next_intersection_query = IntersectionQuery::new(next_incoming_ray, query.t_min, query.t_max);
                    let color = self.estimate(scene, &next_intersection_query, rng, depth + 1);
    
                    scattering_result.scattering_fraction.component_mul(&color)
                } else {
                    Vector3::new(0_f32, 0_f32, 0_f32)
                }
            } else {
                Vector3::new(0_f32, 0_f32, 0_f32)
            }
        } else {
            let unit_direction = query.ray.direction.normalize();
            let t = (unit_direction.y + 1_f32) * 0.5;
    
            // TODO: Convert default value to some kind of ambient light instead of baking into path tracer.
            // When we reach the end of a ray, there is always the ambient lighting to return.
            Vector3::new(1_f32, 1_f32, 1_f32) * (1_f32 - t) + Vector3::new(0.5, 0.7, 1.0) * t
        }
    }

    #[inline]
    fn sample_pixel(&self, scene: &mut Scene, row: usize, column: usize, rng: &mut ThreadRng) -> Vector3<f32> {
        let height = scene.canvas.height;
        let width = scene.canvas.width;
        let mut color = Vector3::new(0_f32, 0_f32, 0_f32);
        for _ in 0..self.samples_per_pixel {
            let du = rng.gen::<f32>();
            let u = (column as f32 + du) / (width as f32);
            let dv = rng.gen::<f32>();
            let v = (((height - row) as f32) + dv) / (height as f32);
            let ray = scene.camera.cast_ray(rng, u, v);
            let query = IntersectionQuery::new(ray, self.t_min, self.t_max);

            color += self.estimate(scene, &query, rng, 0);
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
                let mut color = self.sample_pixel(scene, row, column, &mut rng);
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

