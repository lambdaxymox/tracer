use crate::ray::Ray;
use crate::material::*;
use crate::camera::*;
use crate::canvas::*;
use std::ops;


pub struct Scene {
    pub objects: Vec<Box<dyn Intersect>>,
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

    pub fn push(&mut self, item: Box<dyn Intersect>) {
        self.objects.push(item);
    }
}

impl Intersect for Scene {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord> {
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
