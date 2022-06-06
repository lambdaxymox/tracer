use crate::ray::Ray;
use crate::material::{Hitable, HitRecord};


pub struct HitableList {
    items: Vec<Box<dyn Hitable>>,
}

impl HitableList {
    pub fn new() -> HitableList {
        HitableList {
            items: Vec::new(),
        }
    }

    pub fn len(&self) -> usize {
        self.items.len()
    }

    pub fn push(&mut self, item: Box<dyn Hitable>) {
        self.items.push(item);
    }
}

impl Hitable for HitableList {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let mut closest_record = None;
        let mut closest_so_far = t_max;
        for item in self.items.iter() {
            match item.hit(ray, t_min, closest_so_far) {
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
