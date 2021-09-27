pub extern crate image;

use std::path::Path;
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
// pub mod bvh2;
pub mod perlin;
pub mod texture;
pub mod vec3;
use color::*;

use materials::{Dielectric, EmissiveDiffuse, Isotropic, Lambertian, Metal};
use objects::{create_box, Object, Plane, PlaneType, Sphere};
use rendering::Camera;
use texture::{CheckerBoard, SolidColor, Texture};
use vec3::Vec3;

use crate::bvh::BvhTree;

use crate::objects::{load_obj, BoxObj, ConstantMedium};
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
    let lookfrom = Vec3::new(0.0, 11.5, 10.0);
    let lookat = Vec3::new(0.0, 0.5, 0.0);
    let camera = Camera::new(
        lookfrom,
        lookat,
        Vec3::new(0.0, 1.0, 0.0),
        40.0,
        16.0 / 9.0,
        0.0,
        100.0,
        0.0,
        1.0,
    );
    let mut world: Vec<Object> = Vec::new();

    // !Floor
    // world.push(Sphere::new(
    //     Vec3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     // Lambertian::new(Image::new("bobert.png".to_string())),
    //     Lambertian::new(CheckerBoard::new(
    //         color!(1.0, 0.1, 0.1),
    //         color!(0.5, 0.1, 0.1),
    //         5.0,
    //     )),
    // ));
    // // !Floor
    // world.push(Plane::new(
    //     PlaneType::ZX,
    //     -20.0,
    //     20.0,
    //     -20.0,
    //     20.0,
    //     0.0,
    //     Lambertian::new(SolidColor::new(color!(1.0, 0.1, 0.1))),
    // ));

    // !Block
    world.push(BoxObj::new(
        Vec3::new(-7.0, 0.0, -5.0),
        Vec3::new(7.0, 0.5, 4.0),
        Lambertian::new(SolidColor::new(color!(1.0, 0.1, 0.1))),
    ));

    let teapot = load_obj(
        Path::new("./teapot.obj"),
        Vec3::new(0.0, 0.5, 0.0),
        0.45,
        Lambertian::new(SolidColor::new(WHITE)),
    );
    teapot.into_iter().for_each(|t| world.push(t));

    world.push(Sphere::new(
        Vec3::new(-4.8, 1.5, -0.6),
        1.0,
        Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.01),
    ));

    world.push(Sphere::new(
        Vec3::new(-2.7, 1.2, 2.45),
        0.7,
        Lambertian::new(SolidColor::new(color!(1.0, 1.0, 1.0))),
    ));

    world.push(Sphere::new(
        Vec3::new(4.6, 1.6, 0.0),
        1.1,
        Dielectric::new(1.4), // Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.01),
    ));

    world.push(ConstantMedium::new(
        Sphere::new(
            Vec3::new(2.2, 1.35, -2.15),
            0.85,
            Isotropic::new(SolidColor::new(color!(
                255.0 / 255.0,
                228.0 / 255.0,
                140.0 / 255.0
            ))),
        ),
        100.0, // Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.01),
    ));

    world.push(
        Sphere::new(Vec3::new(2.2, 1.35, -2.15), 0.85, Dielectric::new(1.8)),
        // Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.01),
    );

    // world.push(BoxObj::new(
    //     Vec3::new(-2.0, 0.5, 0.0),
    //     Vec3::new(-1.5, 1.0, 0.5),
    //     EmissiveDiffuse::new(SolidColor::new(color!(
    //         255.0 / 100.0,
    //         228.0 / 100.0,
    //         140.0 / 100.0
    //     ))),
    // ));

    // world.push(BoxObj::new(
    //     Vec3::new(0.5, 0.5, -0.5),
    //     Vec3::new(1.0, 1.0, 0.0),
    //     EmissiveDiffuse::new(SolidColor::new(color!(
    //         255.0 / 100.0,
    //         255.0 / 100.0,
    //         100.0 / 100.0
    //     ))),
    // ));

    let lights = 8;
    let d = 0.8;
    let w = 0.4;
    let s = 0.2;
    (-lights..lights).for_each(|i| {
        world.push(Plane::new(
            PlaneType::ZX,
            -2.0,
            1.1,
            (i as f32 * d) + s,
            (i as f32 * d) + w + s,
            4.3,
            EmissiveDiffuse::new(SolidColor::new(color!(4.8, 4.8, 4.0))),
        ))
    });

    let blocks = 36;
    let w = 0.5;
    let mut flip = true;
    (-blocks..blocks).for_each(|y| {
        (-blocks..blocks).for_each(|x| {
            let center = Vec3::new(x as f32 * w, random_float(-0.8, -0.3), y as f32 * w);

            let x1 = center.x + w;
            let y1 = center.y + w;
            let z1 = center.z + w;

            let mat = if flip {
                Lambertian::new(SolidColor::new(color!(1.0, 0.1, 0.1)))
            } else {
                Lambertian::new(SolidColor::new(color!(0.5, 0.1, 0.1)))
            };

            world.push(BoxObj::new(center, Vec3::new(x1, y1, z1), mat));
            flip = !flip;
        });
        flip = !flip;
    });

    let _dn = Some(DenoiseSettings {
        srgb: false,
        hdr: true,
        clean_aux: false,
    });

    let bvh_world = BvhTree::new(&mut world);
    let img = camera.bvh_render(&bvh_world, &color!(0.0, 0.0, 0.0), 3840, 128, 50, _dn);
    img.save("test.png").unwrap();
    // let img = camera.threaded_render(50, &world, &color!(0.0, 0.0, 0.0), 800, 32, 3, None);
    // let img = camera.pog_render(&world, &color!(0.0, 0.0, 0.0), 1000, 128, 50, None);

    // let (img1, img2) = camera.bvh_render_buffers(&bvh_world, &color!(0.0, 0.0, 0.0), 400);
    // img2.save("normal.png").unwrap();
    // img1.save("albedo.png").unwrap();

    // let (albedo_img, normal_img) = camera.render_buffers(&world, &color!(0.0, 0.0, 0.0), 200);
    // albedo_img.save("albedo.png").unwrap();
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

// let lookfrom = Vec3::new(478.0, 278.0, -600.0);
// let lookat = Vec3::new(278.0, 278.0, 0.0);
// let camera = Camera::new(
//     lookfrom,
//     lookat,
//     Vec3::new(0.0, 1.0, 0.0),
//     40.0,
//     1.0, //16.0 / 9.0,
//     0.0,
//     100.0,
//     0.0,
//     1.0,
// );

// let mut world: Vec<Object> = Vec::new();
// let bps = 2;
// (-bps..bps).for_each(|i| {
//     (-bps..bps).for_each(|j| {
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

// world.push(Sphere::new(
//     Vec3::new(2.2, 1.35, -2.15),
//     0.852,
//     Dielectric::new(2.0), // Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.01),
// ));

// world.push(ConstantMedium::new(
//     Sphere::new(
//         Vec3::new(2.2, 1.35, -2.15),
//         0.845,
//         Isotropic::new(SolidColor::new(color!(255.0 / 255.0, 132.0 / 255.0, 0.0))), // Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.01),
//     ),
//     0.2,
// ));
