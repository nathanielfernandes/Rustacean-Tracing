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

enum Rotate {
    R90,
    R180,
    R270,
}

fn load_image(image_path: &str, rotation: Rotate) -> Arc<DynamicImage> {
    let img = ImageReader::open(image_path).unwrap().decode().unwrap();
    Arc::new(match rotation {
        Rotate::R90 => img.rotate90(),
        Rotate::R180 => img.rotate180(),
        Rotate::R270 => img.rotate270(),
    })
}

fn main() {
    let lookfrom = Vec3::new(478.0, 278.0, -600.0);
    let lookat = Vec3::new(278.0, 278.0, 20.0);
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

    // world.push(Sphere::new(
    //     Vec3::new(0.0, -1000.0, -1.0),
    //     1000.0,
    //     // Lambertian::new(Image::new("bobert.png".to_string())),
    //     Metal::new(
    //         CheckerBoard::new(color!(0.0, 0.0, 0.0), color!(1.0, 1.0, 1.0), 20.0),
    //         0.2,
    //     ),
    // ));

    // world.push(Sphere::new(
    //     Vec3::new(0.0, 3.0, -1.0),
    //     0.5,
    //     EmissiveDiffuse::new(SolidColor::new(color!(1.0, 1.0, 1.0))), // Dielectric::new(1.2)
    //                                                                   //Metal::new(Image::new("earthmap.jpg".to_string()), 0.2),
    // ));

    // world.push(Sphere::new(
    //     Vec3::new(0.0, 1.0, 0.0),
    //     1.0,
    //     // Lambertian::new(Image::new("bobert.png".to_string())),
    //     Dielectric::new(1.2),
    // ));

    // (-6..6).for_each(|a| {
    //     (-6..6).for_each(|b| {
    //         let cmat = random_distribution();
    //         let center = Vec3::new(
    //             a as f64 + 0.9 * random_distribution(),
    //             0.2,
    //             b as f64 + 0.9 * random_distribution(),
    //         );

    //         if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
    //             if cmat < 0.8 {
    //                 let albedo = color!(
    //                     random_distribution(),
    //                     random_distribution(),
    //                     random_distribution()
    //                 ) * color!(
    //                     random_distribution(),
    //                     random_distribution(),
    //                     random_distribution()
    //                 );

    //                 let albedo = SolidColor::new(albedo);
    //                 // world.push(Sphere::new(center, 0.2, Lambertian::new(albedo)))
    //                 world.push(Sphere::new(center, 0.2, EmissiveDiffuse::new(albedo)))
    //             } else if cmat < 0.95 {
    //                 let albedo = color!(
    //                     random_float(0.5, 1.0),
    //                     random_float(0.5, 1.0),
    //                     random_float(0.5, 1.0)
    //                 );
    //                 let albedo = SolidColor::new(albedo);
    //                 let fuzz = random_float(0.0, 0.5);

    //                 world.push(Sphere::new(center, 0.2, Metal::new(albedo, fuzz)))
    //             } else {
    //                 world.push(Sphere::new(center, 0.2, Dielectric::new(1.5)))
    //             }
    //         }
    //     });
    // });

    (-5..5).for_each(|i| {
        (-5..5).for_each(|j| {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;

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

    // world.push(Sphere::new(
    //     Vec3::new(278.0, 140.0, 400.0),
    //     80.0,

    //     Lambertian::new(SolidColor::new(color!(0.73, 0.73, 0.73)))
    //     // EmissiveDiffuse::new(Image::new("bobert.png".to_string())), // Dielectric::new(1.2),
    //     //Dielectric::new(1.2),
    //     //Metal::new(SolidColor::new(color!(1.0, 1.0, 1.0)), 0.0),
    // ));

    // let denoise_settings = DenoiseSettings {
    //     srgb: false,
    //     hdr: true,
    //     clean_aux: true,
    // };

    // let img = camera.threaded_render(
    //     50,
    //     &world,
    //     &color!(0.7, 0.7, 0.7),
    //     800,
    //     8,
    //     50,
    //     Some(denoise_settings),
    // );
    // img.save("test.png").unwrap();

    let (albedo_img, normal_img) = camera.render_buffers(&world, &color!(0.0, 0.0, 0.0), 200);
    //albedo_img.save("albedo.png").unwrap();
    normal_img.save("normal.png").unwrap();
}
