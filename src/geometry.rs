use crate::query::*;
use cglinalg::{
    Vector3,
};

pub trait Intersection {
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult;
}
pub trait Geometry: std::fmt::Debug + Intersection {
    fn center(&self) -> Vector3<f32>;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reason {
    HitBeforeMin { t_got: f32 },
    HitBeforeMax { t_got: f32 },
    NoIntersection,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IntersectionResult {
    Hit(IntersectionDesc),
    Tangent(IntersectionDesc),
    Miss(Reason),
}

impl IntersectionResult {
    pub fn new_hit(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self::Hit(IntersectionDesc::new(t, point, normal))
    }

    pub fn new_tangent(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self::Tangent(IntersectionDesc::new(t, point, normal))
    }

    #[inline]
    pub const fn new_miss(reason: Reason) -> Self {
        Self::Miss(reason)
    }

    #[inline]
    pub fn is_hit(&self) -> bool {
        match *self {
            Self::Hit(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_tangent(&self) -> bool {
        match *self {
            Self::Tangent(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_miss(&self) -> bool {
        match *self {
            Self::Miss(_) => true,
            _ => false
        }
    }

    #[inline]
    pub fn is_hit_or_tangent(&self) -> bool {
        match self {
            Self::Hit(_) | Self::Tangent(_) => true,
            _ => false,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntersectionDesc {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl<'a> IntersectionDesc {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        IntersectionDesc { t, point, normal, }
    }
}

