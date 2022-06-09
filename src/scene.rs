use crate::query::*;
use crate::geometry::*;
use crate::camera::*;
use crate::canvas::*;
use crate::scene_object::*;
use cglinalg::{
    Vector3,
};


#[derive(Copy, Clone, Debug)]
pub struct ObjectIntersectionResult<'a> {
    pub intersection_result: IntersectionResult,
    pub object: &'a SceneObject,
}

impl<'a> ObjectIntersectionResult<'a> {
    pub fn new(intersection_result: IntersectionResult, object: &'a SceneObject) -> Self {
        Self { intersection_result, object, }
    }
}

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
    pub fn ray_cast(&self, query: &IntersectionQuery) -> Option<ObjectIntersectionResult> {
        let mut closest_result = None;
        let mut t_closest_so_far = query.t_max;
        let mut closest_ray = *query;
        for object in self.objects.iter() {
            let new_ray = Ray::new(closest_ray.ray.origin, closest_ray.ray.direction);
            let new_query = IntersectionQuery::new(new_ray, closest_ray.t_min, t_closest_so_far);
            let new_intersection_result = object.intersect(&new_query);
            if let IntersectionResult::Hit(new_intersection_desc) = new_intersection_result {
                if new_intersection_desc.t < t_closest_so_far {
                    closest_ray = new_query;
                    t_closest_so_far = new_intersection_desc.t;
                    closest_result = Some(ObjectIntersectionResult::new(new_intersection_result, &object));
                }
            }
            if let IntersectionResult::Tangent(new_intersection_desc) = new_intersection_result {
                if new_intersection_desc.t < t_closest_so_far {
                    closest_ray = new_query;
                    t_closest_so_far = new_intersection_desc.t;
                    closest_result = Some(ObjectIntersectionResult::new(new_intersection_result, &object));
                }
            }
        }

        closest_result
    }
}

