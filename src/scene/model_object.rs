use crate::bsdf::*;
use crate::geometry::*;
use crate::light::*;
use crate::query::*;
use crate::sampler::*;
use cglinalg::{
    Vector3,
    Magnitude,
};


pub trait ModelSpaceObject: std::fmt::Debug {
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult;

    fn center(&self) -> Vector3<f32>;

    fn contains(&self, point: &Vector3<f32>) -> bool;

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>>;
}

pub trait ModelSpaceScatteringObject: ModelSpaceObject {
    fn scatter(&self, query: &ScatteringQuery, sampler: &mut SphereSampler) -> ScatteringResult;
}

pub trait ModelSpaceLightingObject: ModelSpaceObject {
    fn emit(&self, query: &LightingQuery) -> LightingResult;
}

#[derive(Debug)]
pub struct ModelSpaceGeometryScatteringObject<Geom, Bsdf> 
where 
    Geom: Geometry,
    Bsdf: BsdfMapping,
{
    geometry: Geom,
    bsdf: Box<Bsdf>,
    sampler: Box<dyn BsdfQuerySampler<Bsdf = Bsdf>>,
}

impl<Geom, Bsdf> ModelSpaceGeometryScatteringObject<Geom, Bsdf> 
where 
    Geom: Geometry,
    Bsdf: BsdfMapping, 
{
    pub fn new(geometry: Geom, bsdf: Box<Bsdf>, sampler: Box<dyn BsdfQuerySampler<Bsdf = Bsdf>>) -> Self {
        Self { geometry, bsdf, sampler, }
    }
}

impl<Geom, Bsdf> ModelSpaceObject for ModelSpaceGeometryScatteringObject<Geom, Bsdf>
where 
    Geom: Geometry,
    Bsdf: BsdfMapping,
{
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        self.geometry.intersect(query)
    }

    fn center(&self) -> Vector3<f32> {
        self.geometry.center()
    }

    #[inline]
    fn contains(&self, point: &Vector3<f32>) -> bool {
        self.geometry.contains(point)
    }

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        if self.contains(point) {
            Some((point - self.geometry.center()).normalize())
        } else {
            None
        }
    }
}

impl<Geom, Bsdf> ModelSpaceScatteringObject for ModelSpaceGeometryScatteringObject<Geom, Bsdf>
where 
    Geom: Geometry,
    Bsdf: BsdfMapping,
{
    fn scatter(&self, query: &ScatteringQuery, sampler: &mut SphereSampler) -> ScatteringResult {
        let normal = (query.point - self.geometry.center()).normalize();
        let ray_incoming = query.ray_incoming;
        let bsdf_query = self.sampler.sample(&self.bsdf, &ray_incoming, &normal, &query.point, sampler);
        let bsdf_result = self.bsdf.sample(&bsdf_query);
             
        ScatteringResult::new(
            bsdf_result.ray_incoming,
            bsdf_result.ray_outgoing,
            bsdf_result.point,
            bsdf_result.normal,
            bsdf_result.scattering_fraction,
        )
    }
}

#[derive(Debug)]
pub struct ModelSpaceGeometryLightObject<Geom, Light> 
where 
    Geom: Geometry,
    Light: LightMapping,
{
    geometry: Geom,
    light: Light,
}

impl<Geom, Light> ModelSpaceGeometryLightObject<Geom, Light> 
where 
    Geom: Geometry,
    Light: LightMapping,
{
    pub fn new(geometry: Geom, light: Light) -> Self {
        Self { geometry, light, }
    }
}

impl<Geom, Light> ModelSpaceObject for ModelSpaceGeometryLightObject<Geom, Light>
where 
    Geom: Geometry,
    Light: LightMapping,
{
    fn intersect(&self, query: &IntersectionQuery) -> IntersectionResult {
        self.geometry.intersect(query)
    }

    fn center(&self) -> Vector3<f32> {
        self.geometry.center()
    }

    #[inline]
    fn contains(&self, point: &Vector3<f32>) -> bool {
        self.geometry.contains(point)
    }

    fn normal(&self, point: &Vector3<f32>) -> Option<Vector3<f32>> {
        if self.contains(point) {
            Some((point - self.geometry.center()).normalize())
        } else {
            None
        }
    }
}

impl<Geom, Light> ModelSpaceLightingObject for ModelSpaceGeometryLightObject<Geom, Light> 
where
    Geom: Geometry,
    Light: LightMapping,
{
    fn emit(&self, query: &LightingQuery) -> LightingResult {
        self.light.emit(query)
    }
}

