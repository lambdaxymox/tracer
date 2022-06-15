extern crate tracer;
extern crate approx;
extern crate rand;
extern crate rand_isaac;


#[cfg(test)]
mod scene_tests {
    use tracer::{
        Camera,
        SphereSampler,
    };
    use tracer::bsdf::{
        SimpleLambertianBsdf,
        SimpleLambertianBsdfQuerySampler,
    };
    use tracer::query::{
        IntersectionQuery, 
        IntersectionResult,
        Ray,
        ScatteringQuery,
    };
    use tracer::geometry::{
        Sphere,
    };
    use tracer::scene::*;
    use tracer::light::*;
    use approx::{
        assert_relative_eq,
    };
    use cglinalg::{
        Vector3,
        Matrix4x4,
        Magnitude,
    };
    use rand::prelude::*;

    fn scene() -> Scene {
        let sphere_center_model_space = Vector3::zero();
        let sphere_radius = 1_f32;
        let sphere_center_world_space = Vector3::new(4_f32, 5_f32, 6_f32);
        let model_matrix = Matrix4x4::from_affine_translation(&sphere_center_world_space);
        let sphere = Sphere::new(sphere_center_model_space, sphere_radius);
        let bsdf = Box::new(SimpleLambertianBsdf::new(Vector3::new(0.5, 0.5, 0.5)));
        let bsdf_sampler = Box::new(SimpleLambertianBsdfQuerySampler::new());
        let emitter = Box::new(NoLight::new());
        let object = Box::new(ModelSpaceGeometryObject::new(sphere, bsdf, bsdf_sampler, emitter));
        let scene_object = SceneObject::new(object, model_matrix);
        let camera = (|width: usize, height: usize| {
            let look_from = Vector3::new(-4_f32, -5_f32, 0_f32);
            let look_at = sphere_center_world_space;
            let distance_to_focus = (look_from - look_at).magnitude();
            let aperture = 0.1_f32;
            let v_up = Vector3::new(0_f32, 1_f32, 0_f32);
            let v_fov = 20_f32;
            let aspect_ratio = (width as f32) / (height as f32);

            Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, distance_to_focus)
        })(720, 480);
        let mut scene = Scene::new(720, 480, camera);
        scene.push(scene_object);

        scene
    }

    #[test]
    fn test_scene_occupied() {
        let scene = scene();

        assert!(!scene.is_empty_objects());
    }

    #[test]
    fn test_scene_sphere_sample_ray_intersects_from_camera() {
        let scene = scene();
        let ray = Ray::new(scene.camera.position(), scene.camera.forward());
        let query = IntersectionQuery::new(ray, 0.1, f32::MAX);
        
        assert!(scene.ray_cast(&query).is_some());
    }

    #[test]
    fn test_scene_sphere_sample_ray() {
        let scene = scene();
        let ray_origin = scene.camera.position();
        let ray_direction = scene.camera.forward();
        let ray = Ray::new(ray_origin, ray_direction);
        let query = IntersectionQuery::new(ray, 0.1, f32::MAX);
        let sphere = scene.ray_cast(&query).unwrap();
        let expected = IntersectionResult::new_hit(
            13.142121,
            Vector3::new(4_f32, 5_f32, 6_f32) - ray_direction,
            -ray_direction,
        ).unwrap_hit();
        let result = sphere.object.intersect(&query).unwrap_hit();
        
        assert_relative_eq!(result.t, expected.t);
        assert_relative_eq!(result.point, expected.point, epsilon = 1e-4);
        assert_relative_eq!(result.normal, expected.normal, epsilon = 1e-4);
    }

    #[test]
    fn test_scene_sphere_center() {
        let scene = scene();
        let sphere = &scene.objects[0];
        let expected = Vector3::zero().extend(1_f32);
        let sphere_center = sphere.center().extend(1_f32);
        let result = sphere.model_matrix.inverse().unwrap() * sphere_center;

        assert_eq!(result, expected);
    }

    #[test]
    fn test_scene_sphere_sample_bsdf() {
        let scene = scene();
        let rng = rand_isaac::Isaac64Rng::from_rng(rand::prelude::thread_rng()).unwrap();
        let mut sampler = SphereSampler::new(rng);
        let ray = Ray::new(scene.camera.position(), scene.camera.forward());
        let intersection_query = IntersectionQuery::new(ray, 0.1, f32::MAX);
        let sphere = scene.ray_cast(&intersection_query).unwrap();
        let intersection_result = sphere.intersection_result.unwrap_hit();
        let scattering_query = ScatteringQuery::new(
            intersection_query.ray.direction,
            intersection_result.point,
        );
        let expected = Vector3::new(0.5, 0.5, 0.5);
        let scattering_result = sphere.object.scatter(&scattering_query, &mut sampler);

        let result = scattering_result.scattering_fraction;

        assert_eq!(result, expected);
    }
}

