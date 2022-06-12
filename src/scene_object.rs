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

    fn scatter(&mut self, query: &ScatteringQuery) -> ScatteringResult;

    fn center(&self) -> Vector3<f32>;
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

    fn scatter(&mut self, query: &ScatteringQuery) -> ScatteringResult {
        let normal = (query.point - self.geometry.center()).normalize();
        let ray_incoming = query.ray_incoming;
        let bsdf_query = self.sampler.sample(&self.bsdf, &ray_incoming, &normal, &query.point);
        let bsdf_result = self.bsdf.sample(&bsdf_query);
         
        ScatteringResult::new(
            bsdf_result.ray_incoming,
            bsdf_result.ray_outgoing,
            bsdf_result.point,
            bsdf_result.normal,
            bsdf_result.scattering_fraction,
        )
    }

    fn center(&self) -> Vector3<f32> {
        self.geometry.center()
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
    fn query_to_model_space(&self, query: &IntersectionQuery) -> IntersectionQuery {
        let ray_model_space = self.ray_to_model_space(&query.ray);

        IntersectionQuery::new(ray_model_space, query.t_min, query.t_max)
    }

    pub fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        let query_model_space = self.query_to_model_space(query);
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

    pub fn scatter(&self, query: &ScatteringQuery, rng: &mut ThreadRng) -> Option<ScatteringResult> {
        // let scattering_query_model_space = Convert world space scattering query to model space scattering query.
        // let scattering_result_model_space = Run the model space scattering query.
        // Convert the model space scattering result to a world space scattering result.
        // return.
        // TODO: scatter should choose the next ray tracing direction, not sampling the BSDF.
        /*
        let intersection_result = self.intersect(query);
        if let IntersectionResult::Hit(result) | IntersectionResult::Tangent(result) = intersection_result {
            // The ray hit or grazed the object.
            let 

            Some(self.material.sample_bsdf(query.ray, &result, rng))
        } else {
            // The ray missed the object.
            None
        }
        */
        None
    }

    #[inline]
    pub fn center(&self) -> Vector3<f32> {
        (self.model_matrix * self.object.center().extend(1_f32)).contract()
    }
}

