use crate::query::*;
use crate::canvas::*;
use crate::sampler::*;
use crate::scene::*;
use cglinalg::{ 
    Vector3,
    Magnitude,
};


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

    fn path_trace(&self, scene: &Scene, query: &IntersectionQuery, sampler: &mut SphereSampler, depth: usize) -> Vector3<f32> {
        if let Some(hit) = scene.ray_cast(query) {
            if depth < self.max_path_depth {
                let scattering_query = ScatteringQuery::new(
                    query.ray.direction,
                    hit.intersection_result.unwrap_hit_or_tangent().point
                );
                let scattering_result = hit.object.scatter(&scattering_query, sampler);
                let next_origin = scattering_result.point;
                let next_direction = scattering_result.ray_outgoing;
                let next_incoming_ray = Ray::new(next_origin, next_direction);
                let next_intersection_query = IntersectionQuery::new(next_incoming_ray, query.t_min, query.t_max);
                let next_estimate = self.path_trace(scene, &next_intersection_query, sampler, depth + 1);
                let estimated_from_indirect_light = scattering_result.scattering_fraction.component_mul(&next_estimate);
                
                scattering_result.emission + estimated_from_indirect_light
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
    fn sample_pixel(&self, scene: &Scene, row: usize, column: usize, sampler: &mut SphereSampler, canvas: &mut Canvas) -> Vector3<f32> {
        let height = canvas.height;
        let width = canvas.width;
        let mut color = Vector3::new(0_f32, 0_f32, 0_f32);
        for _ in 0..self.samples_per_pixel {
            let du = sampler.sample_f32();
            let u = (column as f32 + du) / (width as f32);
            let dv = sampler.sample_f32();
            let v = (((height - row) as f32) + dv) / (height as f32);
            let ray = scene.camera.cast_ray(sampler, u, v);
            let query = IntersectionQuery::new(ray, self.t_min, self.t_max);

            color += self.path_trace(scene, &query, sampler, 0);
        }
        
        color / self.samples_per_pixel as f32
    }

    pub fn render(&self, scene: &Scene, sampler: &mut SphereSampler, canvas: &mut Canvas) {
        let height = canvas.height;
        let width = canvas.width;
        for row in 0..height {
            println!("Rendering line {} of {}", row+1, height);
            for column in 0..width {
                let mut color = self.sample_pixel(scene, row, column, sampler, canvas);
                color = Vector3::new(
                    f32::sqrt(color[0]), 
                    f32::sqrt(color[1]), 
                    f32::sqrt(color[2])
                );

                let ir = (255.99 * color[0]) as u8;
                let ig = (255.99 * color[1]) as u8;
                let ib = (255.99 * color[2]) as u8;
            
                canvas[row][column] = Rgba::new(ir, ig, ib);
            }
        }
    }
}

