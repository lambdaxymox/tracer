use crate::ray::Ray;
use crate::material::{
    ObjectMaterial, 
};
use cglinalg::{
    Vector3,
    Matrix4x4,
};
use rand::prelude::*;


pub trait Geometry: std::fmt::Debug {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<GeometryIntersectionResult>;

    fn center(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone, Debug)]
pub struct IntersectionResult<'a> {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub object: &'a SceneObject,
}

impl<'a> IntersectionResult<'a> {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>, object: &'a SceneObject) -> IntersectionResult<'a> {
        IntersectionResult {
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

    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionResult> {
        let ray_model_space = self.ray_to_model_space(ray);
        self.geometry.intersect(&ray_model_space, t_min, t_max).map(|res_model_space| {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.p.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();
            let object = self;

            IntersectionResult::new(
                res_t_world_space,
                res_p_world_space,
                res_normal_world_space,
                object
            )
        })
    }

    pub fn sample_bsdf(&self, ray_in: Ray, hit: &IntersectionResult, rng: &mut ThreadRng) -> ScatteredRay {
        self.material.sample_bsdf(ray_in, hit, rng)
    }

    pub fn center(&self) -> Vector3<f32> {
        (self.model_matrix * self.geometry.center().extend(1_f32)).contract()
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct GeometryIntersectionResult {
    pub t: f32,
    pub p: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl<'a> GeometryIntersectionResult {
    pub fn new(t: f32, p: Vector3<f32>, normal: Vector3<f32>) -> GeometryIntersectionResult {
        GeometryIntersectionResult {
            t, p, normal,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Sphere {
    center: Vector3<f32>,
    radius: f32,
}

impl Sphere {
    pub fn new(center: Vector3<f32>, radius: f32) -> Sphere {
        Sphere {
            center, radius,
        }
    }
}

impl Geometry for Sphere {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<GeometryIntersectionResult> {
        let oc = ray.origin - self.center;
        let a = ray.direction.dot(&ray.direction);
        let b = oc.dot(&ray.direction);
        let c = oc.dot(&oc) - self.radius * self.radius;
        let discriminant = b * b - a * c; // 4 * a * c?

        if discriminant > 0_f32 {
            let mut temp = (-b - f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(GeometryIntersectionResult::new(
                    temp,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                ));
            }
            temp = (-b + f32::sqrt(b * b - a * c)) / a; // 4 * a * c?
            if temp < t_max && temp > t_min {
                let hit_point = ray.point_at_parameter(temp);
                return Some(GeometryIntersectionResult::new(
                    temp,
                    hit_point,
                    (hit_point - self.center) / self.radius,
                ));
            }
        }

        None
    }

    #[inline]
    fn center(&self) -> Vector3<f32> {
        self.center
    }
}

