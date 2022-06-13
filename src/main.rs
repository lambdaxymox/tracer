extern crate cglinalg;
extern crate rand;


mod query;
mod scene;
mod model_object;
mod scene_object;
mod canvas;
mod camera;
mod geometry;
mod bsdf;
mod renderer;
mod sampler;
mod sphere;

use rand::prelude::*;

use std::fs::File;
use std::io;
use std::io::Write;
use std::f32;


use cglinalg::{
    Vector3,
    Matrix4x4,
    Magnitude,
};
use camera::{
    Camera
};
use canvas::*;
use sphere::*;
use sampler::*;
use model_object::*;
use scene_object::*;
use scene::*;
use renderer::*;
use bsdf::*;


const SAMPLES_PER_PIXEL: usize = 128;
const MAX_DEPTH: usize = 16;


fn camera(width: usize, height: usize) -> Camera {
    let look_from = Vector3::new(12_f32, 2_f32, 4_f32);
    let look_at = Vector3::new(0_f32, 0_f32, 0_f32);
    let distance_to_focus = (look_from - look_at).magnitude();
    let aperture = 0.1_f32;
    let v_up = Vector3::new(0_f32, 1_f32, 0_f32);
    let v_fov = 20_f32;
    let aspect_ratio = (width as f32) / (height as f32);
    
    Camera::new(look_from, look_at, v_up, v_fov, aspect_ratio, aperture, distance_to_focus)
}

fn generate_scene(rng: &mut ThreadRng, width: usize, height: usize) -> Scene {
    let camera = camera(width, height);
    let mut scene = Scene::new(width, height, camera);
    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
            Sphere::new(Vector3::zero(), 1000_f32),
            Box::new(SimpleLambertianBsdf::new(Vector3::new(0.5, 0.5, 0.5))),
            Box::new(SimpleLambertianBsdfQuerySampler::new())
        )),
        Matrix4x4::from_affine_translation(&Vector3::new(0_f32, -1000_f32, 0_f32))
    ));
    
    for a in -10..10 {
        for b in -10..10 {
            let choose_mat = rng.gen::<f32>();
            let center_x = a as f32 + 0.9 * rng.gen::<f32>();
            let center_y = 0.2;
            let center_z = b as f32 + 0.9 * rng.gen::<f32>();
            let center = Vector3::new(center_x, center_y, center_z);
            let scene_center =  Vector3::new(4_f32, 2_f32, 0_f32);
            if (center - scene_center).magnitude() > 0.9 {
                if choose_mat < 0.8 {
                    // Lambertian (diffuse).
                    let albedo = Vector3::new(
                        rng.gen::<f32>() * rng.gen::<f32>(), 
                        rng.gen::<f32>() * rng.gen::<f32>(), 
                        rng.gen::<f32>() * rng.gen::<f32>()
                    );
                    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
                            Sphere::new(Vector3::zero(), 0.2),
                            Box::new(SimpleLambertianBsdf::new(albedo)),
                            Box::new(SimpleLambertianBsdfQuerySampler::new())
                        )),
                        Matrix4x4::from_affine_translation(&center)
                    ));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = Vector3::new(
                        0.5 * (1_f32 + rng.gen::<f32>()), 
                        0.5 * (1_f32 + rng.gen::<f32>()), 
                        0.5 * (1_f32 + rng.gen::<f32>())
                    );
                    let fuzz = 0.5 * rng.gen::<f32>();
                    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
                            Sphere::new(Vector3::zero(), 0.2),
                            Box::new(SimpleMetalBsdf::new(albedo, fuzz)),
                            Box::new(SimpleMetalBsdfQuerySampler::new())
                        )),
                        Matrix4x4::from_affine_translation(&center)
                    ));
                } else {
                    // Glass.
                    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
                            Sphere::new(Vector3::zero(), 0.2),
                            Box::new(SimpleDielectricBsdf::new(1.5)),
                            Box::new(SimpleDielectricBsdfQuerySampler::new())
                        )),
                        Matrix4x4::from_affine_translation(&center)
                    ));
                }
            }
        }
    }

    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
            Sphere::new(Vector3::zero(), 1_f32),
            Box::new(SimpleDielectricBsdf::new(1.5)),
            Box::new(SimpleDielectricBsdfQuerySampler::new())
        )),
        Matrix4x4::from_affine_translation(&Vector3::new(0_f32, 1_f32, 0_f32))
    ));
    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
            Sphere::new(Vector3::zero(), 1_f32), 
            Box::new(SimpleLambertianBsdf::new(Vector3::new(0.4, 0.2, 0.1))),
            Box::new(SimpleLambertianBsdfQuerySampler::new())
        )),
        Matrix4x4::from_affine_translation(&Vector3::new(-4_f32, 1_f32, 0_f32))
    ));
    scene.push(SceneObject::new(Box::new(SphereModelObject::new(
            Sphere::new(Vector3::zero(), 1_f32), 
            Box::new(SimpleMetalBsdf::new(Vector3::new(0.7, 0.6, 0.5), 0.1)),
            Box::new(SimpleMetalBsdfQuerySampler::new())
        )),
        Matrix4x4::from_affine_translation(&Vector3::new(4_f32, 1_f32, 0_f32))
    ));

    scene
}


fn write_image_to_file(canvas: &Canvas, file: &mut File) -> io::Result<()> {
    write!(file, "P3\n{} {}\n255\n", canvas.width, canvas.height).unwrap();
    for pixel in canvas.data.iter() {
        writeln!(file, "{} {} {}", pixel.r, pixel.g, pixel.b).unwrap();
    }

    Ok(())
}

fn main() -> io::Result<()> {
    let width = 480;
    let height = 270;
    let mut canvas = Canvas::new(width, height);
    let mut rng = rand::prelude::thread_rng();
    let settings = RendererSettings::new(SAMPLES_PER_PIXEL, MAX_DEPTH);
    let renderer = Renderer::new(settings);

    println!("Generating scene.");
    let mut scene = generate_scene(&mut rng, width, height);

    println!("Generating image.");
    let mut sampler = SphereSampler::new(rand::prelude::thread_rng());
    renderer.render(&mut scene, &mut sampler, &mut canvas);
    
    println!("Writing image to file.");
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&canvas, &mut file)
}

