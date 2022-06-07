extern crate cglinalg;
extern crate rand;


mod ray;
mod scene;
mod scene_object;
mod canvas;
mod camera;
mod material;
mod renderer;
mod sample;

use rand::prelude::*;

use std::fs::File;
use std::io;
use std::io::Write;
use std::f32;


use cglinalg::{
    Vector3,
    Magnitude,
};
use camera::{
    Camera
};
use canvas::*;
use scene_object::*;
use scene::*;
use renderer::*;
use material::*;


const SAMPLES_PER_PIXEL: u32 = 32;


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
    scene.push(SceneObject::new(
        Box::new(Sphere::new(
            Vector3::new(0_f32, -1000_f32, 0_f32), 
            1000_f32
        )), 
        Box::new(SimpleLambertianMaterial::new(Vector3::new(0.5, 0.5, 0.5)))
    ));
    
    for a in -5..5 {
        for b in -5..5 {
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
                    scene.push(SceneObject::new(
                        Box::new(Sphere::new(center, 0.2)),
                        Box::new(SimpleLambertianMaterial::new(albedo))
                    ));
                } else if choose_mat < 0.95 {
                    // Metal.
                    let albedo = Vector3::new(
                        0.5 * (1_f32 + rng.gen::<f32>()), 
                        0.5 * (1_f32 + rng.gen::<f32>()), 
                        0.5 * (1_f32 + rng.gen::<f32>())
                    );
                    let fuzz = 0.5 * rng.gen::<f32>();
                    scene.push(SceneObject::new(
                        Box::new(Sphere::new(center, 0.2)), 
                        Box::new(SimpleMetalMaterial::new(albedo, fuzz))
                    ));
                } else {
                    // Glass.
                    scene.push(SceneObject::new(
                        Box::new(Sphere::new(center, 0.2)),
                        Box::new(SimpleDielectricMaterial::new(1.5))
                    ));
                }
            }
        }
    }

    scene.push(SceneObject::new(
        Box::new(Sphere::new(
            Vector3::new(0_f32, 1_f32, 0_f32), 
            1_f32
        )),
        Box::new(SimpleDielectricMaterial::new(1.5))
    ));
    scene.push(SceneObject::new(
        Box::new(Sphere::new(
            Vector3::new(-4_f32, 1_f32, 0_f32), 
            1_f32
        )), 
        Box::new(SimpleLambertianMaterial::new(Vector3::new(0.4, 0.2, 0.1)))
    ));
    scene.push(SceneObject::new(
        Box::new(Sphere::new(
            Vector3::new(4_f32, 1_f32, 0_f32), 
            1_f32
        )), 
        Box::new(SimpleMetalMaterial::new(Vector3::new(0.7, 0.6, 0.5), 0.1))
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
    let samples_per_pixel = SAMPLES_PER_PIXEL;
    let mut rng = rand::prelude::thread_rng();
    let renderer = Renderer::new(samples_per_pixel);

    println!("Generating scene.");
    let mut scene = generate_scene(&mut rng, width, height);

    println!("Generating image.");
    renderer.render(&mut scene);
    
    println!("Writing image to file.");
    let mut file = File::create("output.ppm").unwrap();
    write_image_to_file(&scene.canvas, &mut file)
}

