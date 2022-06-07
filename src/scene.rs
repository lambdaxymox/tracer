use crate::ray::Ray;
use crate::material::*;
use crate::camera::*;
use crate::canvas::*;
use crate::scene_object::*;


pub struct Scene {
    pub objects: Vec<Box<SceneObject>>,
    pub canvas: Canvas,
    pub camera: Camera,
}

impl Scene {
    pub fn new(width: usize, height: usize, camera: Camera) -> Scene {
        Scene {
            objects: Vec::new(),
            canvas: Canvas::new(width, height),
            camera: camera,
        }
    }

    pub fn len_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn push(&mut self, object: SceneObject) {
        self.objects.push(Box::new(object));
    }
}

impl Scene {
    pub fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionResult> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            if let Some(temp_record) = object.intersect(ray, t_min, closest_so_far) {
                if temp_record.t < closest_so_far {
                    closest_so_far = temp_record.t;
                    closest_record = Some(temp_record);
                }
            }
        }

        closest_record
    }
}
