pub extern crate image;

use std::sync::Arc;

use image::io::Reader as ImageReader;
use image::{DynamicImage, ImageBuffer, RgbImage};
pub mod color;
pub mod intersection;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod rendering;

pub mod perlin;
pub mod texture;
pub mod vec3;
use color::*;

use materials::{Dielectric, EmissiveDiffuse, Lambertian, Metal};
use objects::{create_box, MovingSphere, Object, Plane, PlaneType, Sphere};
use rendering::Camera;
use texture::{CheckerBoard, Image, SolidColor, Texture};
use vec3::Vec3;

use crate::rendering::{random_distribution, random_float, DenoiseSettings};

use oidn;

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

#[allow(dead_code)]
enum Rotate {
    R90,
    R180,
    R270,
}

#[allow(dead_code)]
fn load_image(image_path: &str, rotation: Rotate) -> Arc<DynamicImage> {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    Arc::new(match rotation {
        Rotate::R90 => img.rotate90(),
        Rotate::R180 => img.rotate180(),
        Rotate::R270 => img.rotate270(),
    })
}

fn main() {
    // let bobert_png: Arc<DynamicImage> = load_image("bobert.png", Rotate::R270);
    // let bobert_png_1: Arc<DynamicImage> = load_image("bobert.png", Rotate::R180);

    let lookfrom = Vec3::new(278.0, 278.0, -800.0);
    let lookat = Vec3::new(278.0, 278.0, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        1.0, //16.0 / 9.0,
        0.0,
        100.0,
        0.0,
        1.0,
    );

    let mut world: Vec<Object> = Vec::new();

    //green
    world.push(Plane::new(
        PlaneType::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Lambertian::new(SolidColor::new(color!(0.12, 0.45, 0.15))),
        //Lambertian::new(Image::new(bobert_png_1.clone())),
    ));

    //red
    world.push(Plane::new(
        PlaneType::YZ,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Lambertian::new(SolidColor::new(color!(0.65, 0.05, 0.05))),
        //Lambertian::new(Image::new(bobert_png_1.clone())),
    ));

    //white walls
    world.push(Plane::new(
        PlaneType::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        0.0,
        Lambertian::new(SolidColor::new(color!(0.73, 0.73, 0.73))),
        //Lambertian::new(Image::new(bobert_png_1.clone())),
    ));

    world.push(Plane::new(
        PlaneType::ZX,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Lambertian::new(SolidColor::new(color!(0.73, 0.73, 0.73))),
        //Lambertian::new(Image::new(bobert_png_1.clone())),
    ));

    world.push(Plane::new(
        PlaneType::XY,
        0.0,
        555.0,
        0.0,
        555.0,
        555.0,
        Lambertian::new(SolidColor::new(color!(0.73, 0.73, 0.73))),
        //Lambertian::new(Image::new(bobert_png.clone())), //Lambertian::new(SolidColor::new(color!(0.73, 0.73, 0.73))),
    ));

    //light
    world.push(Plane::new(
        PlaneType::ZX,
        213.0,
        343.0,
        227.0,
        332.0,
        554.0,
        EmissiveDiffuse::new(SolidColor::new(color!(15.0, 15.0, 15.0))),
    ));

    create_box(
        Vec3::new(130.0, 0.0, 65.0),
        Vec3::new(295.0, 165.0, 230.0),
        Lambertian::new(SolidColor::new(color!(0.73, 0.73, 0.73))),
    )
    .into_iter()
    .for_each(|plane| world.push(plane));

    let img = camera.threaded_render(50, &world, &color!(0.0, 0.0, 0.0), 800, 2, 50, None);
    img.save("test.png").unwrap();

    //let (albedo_img, normal_img) = camera.render_buffers(&world, &color!(0.0, 0.0, 0.0), 200);
    //albedo_img.save("albedo.png").unwrap();
    // normal_img.save("normal.png").unwrap();
}

// (-5..5).for_each(|i| {
//     (-5..5).for_each(|j| {
//         let w = 100.0;
//         let x0 = -1000.0 + i as f64 * w;
//         let z0 = -1000.0 + j as f64 * w;

//         let y0 = 0.0;
//         let x1 = x0 + w;
//         let y1 = random_float(1.0, 101.0);
//         let z1 = z0 + w;

//         create_box(
//             Vec3::new(x0, y0, z0),
//             Vec3::new(x1, y1, z1),
//             Lambertian::new(SolidColor::new(color!(1.0, 0.73, 0.73))),
//         )
//         .into_iter()
//         .for_each(|plane| world.push(plane));
//     });
// });
