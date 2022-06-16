use crate::light::*;
use cglinalg::{
    Matrix4x4,
    Vector3,
};


#[derive(Debug)]
pub struct ModelSpacePointLightObject {
    light: PointLight,
}

impl ModelSpacePointLightObject {
    pub fn new(light: PointLight) -> Self {
        Self { light, }
    }
}

#[derive(Debug)]
pub struct ScenePointLightObject {
    light: PointLight,
    model_matrix: Matrix4x4<f32>,
    model_matrix_inv: Matrix4x4<f32>,
}

impl ScenePointLightObject {
    pub fn new(light: PointLight, model_matrix: Matrix4x4<f32>) -> Self {
        Self { 
            light, 
            model_matrix, 
            model_matrix_inv: model_matrix.inverse().unwrap(), 
        }
    }

    #[inline]
    pub fn position(&self) -> Vector3<f32> {
        (self.model_matrix * Vector3::zero().extend(1_f32)).contract()
    }

    #[inline]
    pub fn emission(&self) -> Vector3<f32> {
        self.light.emission
    }
}

