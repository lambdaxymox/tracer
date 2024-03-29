extern crate tracer;
extern crate rand;
extern crate rand_isaac;


#[cfg(test)]
mod sphere_lambertian_model_tests {
    use tracer::core::*;
    use tracer::bsdf::{
        SimpleLambertianBsdf,
        SimpleLambertianBsdfQuerySampler,
    };
    use tracer::core::{
        Ray,
        IntersectionQuery,
        IntersectionResult,
        ScatteringQuery,
        ScatteringResult,
    };
    use tracer::geometry::{
        Sphere,
    };
    use tracer::scene::*;
    use tracer::light::*;
    use cglinalg::{
        Vector3,
        Magnitude,
    };
    use rand::prelude::*;


    fn sphere() -> ModelSpaceGeometryObject<Sphere, SimpleLambertianBsdf, NoLight> {
        let sphere = Sphere::new(Vector3::zero(), 1_f32);
        let bsdf = Box::new(
            SimpleLambertianBsdf::new(Vector3::new(0.5, 0.5, 0.5))
        );
        let bsdf_sampler = Box::new(SimpleLambertianBsdfQuerySampler::new());
        let emitter = Box::new(NoLight::new());

        ModelSpaceGeometryObject::new(sphere, bsdf, bsdf_sampler, emitter)
    }

    #[test]
    fn test_intersection() {
        let sphere = sphere();
        let ray = Ray::new(
            Vector3::new(0_f32, 0_f32, 30_f32), 
            Vector3::new(0_f32, 0_f32, -1_f32)
        );
        let intersection_query = IntersectionQuery::new(ray, 0.001, f32::MAX);
        let expected = IntersectionResult::new_hit(
            29_f32, 
            Vector3::new(0_f32, 0_f32, 1_f32), 
            Vector3::new(0_f32, 0_f32, 1_f32)
        );
        let result = sphere.intersect(&intersection_query);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scattering() {
        let sphere = sphere();
        let rng = rand_isaac::Isaac64Rng::from_rng(rand::prelude::thread_rng()).unwrap();
        let mut sampler = SphereSampler::new(rng);
        let ray_incoming = Ray::new(
            Vector3::new(0_f32, 0_f32, 30_f32), 
            Vector3::new(0_f32, 0_f32, -1_f32)
        );
        let intersection_query = IntersectionQuery::new(ray_incoming, 0.001, f32::MAX);
        let intersection_result = sphere.intersect(&intersection_query).unwrap_hit();
        let ray_outgoing_direction = (intersection_result.normal + Vector3::new(1_f32, 1_f32, 0_f32)).normalize();
        let scattering_query = ScatteringQuery::new(
            ray_incoming.direction,
            intersection_result.point,
        );
        let expected = ScatteringResult::new(
            ray_incoming.direction,
            ray_outgoing_direction,
            intersection_result.point,
            intersection_result.normal,
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::zero()
        );
        let result = sphere.scatter(&scattering_query, &mut sampler);

        assert_eq!(result.ray_incoming, expected.ray_incoming);
        assert_eq!(result.point, expected.point);
        assert_eq!(result.normal, expected.normal);
        assert_eq!(result.scattering_fraction, expected.scattering_fraction);
    }
}


#[cfg(test)]
mod sphere_metal_model_tests {
    use tracer::core::*;
    use tracer::bsdf::{
        SimpleMetalBsdf,
        SimpleMetalBsdfQuerySampler,
    };
    use tracer::core::{
        Ray,
        IntersectionQuery,
        IntersectionResult,
        ScatteringQuery,
        ScatteringResult,
    };
    use tracer::geometry::{
        Sphere,
    };
    use tracer::scene::*;
    use tracer::light::*;
    use cglinalg::{
        Vector3,
        Magnitude,
    };
    use rand::prelude::*;

    fn sphere() -> ModelSpaceGeometryObject<Sphere, SimpleMetalBsdf, NoLight> {
        let sphere = Sphere::new(Vector3::zero(), 1_f32);
        let bsdf = Box::new(
            SimpleMetalBsdf::new(Vector3::new(0.5, 0.5, 0.5), 0.2)
        );
        let bsdf_sampler = Box::new(SimpleMetalBsdfQuerySampler::new());
        let emitter = Box::new(NoLight::new());

        ModelSpaceGeometryObject::new(sphere, bsdf, bsdf_sampler, emitter)
    }

    #[test]
    fn test_intersection() {
        let sphere = sphere();
        let ray = Ray::new(
            Vector3::new(0_f32, 0_f32, 30_f32), 
            Vector3::new(0_f32, 0_f32, -1_f32)
        );
        let intersection_query = IntersectionQuery::new(ray, 0.001, f32::MAX);
        let expected = IntersectionResult::new_hit(
            29_f32, 
            Vector3::new(0_f32, 0_f32, 1_f32), 
            Vector3::new(0_f32, 0_f32, 1_f32)
        );
        let result = sphere.intersect(&intersection_query);

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scattering() {
        let sphere = sphere();
        let rng = rand_isaac::Isaac64Rng::from_rng(rand::prelude::thread_rng()).unwrap();
        let mut sampler = SphereSampler::new(rng);
        let ray_incoming = Ray::new(
            Vector3::new(0_f32, 0_f32, 30_f32), 
            Vector3::new(0_f32, 0_f32, -1_f32)
        );
        let intersection_query = IntersectionQuery::new(ray_incoming, 0.001, f32::MAX);
        let intersection_result = sphere.intersect(&intersection_query).unwrap_hit();
        let ray_outgoing_direction = (intersection_result.normal + Vector3::new(1_f32, 1_f32, 0_f32)).normalize();
        let scattering_query = ScatteringQuery::new(
            ray_incoming.direction,
            intersection_result.point,
        );
        let expected = ScatteringResult::new(
            ray_incoming.direction,
            ray_outgoing_direction,
            intersection_result.point,
            intersection_result.normal,
            Vector3::new(0.5, 0.5, 0.5),
            Vector3::zero()
        );
        let result = sphere.scatter(&scattering_query, &mut sampler);

        assert_eq!(result.ray_incoming, expected.ray_incoming);
        assert_eq!(result.point, expected.point);
        assert_eq!(result.normal, expected.normal);
        assert_eq!(result.scattering_fraction, expected.scattering_fraction);
    }
}

