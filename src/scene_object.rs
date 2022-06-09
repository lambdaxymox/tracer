use crate::geometry::*;
use crate::ray::Ray;
use crate::material::{
    ObjectMaterial, 
};
use cglinalg::{
    Vector3,
    Matrix4x4,
};
use rand::prelude::*;




#[derive(Copy, Clone, Debug)]
pub struct ObjectIntersectionResult<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub object: &'a SceneObject,
}

impl<'a> ObjectIntersectionResult<'a> {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>, object: &'a SceneObject) -> ObjectIntersectionResult<'a> {
        ObjectIntersectionResult {
            t, p, normal, object,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScatteredRay {
    pub scattering_fraction: Vector3<f32>,
    pub ray: Ray,
}

impl ScatteredRay {
    pub fn new(scattering_fraction: Vector3<f32>, ray: Ray) -> ScatteredRay {
        ScatteredRay { 
            scattering_fraction, ray,
        }
    }
}

#[derive(Debug)]
pub struct SceneObject {
    geometry: Box<dyn Geometry>,
    material: Box<dyn ObjectMaterial>,
    pub model_matrix: Matrix4x4<f32>,
    model_matrix_inv: Matrix4x4<f32>,
}

impl SceneObject {
    pub fn new(
        geometry: Box<dyn Geometry>, 
        material: Box<dyn ObjectMaterial>, 
        model_matrix: Matrix4x4<f32>) -> Self 
    {
        Self { 
            geometry, 
            material, 
            model_matrix,
            model_matrix_inv: model_matrix.inverse().unwrap()
        }
    }

    #[inline]
    fn ray_to_model_space(&self, ray: &Ray) -> Ray {
        let ray_origin_model_space = (self.model_matrix_inv * ray.origin.extend(1_f32)).contract();
        let ray_direction_model_space = (self.model_matrix_inv * ray.direction.extend(0_f32)).contract();
        
        Ray::new(ray_origin_model_space, ray_direction_model_space)
    }

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<ObjectIntersectionResult> {
        let ray_model_space = self.ray_to_model_space(ray);
        let result = self.geometry.intersect(&ray_model_space, t_min, t_max);
        if let IntersectionResult::Hit(res_model_space) = result {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.point.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();
            let object = self;

            Some(ObjectIntersectionResult::new(
                res_t_world_space,
                res_p_world_space,
                res_normal_world_space,
                object
            ))
        } else if let IntersectionResult::Tangent(res_model_space) = result {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.point.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();
            let object = self;

            Some(ObjectIntersectionResult::new(
                res_t_world_space,
                res_p_world_space,
                res_normal_world_space,
                object
            ))
        } else {
            None
        }
    }

    pub fn sample_bsdf(&self, ray_in: Ray, hit: &ObjectIntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        self.material.sample_bsdf(ray_in, hit, rng)
    }

    #[inline]
    pub fn center(&self) -> Vector3<f32> {
        (self.model_matrix * self.geometry.center().extend(1_f32)).contract()
    }
}

