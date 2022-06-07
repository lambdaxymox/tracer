use crate::ray::Ray;
use crate::material::{Intersect, IntersectionRecord};
use crate::camera::*;
use std::ops;


#[derive(Copy, Clone, Debug, PartialEq, Eq, Hash)]
pub struct Rgba {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Rgba {
    #[inline]
    pub fn new(r: u8, g: u8, b: u8) -> Rgba {
        Rgba { 
            r, b, g,
        }
    }

    #[inline]
    pub const fn zero() -> Rgba {
        Rgba { r: 0, g: 0, b: 0 }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Rgba>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self { 
            width, height, data: vec![Rgba::zero(); width * height]
        }
    }

    pub fn clear(&mut self) {
        let zero = Rgba::zero();
        for pixel in self.data.as_mut_slice() {
            *pixel = zero;
        }
    }
}

impl ops::Index<usize> for Canvas {
    type Output = [Rgba];

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        let row_start = index * self.width;
        let row_end = row_start + self.width;

        &self.data[row_start..row_end]
    }
}

impl ops::IndexMut<usize> for Canvas {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let row_start = index * self.width;
        let row_end = row_start + self.width;

        &mut self.data[row_start..row_end]
    }
}

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
