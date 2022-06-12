use crate::geometry::*;
use crate::sphere::*;
use crate::query::*;
use crate::bsdf::*;
use cglinalg::{
    Vector3,
    Matrix4x4,
    Magnitude,
};
use rand::prelude::*;


pub trait ModelObject: std::fmt::Debug {
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult;

    fn scatter(&mut self, query: &ScatteringQuery) -> Option<ScatteringResult>;

    fn center(&self) -> Vector3<f32>;

    fn contains(&self, point: &Vector3<f32>) -> bool;

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>>;
}

#[derive(Debug)]
pub struct SphereModelObject<Bsdf> 
where 
    Bsdf: BsdfMapping 
{
    geometry: Sphere,
    bsdf: Box<Bsdf>,
    sampler: Box<dyn BsdfQuerySampler<Bsdf = Bsdf>>,
}

impl<Bsdf> SphereModelObject<Bsdf> 
where 
    Bsdf: BsdfMapping 
{
    pub fn new(geometry: Sphere, bsdf: Box<Bsdf>, sampler: Box<dyn BsdfQuerySampler<Bsdf = Bsdf>>) -> Self {
        Self { geometry, bsdf, sampler, }
    }
}

impl<Bsdf> ModelObject for SphereModelObject<Bsdf>
where 
    Bsdf: BsdfMapping
{
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        self.geometry.intersect(query)
    }

    fn scatter(&mut self, query: &ScatteringQuery) -> Option<ScatteringResult> {
        if let Some(normal) = self.normal(&query.point) {
            let ray_incoming = query.ray_incoming;
            let bsdf_query = self.sampler.sample(&self.bsdf, &ray_incoming, &normal, &query.point);
            let bsdf_result = self.bsdf.sample(&bsdf_query);
             
            Some(ScatteringResult::new(
                bsdf_result.ray_incoming,
                bsdf_result.ray_outgoing,
                bsdf_result.point,
                bsdf_result.normal,
                bsdf_result.scattering_fraction,
            ))
        } else {
            None
        }
    }

    fn center(&self) -> Vector3<f32> {
        self.geometry.center()
    }

    fn contains(&self, point: &Vector3<f32>) -> bool {
        let diff = point - self.geometry.center;
        
        diff.dot(&diff) <= self.geometry.radius * self.geometry.radius
    }

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        if self.contains(point) {
            Some((point - self.geometry.center()).normalize())
        } else {
            None
        }
    }
}



#[derive(Debug)]
pub struct SceneObject {
    object: Box<dyn ModelObject>,
    pub model_matrix: Matrix4x4<f32>,
    model_matrix_inv: Matrix4x4<f32>,
}

impl SceneObject {
    pub fn new(
        object: Box<dyn ModelObject>, 
        model_matrix: Matrix4x4<f32>) -> Self 
    {
        let model_matrix_inv = model_matrix.inverse().unwrap();
        
        Self { object, model_matrix, model_matrix_inv, }
    }

    #[inline]
    fn ray_to_model_space(&self, ray: &Ray) -> Ray {
        let ray_origin_model_space = (self.model_matrix_inv * ray.origin.extend(1_f32)).contract();
        let ray_direction_model_space = (self.model_matrix_inv * ray.direction.extend(0_f32)).contract();
        
        Ray::new(ray_origin_model_space, ray_direction_model_space)
    }

    #[inline]
    fn intersection_query_to_model_space(&self, query: &IntersectionQuery) -> IntersectionQuery {
        let ray_model_space = self.ray_to_model_space(&query.ray);

        IntersectionQuery::new(ray_model_space, query.t_min, query.t_max)
    }

    pub fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        let query_model_space = self.intersection_query_to_model_space(query);
        let result = self.object.intersect(&query_model_space);
        if let IntersectionResult::Hit(res_model_space) = result {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.point.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();

            IntersectionResult::new_hit(
                res_t_world_space,
                res_p_world_space,
                res_normal_world_space,
            )
        } else if let IntersectionResult::Tangent(res_model_space) = result {
            let res_t_world_space = res_model_space.t;
            let res_p_world_space = (self.model_matrix * res_model_space.point.extend(1_f32)).contract();
            let res_normal_world_space = (self.model_matrix * res_model_space.normal.extend(0_f32)).contract();

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

    fn scattering_query_world_space_to_model_space(&self, query: &ScatteringQuery) -> ScatteringQuery {
        unimplemented!()
    }

    fn scattering_result_model_space_to_world_space(&self, result: &ScatteringResult) -> ScatteringResult {
        unimplemented!()
    }

    pub fn scatter(&mut self, query: &ScatteringQuery, rng: &mut ThreadRng) -> Option<ScatteringResult> {
        let query_model_space = self.scattering_query_world_space_to_model_space(query);
        let result_world_space = if let Some(result_model_space) = self.object.scatter(&query_model_space) {
            Some(self.scattering_result_model_space_to_world_space(&result_model_space))
        } else {
            None
        };

        result_world_space
    }

    #[inline]
    pub fn center(&self) -> Vector3<f32> {
        (self.model_matrix * self.object.center().extend(1_f32)).contract()
    }
}

