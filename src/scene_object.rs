use crate::geometry::*;
use crate::query::*;
use crate::material::{
    ObjectMaterial, 
};
use cglinalg::{
    Vector3,
    Matrix4x4,
};
use rand::prelude::*;


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

    #[inline]
    fn query_to_model_space(&self, query: &IntersectionQuery) -> IntersectionQuery {
        let ray_model_space = self.ray_to_model_space(&query.ray);

        IntersectionQuery::new(ray_model_space, query.t_min, query.t_max)
    }

    pub fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        let query_model_space = self.query_to_model_space(query);
        let result = self.geometry.intersect(&query_model_space);
        if let IntersectionResult::Hit(res_model_space) = result {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.point.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();
            let object = self;

            IntersectionResult::new_hit(
                res_t_world_space,
                res_p_world_space,
                res_normal_world_space,
            )
        } else if let IntersectionResult::Tangent(res_model_space) = result {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.point.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();
            let object = self;

            IntersectionResult::new_tangent(
                res_t_world_space,
                res_p_world_space,
                res_normal_world_space,
            )
        } else {
            // The ray missed the object.
            result
        }
    }

    pub fn scatter(&self, query: &IntersectionQuery, rng: &mut ThreadRng) -> Option<ScatteringResult> {
        let result = self.intersect(query);
        if let IntersectionResult::Hit(_) = result {
            Some(self.material.sample_bsdf(query.ray, &result, rng))
        } else if let IntersectionResult::Tangent(graze) = result {
            Some(self.material.sample_bsdf(query.ray, &result, rng))
        } else {
            // We missed the object.
            None
        }

    } 

    #[inline]
    pub fn center(&self) -> Vector3<f32> {
        (self.model_matrix * self.geometry.center().extend(1_f32)).contract()
    }
}

