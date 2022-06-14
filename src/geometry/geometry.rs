use crate::query::*;
use cglinalg::{
    Vector3,
};

pub trait Intersection {
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult;
}
pub trait Geometry: std::fmt::Debug + Intersection {
    fn center(&self) -> Vector3<f32>;

    fn contains(&self, point: &Vector3<f32>) -> bool;
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Reason {
    HitBeforeMin { t_got: f32 },
    HitBeforeMax { t_got: f32 },
    NoIntersection,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum IntersectionResult {
    Hit(IntersectionData),
    Tangent(IntersectionData),
    Miss(Reason),
}

impl IntersectionResult {
    pub fn new_hit(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self::Hit(IntersectionData::new(t, point, normal))
    }

    pub fn new_tangent(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        Self::Tangent(IntersectionData::new(t, point, normal))
    }

    pub fn new_miss(reason: Reason) -> Self {
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

    #[inline]
    pub fn unwrap_hit(&self) -> IntersectionData {
        match self {
            Self::Hit(value) => *value,
            _ => panic!("`IntersectionResult::unwrap_hit()` got a `Tangent` or `Miss` value`"),
        }
    }

    #[inline]
    pub fn unwrap_tangent(&self) -> IntersectionData {
        match self {
            Self::Tangent(value) => *value,
            _ => panic!("`IntersectionResult::unwrap_tangent()` got a `Hit` or `Miss` value`"),
        }
    }

    #[inline]
    pub fn unwrap_hit_or_tangent(&self) -> IntersectionData {
        match self {
            Self::Hit(value) | Self::Tangent(value) => *value,
            _ => panic!("`IntersectionResult::unwrap_hit_or_tangent()` got a `Miss` value`"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntersectionData {
    pub t: f32,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
}

impl<'a> IntersectionData {
    pub fn new(t: f32, point: Vector3<f32>, normal: Vector3<f32>) -> Self {
        IntersectionData { t, point, normal, }
    }
}

