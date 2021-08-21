extern crate image;
pub mod color;
pub mod intersection;
pub mod lights;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod scene;
pub mod vec3;

use color::*;
use ray::Ray;
// use image::{ImageBuffer, RgbImage};
use objects::{Plane, Sphere};
use scene::Scene;
use vec3::Vec3;

use crate::{
    lights::{DirectionalLight, SphericalLight},
    materials::Material,
};

macro_rules! color {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            r: $r,
            g: $g,
            b: $b,
        }
    };
}

macro_rules! rgb {
    ($r:expr, $g:expr, $b:expr) => {
        Color {
            r: $r / 255.0,
            g: $g / 255.0,
            b: $b / 255.0,
        }
    };
}

macro_rules! vec3 {
    ($x:expr, $y:expr, $z:expr) => {
        Vec3 {
            x: $x,
            y: $y,
            z: $z,
        }
    };
}

fn main() {
    let mut scene = Scene::new(7680, 4320, 60.0, vec3!(0.0, 8.0, 20.0), 28, 1e-3);

    scene.add_obj(Sphere::new(
        vec3!(0.0, 2.0, -8.0),
        4.0,
        Material::new(color!(1.0, 1.0, 1.0), 0.2, 0.9, 0.0, 0.0),
    ));

    scene.add_obj(Plane::new(
        vec3!(0.0, -4.5, 0.0),
        vec3!(0.0, -1.0, 0.0),
        //   Material::matte(color!(1.0, 1.0, 1.0), 0.2),
        Material::new(color!(0.0, 0.0, 0.0), 0.1, 0.9, 0.0, 0.0),
    ));

    scene.add_obj(Plane::new(
        vec3!(0.0, 0.0, -20.0),
        vec3!(0.0, 0.0, -1.0),
        Material::new(color!(1.0, 1.0, 1.0), 0.1, 0.8, 0.0, 0.0),
    ));

    scene.add_light(SphericalLight::new(
        vec3!(-30.0, 20.0, -2.0),
        rgb!(255.0, 0.0, 0.0),
        20000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(-20.0, 20.0, -2.0),
        rgb!(252.0, 30.0, 38.0),
        20000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(-10.0, 20.0, -2.0),
        rgb!(255.0, 214.0, 64.0),
        20000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(0.0, 20.0, -2.0),
        rgb!(0.0, 255.0, 0.0),
        20000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(10.0, 20.0, -2.0),
        rgb!(0.0, 50.0, 255.0),
        20000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(20.0, 20.0, -2.0),
        rgb!(189.0, 56.0, 255.0),
        20000.0,
    ));
    scene.add_light(SphericalLight::new(
        vec3!(30.0, 20.0, -2.0),
        rgb!(255.0, 84.0, 232.0),
        20000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(0.0, 30.0, 5.0),
        color!(1.0, 1.0, 1.0),
        60000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(0.0, 25.0, -18.0),
        color!(1.0, 1.0, 1.0),
        60000.0,
    ));

    let rendered = scene.threaded_render(180);
    rendered.save("test.png").unwrap();
}
