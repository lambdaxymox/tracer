use crate::sampler::*;
use crate::query::*;
use crate::scene::*;
use cglinalg::{
    Vector3,
    Matrix4x4,
};


#[derive(Debug)]
pub struct SceneObject {
    object: Box<dyn ModelSpaceScatteringObject>,
    pub model_matrix: Matrix4x4<f32>,
    model_matrix_inv: Matrix4x4<f32>,
}

impl SceneObject {
    pub fn new(object: Box<dyn ModelSpaceScatteringObject>, model_matrix: Matrix4x4<f32>) -> Self {
        let model_matrix_inv = model_matrix.inverse().unwrap();
        
        Self { object, model_matrix, model_matrix_inv, }
    }

    #[inline]
    fn ray_world_space_to_model_space(&self, ray: &Ray) -> Ray {
        let ray_origin_model_space = (self.model_matrix_inv * ray.origin.extend(1_f32)).contract();
        let ray_direction_model_space = (self.model_matrix_inv * ray.direction.extend(0_f32)).contract();
        
        Ray::new(ray_origin_model_space, ray_direction_model_space)
    }

    #[inline]
    fn intersection_query_world_space_to_model_space(&self, query: &IntersectionQuery) -> IntersectionQuery {
        let ray_model_space = self.ray_world_space_to_model_space(&query.ray);

        IntersectionQuery::new(ray_model_space, query.t_min, query.t_max)
    }

    #[inline]
    fn intersection_result_model_space_to_world_space(&self, result: &IntersectionResult) -> IntersectionResult {
        if let IntersectionResult::Hit(result_model_space) = result {
            let result_t_world_space = result_model_space.t;
            let result_p_world_space = (self.model_matrix * result_model_space.point.extend(1_f32)).contract();
            let result_normal_world_space = (self.model_matrix * result_model_space.normal.extend(0_f32)).contract();

            IntersectionResult::new_hit(
                result_t_world_space,
                result_p_world_space,
                result_normal_world_space,
            )
        } else if let IntersectionResult::Tangent(result_model_space) = result {
            let result_t_world_space = result_model_space.t;
            let result_p_world_space = (self.model_matrix * result_model_space.point.extend(1_f32)).contract();
            let result_normal_world_space = (self.model_matrix * result_model_space.normal.extend(0_f32)).contract();

            IntersectionResult::new_tangent(
                result_t_world_space,
                result_p_world_space,
                result_normal_world_space,
            )
        } else {
            // The ray missed the object.
            *result
        }
    }

    pub fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        let query_model_space = self.intersection_query_world_space_to_model_space(query);
        let result_model_space = self.object.intersect(&query_model_space);

        self.intersection_result_model_space_to_world_space(&result_model_space)
    }

    fn scattering_query_world_space_to_model_space(&self, query: &ScatteringQuery) -> ScatteringQuery {
        let ray_incoming_model_space = (self.model_matrix_inv * query.ray_incoming.extend(0_f32)).contract();
        let point_model_space = (self.model_matrix_inv * query.point.extend(1_f32)).contract();

        ScatteringQuery::new(ray_incoming_model_space, point_model_space)
    }

    fn scattering_result_model_space_to_world_space(&self, result: &ScatteringResult) -> ScatteringResult {
        let ray_incoming_model_space = (self.model_matrix * result.ray_incoming.extend(0_f32)).contract();
        let ray_outgoing_model_space = (self.model_matrix * result.ray_outgoing.extend(0_f32)).contract();
        let point_model_space = (self.model_matrix * result.point.extend(1_f32)).contract();
        let normal_model_space = (self.model_matrix * result.normal.extend(0_f32)).contract();

        ScatteringResult::new(
            ray_incoming_model_space,
            ray_outgoing_model_space,
            point_model_space,
            normal_model_space,
            result.scattering_fraction
        )
    }

    pub fn scatter(&self, query: &ScatteringQuery, sampler: &mut SphereSampler) -> ScatteringResult {
        let query_model_space = self.scattering_query_world_space_to_model_space(query);
        let result_model_space = self.object.scatter(&query_model_space, sampler);
        
        self.scattering_result_model_space_to_world_space(&result_model_space)
    }

    #[inline]
    pub fn center(&self) -> Vector3<f32> {
        (self.model_matrix * self.object.center().extend(1_f32)).contract()
    }
}

