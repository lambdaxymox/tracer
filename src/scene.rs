use crate::ray::Ray;
use crate::material::{Hitable, IntersectionRecord};


pub struct Scene {
    objects: Vec<Box<dyn Hitable>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn len_objects(&self) -> usize {
        self.objects.len()
    }

    pub fn push(&mut self, item: Box<dyn Hitable>) {
        self.objects.push(item);
    }
}

impl Hitable for Scene {
    fn intersect(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<IntersectionRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for object in self.objects.iter() {
            match object.intersect(ray, t_min, closest_so_far) {
                Some(temp_record) => {
                    if temp_record.t < closest_so_far {
                        closest_so_far = temp_record.t;
                        closest_record = Some(temp_record);
                    }
                }
                None => {}
            }
        }

        closest_record
    }
}
