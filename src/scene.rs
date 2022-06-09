use crate::ray::Ray;
use crate::camera::*;
use crate::canvas::*;
use crate::scene_object::*;


pub struct Scene {
    pub objects: Vec<SceneObject>,
    pub canvas: Canvas,
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: usize, height: usize, camera: Camera) -> Scene {
        Scene {
            objects: Vec::new(),
            canvas: Canvas::new(width, height),
            camera,
        }
    }

    /// Returns the number of objects in a scene.
    pub fn len_objects(&self) -> usize {
        self.objects.len()
    }

    /// Determine whether a scene contains any objects.
    pub fn is_empty_objects(&self) -> bool {
        self.objects.is_empty()
    }

    /// Insert an object into a scene.
    pub fn push(&mut self, object: SceneObject) {
        self.objects.push(object);
    }

    /// Cast a ray into a scene and determine whether the ray intersects and 
    /// object inside the scene.
    pub fn ray_cast(&self, ray: &Ray) -> Option<ObjectIntersectionResult> {
        let mut closest_result = None;
        let mut closest_so_far = ray.t_max;
        let mut closest_ray = *ray;
        for object in self.objects.iter() {
            let new_ray = Ray::new(closest_ray.origin, closest_ray.direction, ray.t_min, closest_so_far);
            if let Some(new_result) = object.intersect(&new_ray) {
                if new_result.t < closest_so_far {
                    closest_ray = new_ray;
                    closest_so_far = new_result.t;
                    closest_result = Some(new_result);
                }
            }
        }

        closest_result
    }
}

