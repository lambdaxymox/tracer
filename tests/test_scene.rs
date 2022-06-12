extern crate tracer;


#[cfg(test)]
mod scene_tests {
    use tracer::{
        Camera,
        Scene,
        Ray,
        Sphere,
        SceneObject,
        SimpleLambertianBsdf,
        IntersectionQuery, SimpleLambertianBsdfQuerySampler,
        SphereModelObject,
    };
    use cglinalg::{
        Vector3,
        Matrix4x4,
        Magnitude,
    };

    fn scene() -> Scene {
        let sphere_center_model_space = Vector3::zero();
        let sphere_radius = 1_f32;
        let sphere_center_world_space = Vector3::new(4_f32, 5_f32, 6_f32);
        let model_matrix = Matrix4x4::from_affine_translation(&sphere_center_world_space);
        let sphere = Sphere::new(sphere_center_model_space, sphere_radius);
        let bsdf = Box::new(SimpleLambertianBsdf::new(Vector3::new(0.5, 0.5, 0.5)));
        let rng = rand::prelude::thread_rng();
        let bsdf_sampler = Box::new(SimpleLambertianBsdfQuerySampler::new(rng));
        let object = Box::new(SphereModelObject::new(sphere, bsdf, bsdf_sampler));
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

        eprintln!("{:?}", ray);
        eprintln!("{:?}", scene.objects[0].center());
        eprintln!("{:?}", (scene.objects[0].center() - ray.origin).normalize());
        eprintln!("{:?}", scene.camera.forward());
        
        assert!(scene.ray_cast(&query).is_some());
    }

    #[test]
    fn test_scene_sphere_sample_ray() {
        let scene = scene();
        let ray = Ray::new(scene.camera.position(), scene.camera.forward());
        let query = IntersectionQuery::new(ray, 0.1, f32::MAX);
        let sphere = scene.ray_cast(&query).unwrap();
        
        /*
        let expected = ;
        let intersection_result = sphere.intersect(&ray, 0.1, f32::MAX);
        let result = intersection_result.ray;

        assert_eq!(result, expected);
        */
        todo!("FINISH ME!")
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
    fn test_scene_sphere_sample_normal() {
        let scene = scene();
        let ray = Ray::new(scene.camera.position(), scene.camera.forward());
        let query = IntersectionQuery::new(ray, 0.1, f32::MAX);
        let sphere = scene.ray_cast(&query).unwrap();
        
        /*
        let expected = ;
        let intersection_result = sphere.intersect(&ray, 0.1, f32::MAX);;
        let result = intersection_result.normal;

        assert_eq!(result, expected);
        */
        todo!("FINISH ME!")
    }

    #[test]
    fn test_scene_sphere_sample_bsdf() {
        let scene = scene();
        let ray = Ray::new(scene.camera.position(), scene.camera.forward());
        let query = IntersectionQuery::new(ray, 0.1, f32::MAX);
        let sphere = scene.ray_cast(&query).unwrap();
        
        /*
        let expected = ;
        let scattered_ray = sphere.sample_bsdf(&ray);
        let result = scattered_ray.scattering_fraction;

        assert_eq!(result, expected);
        */
        todo!("FINISH ME!")
    }
}