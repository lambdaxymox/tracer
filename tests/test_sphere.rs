extern crate tracer;


#[cfg(test)]
mod sphere_tests {
    use tracer::{
        Ray,
        Sphere,
        Geometry,
    };
    use cglinalg::{
        Vector3,
        Magnitude,
    };
    use std::f32;


    fn sphere() -> Sphere {
        Sphere::new(
            Vector3::zero(),
            1_f32
        )
    }

    #[test]
    fn test_rays_cast_towards_model_space_center_intersect_sphere_at_origin() {
        let sphere = sphere();
        let path_radius = 2_f32;

        for i in 0..65 {
            let ray_origin = Vector3::new(
                path_radius * f32::cos((i as f32) * f32::consts::PI / 64_f32),
                path_radius * f32::sin((i as f32) * f32::consts::PI / 64_f32),
                0_f32
            );
            let ray_direction = (sphere.center() - ray_origin).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }

        for j in 0..65 {
            let ray_origin = Vector3::new(
                0_f32,
                path_radius * f32::cos((j as f32) * f32::consts::PI / 64_f32),                
                path_radius * f32::sin((j as f32) * f32::consts::PI / 64_f32),
            );
            let ray_direction = (sphere.center() - ray_origin).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }

        for k in 0..65 {
            // We are sweeping the zx plane, not the xz plane.
            let ray_origin = Vector3::new(
                path_radius * f32::sin((k as f32) * f32::consts::PI / 64_f32),
                0_f32,
                path_radius * f32::cos((k as f32) * f32::consts::PI / 64_f32),                
            );
            let ray_direction = (sphere.center() - ray_origin).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_away_from_model_space_center_do_not_intersect_sphere_at_origin() {
        let sphere = sphere();
        let path_radius = 2_f32;

        for i in 0..65 {
            let ray_origin = Vector3::new(
                path_radius * f32::cos((i as f32) * f32::consts::PI / 64_f32),
                path_radius * f32::sin((i as f32) * f32::consts::PI / 64_f32),
                0_f32
            );
            let ray_direction = (ray_origin - sphere.center()).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
        }

        for j in 0..65 {
            let ray_origin = Vector3::new(
                0_f32,
                path_radius * f32::cos((j as f32) * f32::consts::PI / 64_f32),                
                path_radius * f32::sin((j as f32) * f32::consts::PI / 64_f32),
            );
            let ray_direction = (ray_origin - sphere.center()).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
        }

        for k in 0..65 {
            // We are sweeping the zx plane, not the xz plane.
            let ray_origin = Vector3::new(
                path_radius * f32::sin((k as f32) * f32::consts::PI / 64_f32),
                0_f32,
                path_radius * f32::cos((k as f32) * f32::consts::PI / 64_f32),                
            );
            let ray_direction = (ray_origin - sphere.center()).normalize();
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_x_axis_sweeping_in_y_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_x = 3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_y = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let origin_z = 0_f32;
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(-1_f32, 0_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_x_axis_sweeping_in_y_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_x = -3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_y = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let origin_z = 0_f32;
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(1_f32, 0_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_x_axis_sweeping_in_z_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_x = 3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_y = 0_f32;
            let origin_z = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(-1_f32, 0_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_x_axis_sweeping_in_z_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_x = -3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_y = 0_f32;
            let origin_z = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(1_f32, 0_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_y_axis_sweeping_in_x_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_y = 3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let origin_z = 0_f32;
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0_f32, -1_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_y_axis_sweeping_in_x_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_y = -3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let origin_z = 0_f32;
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0_f32, 1_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_y_axis_sweeping_in_z_axis_1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_y = 3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = 0_f32;
            let origin_z = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0f32, -1_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_y_axis_swweping_in_z_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_y = -3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = 0_f32;
            let origin_z = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0_f32, 1_f32, 0_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_z_axis_sweeping_in_x_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_z = 3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let origin_y = 0_f32;
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0_f32, 0_f32, -1_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_z_axis_sweeping_in_x_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_z = -3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let origin_y = 0_f32;
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0_f32, 0_f32, 1_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_z_axis_sweeping_in_y_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_z = 3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = 0_f32;
            let origin_y = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0f32, 0_f32, -1_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_towards_sphere_along_z_axis_sweeping_in_y_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let sphere_diameter = sphere.diameter();
        let total_rays_cast = 32;
        let origin_z = -3_f32 * sphere_radius;

        for i in 1..total_rays_cast {
            let origin_x = 0_f32;
            let origin_y = sphere_radius - (i as f32) * sphere_diameter / (total_rays_cast as f32);
            let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
            let ray_direction = Vector3::new(0_f32, 0_f32, 1_f32);
            let ray = Ray::new(ray_origin, ray_direction);

            assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_some());
        }
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_y_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 3_f32 * sphere_radius;
        let origin_y = sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(-1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_y_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 3_f32 * sphere_radius;
        let origin_y = -sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(-1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }


    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_y_axis3() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -3_f32 * sphere_radius;
        let origin_y = sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_y_axis4() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -3_f32 * sphere_radius;
        let origin_y = -sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_z_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 3_f32 * sphere_radius;
        let origin_y = 0_f32;
        let origin_z = sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(-1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_z_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 3_f32 * sphere_radius;
        let origin_y = 0_f32;
        let origin_z = -sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(-1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_z_axis3() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -3_f32 * sphere_radius;
        let origin_y = 0_f32;
        let origin_z = sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_x_axis_sweeping_in_z_axis4() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -3_f32 * sphere_radius;
        let origin_y = 0_f32;
        let origin_z = -sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(1_f32, 0_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_x_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = sphere_radius;
        let origin_y = 3_f32 * sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, -1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_x_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -sphere_radius;
        let origin_y = 3_f32 * sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, -1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_x_axis3() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = sphere_radius;
        let origin_y = -3_f32 * sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_x_axis4() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -sphere_radius;
        let origin_y = -3_f32 * sphere_radius;
        let origin_z = 0_f32;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_z_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = 3_f32 * sphere_radius;
        let origin_z = sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, -1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_z_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = 3_f32 * sphere_radius;
        let origin_z = -sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, -1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_z_axis3() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = -3_f32 * sphere_radius;
        let origin_z = sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_y_axis_sweeping_in_z_axis4() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = -3_f32 * sphere_radius;
        let origin_z = -sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 1_f32, 0_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_x_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = sphere_radius;
        let origin_y = 0_f32; 
        let origin_z = 3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, -1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_x_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -sphere_radius;
        let origin_y = 0_f32; 
        let origin_z = 3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, -1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_x_axis3() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = sphere_radius;
        let origin_y = 0_f32; 
        let origin_z = -3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, 1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_x_axis4() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = -sphere_radius;
        let origin_y = 0_f32; 
        let origin_z = -3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, 1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_y_axis1() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = sphere_radius;
        let origin_z = 3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, -1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_y_axis2() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = sphere_radius;
        let origin_z = 3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, -1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_y_axis3() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = sphere_radius;
        let origin_z = -3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, 1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }

    #[test]
    fn test_rays_cast_tangent_to_sphere_along_z_axis_sweeping_in_y_axis4() {
        let sphere = sphere();
        let sphere_radius = sphere.radius();
        let origin_x = 0_f32;
        let origin_y = sphere_radius;
        let origin_z = -3_f32 * sphere_radius;
        let ray_origin = Vector3::new(origin_x, origin_y, origin_z);
        let ray_direction = Vector3::new(0_f32, 0_f32, 1_f32);
        let ray = Ray::new(ray_origin, ray_direction);

        assert!(sphere.intersect(&ray, 0.01_f32, f32::MAX).is_none());
    }
}