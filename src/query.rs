use cglinalg::{
    Vector3,
};

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Ray {
    pub origin: Vector3<f32>,
    pub direction: Vector3<f32>,
}

impl Ray {
    pub fn new(origin: Vector3<f32>, direction: Vector3<f32>) -> Self {
        Ray { origin, direction }
    }

    pub fn interpolate(&self, t: f32) -> Vector3<f32> {
        self.origin + self.direction * t
    }
}


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct IntersectionQuery {
    pub ray: Ray,
    pub t_min: f32,
    pub t_max: f32,
}

impl IntersectionQuery {
    pub fn new(ray: Ray, t_min: f32, t_max: f32) -> Self {
        Self { ray, t_min, t_max }
    }
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


#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScatteringQuery {
    pub ray_incoming: Vector3<f32>,
    pub point: Vector3<f32>,
}

impl ScatteringQuery {
    pub fn new(ray_incoming: Vector3<f32>, point: Vector3<f32>) -> Self {
        Self { ray_incoming, point, }
    }
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct ScatteringResult {
    pub ray_incoming: Vector3<f32>,
    pub ray_outgoing: Vector3<f32>,
    pub point: Vector3<f32>,
    pub normal: Vector3<f32>,
    pub scattering_fraction: Vector3<f32>,
}

impl ScatteringResult {
    pub fn new(
        ray_incoming: Vector3<f32>,
        ray_outgoing: Vector3<f32>,
        point: Vector3<f32>,
        normal: Vector3<f32>,
        scattering_fraction: Vector3<f32>) -> Self 
    {
        Self {
            ray_incoming, ray_outgoing, point, normal, scattering_fraction,
        }
    }
}

