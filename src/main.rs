pub extern crate image;
//use image::ImageBuffer;

pub mod color;
pub mod intersection;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod rendering;

pub mod vec3;
use color::*;
use materials::{Dielectric, Lambertian, Metal};
use objects::{Object, Sphere};
use rendering::Camera;
use vec3::Vec3;

//use crate::rendering::{random_distribution, random_float};

macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

fn main() {
    let lookfrom = Vec3::new(0.0, 2.0, 10.0);
    let lookat = Vec3::new(0.0, 1.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        16.0 / 9.0,
        0.0,
        100.0,
    );

    let mut all_objs: Vec<Object> = Vec::new();

    all_objs.push(Sphere::new(
        Vec3::new(0.0, -1000.0, -1.0),
        1000.0,
        Lambertian::new(color!(0.5, 0.5, 0.5)),
    ));

    all_objs.push(Sphere::new(
        Vec3::new(1.0, 1.0, -1.0),
        1.0,
        Dielectric::new(1.2),
    ));

    all_objs.push(Sphere::new(
        Vec3::new(-1.0, 1.0, -1.0),
        1.0,
        Metal::new(color!(0.0, 1.0, 1.0), 0.2),
    ));

    let img = camera.threaded_render(90, &all_objs, 1920, 128, 50);
    img.save("test.png").unwrap();
}
