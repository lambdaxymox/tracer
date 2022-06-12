extern crate tracer;


#[cfg(test)]
mod sphere_lambertian_model_tests {
    use tracer::{
        Camera,
        Scene,
        Ray,
        Sphere,
        SceneObject,
        SimpleLambertianBsdf,
        SimpleLambertianBsdfQuerySampler,
        IntersectionQuery,
        IntersectionResult,
        SphereModelObject,
        ModelObject,
        ScatteringQuery,
        ScatteringResult,
    };
    use cglinalg::{
        Vector3,
        Matrix4x4,
        Magnitude,
    };

    fn sphere() -> SphereModelObject<SimpleLambertianBsdf> {
        let rng = rand::prelude::thread_rng();
        let sphere = Sphere::new(Vector3::zero(), 1_f32);
        let bsdf = SimpleLambertianBsdf::new(Vector3::new(0.5, 0.5, 0.5));
        let bsdf_sampler = SimpleLambertianBsdfQuerySampler::new(rng);

        SphereModelObject::new(sphere, Box::new(bsdf), Box::new(bsdf_sampler))
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
        let mut sphere = sphere();
        let ray_incoming = Ray::new(
            Vector3::new(0_f32, 0_f32, 30_f32), 
            Vector3::new(0_f32, 0_f32, -1_f32)
        );
        let intersection_query = IntersectionQuery::new(ray_incoming, 0.001, f32::MAX);
        let intersection_result = sphere.intersect(&intersection_query).unwrap_hit();
        let ray_outgoing_direction = (intersection_result.normal + Vector3::new(1_f32, 1_f32, 0_f32)).normalize();
        let scattering_query = ScatteringQuery::new(
            ray_incoming.direction,
            ray_outgoing_direction,
            intersection_result.point,
        );
        let expected = ScatteringResult::new(
            ray_incoming.direction,
            ray_outgoing_direction,
            intersection_result.point,
            intersection_result.normal,
            Vector3::new(0.5, 0.5, 0.5)
        );
        let result = sphere.scatter(&scattering_query);

        assert_eq!(result, expected);
    }
}

