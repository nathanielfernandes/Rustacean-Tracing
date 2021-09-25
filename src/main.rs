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

pub mod aabb;
pub mod bvh;
pub mod perlin;
pub mod texture;
pub mod vec3;
use color::*;

use materials::{Dielectric, EmissiveDiffuse, Lambertian, Metal};
use objects::{create_box, Object, Plane, PlaneType, Sphere};
use rendering::Camera;
use texture::{CheckerBoard, Image, SolidColor, Texture};
use vec3::Vec3;

use crate::bvh::BvhTree;
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
    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
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
    let bps = 20;
    (-bps..bps).for_each(|i| {
        (-bps..bps).for_each(|j| {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;

            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_float(1.0, 101.0);
            let z1 = z0 + w;

            create_box(
                Vec3::new(x0, y0, z0),
                Vec3::new(x1, y1, z1),
                Lambertian::new(SolidColor::new(color!(1.0, 0.73, 0.73))),
            )
            .into_iter()
            .for_each(|plane| world.push(plane));
        });
    });

    world.push(Plane::new(
        PlaneType::ZX,
        123.0,
        423.0,
        147.0,
        412.0,
        554.0,
        EmissiveDiffuse::new(SolidColor::new(color!(1.0, 1.0, 1.0))),
    ));

    let _dn = Some(DenoiseSettings {
        srgb: false,
        hdr: true,
        clean_aux: false,
    });

    let bvh_world = BvhTree::new(&mut world);
    let img = camera.bvh_render(&bvh_world, &color!(0.0, 0.0, 0.0), 800, 32, 3, _dn);

    // let img = camera.threaded_render(50, &world, &color!(0.0, 0.0, 0.0), 800, 32, 3, None);
    // let img = camera.pog_render(&world, &color!(0.0, 0.0, 0.0), 800, 32, 3, None);

    // let (img1, img2) = camera.bvh_render_buffers(&bvh_world, &color!(0.0, 0.0, 0.0), 800);
    img.save("test.png").unwrap();

    //let (albedo_img, normal_img) = camera.render_buffers(&world, &color!(0.0, 0.0, 0.0), 200);
    //albedo_img.save("albedo.png").unwrap();
    // normal_img.save("normal.png").unwrap();
}

// (-5..5).for_each(|i| {
//     (-5..5).for_each(|j| {
//         let w = 100.0;
//         let x0 = -1000.0 + i as f32 * w;
//         let z0 = -1000.0 + j as f32 * w;

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
