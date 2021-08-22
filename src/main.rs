extern crate image;
//extern crate oidn;

pub mod color;
pub mod intersection;
pub mod lights;
pub mod materials;
pub mod objects;
pub mod ray;
pub mod scene;
pub mod vec3;

use color::*;
use image::{ImageBuffer, RgbImage};
use objects::{Plane, Sphere};
use ray::Ray;
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
    let mut scene = Scene::new(1920, 1080, 60.0, vec3!(0.0, 8.0, 35.0), 2, 20, 1e-3);

    // scene.add_obj(Sphere::new(
    //     vec3!(0.0, 2.0, -8.0),
    //     4.0,
    //     Material::new(color!(1.0, 1.0, 1.0), 0.2, 0.9, 0.0, 0.0),
    // ));

    // scene.add_obj(Sphere::new(
    //     vec3!(-8.0, 1.0, -6.0),
    //     2.0,
    //     Material::new(color!(1.0, 1.0, 1.0), 0.4, 0.1, 1250.0, 2.0),
    // ));

    // scene.add_obj(Sphere::new(
    //     vec3!(8.0, 1.0, -6.0),
    //     2.0,
    //     Material::new(color!(1.0, 1.0, 1.0), 0.6, 0.4, 1250.0, 2.0),
    // ));

    // scene.add_obj(Sphere::new(
    //     vec3!(0.0, 0.0, -5.0),
    //     1.0,
    //     Material::new(color!(0.2, 0.2, 1.0), 0.18, 0.5, 250.0, 0.15),
    // ));

    // scene.add_obj(Sphere::new(
    //     vec3!(-3.0, 1.0, -6.0),
    //     1.4,
    //     Material::matte(color!(0.2, 0.7, 0.2), 0.2),
    // ));

    // scene.add_obj(Sphere::new(
    //     vec3!(2.0, 2.0, -4.0),
    //     0.5,
    //     Material::shiny(color!(1.0, 0.2, 0.2), 0.18, 0.5),
    // ));

    // scene.add_obj(Plane::new(
    //     vec3!(0.0, -4.5, 0.0),
    //     vec3!(0.0, -1.0, 0.0),
    //     //   Material::matte(color!(1.0, 1.0, 1.0), 0.2),
    //     Material::new(color!(0.0, 0.0, 0.0), 0.1, 0.9, 0.0, 0.0),
    // ));

    scene.add_obj(Plane::new(
        vec3!(0.0, 0.0, -20.0),
        vec3!(0.0, 0.0, -1.0),
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
        //Material::new(color!(1.0, 1.0, 1.0), 1.0, 1.0, 1250.0, 0.2), //Material::matte(color!(0.0, 0.0, 1.0), 0.18),
    ));

    scene.add_obj(Plane::new(
        vec3!(0.0, 0.0, 36.0),
        vec3!(0.0, 0.0, 1.0),
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Plane::new(
        vec3!(0.0, -4.5, 0.0),
        vec3!(0.0, -1.0, 0.0),
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Plane::new(
        vec3!(0.0, 35.0, 0.0),
        vec3!(0.0, 1.0, 0.0),
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Plane::new(
        vec3!(-20.0, 0.0, 0.0),
        vec3!(-1.0, 0.0, 0.0),
        Material::matte(color!(1.0, 0.0, 0.0), 0.58),
    ));

    scene.add_obj(Plane::new(
        vec3!(20.0, 0.0, 0.0),
        vec3!(1.0, 0.0, 0.0),
        Material::matte(color!(0.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Plane::new(
        vec3!(20.0, 0.0, 0.0),
        vec3!(1.0, 0.0, 0.0),
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Sphere::new(
        vec3!(-8.0, 3.0, -6.0),
        7.0,
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Sphere::new(
        vec3!(8.0, 1.0, 4.0),
        5.0,
        Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    scene.add_obj(Sphere::new(
        vec3!(0.0, 20.0, 4.0),
        3.0,
        Material::new(color!(1.0, 1.0, 1.0), 0.7, 0.3, 1250.0, 0.2), //Material::matte(color!(0.0, 0.0, 1.0), 0.18),
                                                                     //Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    ));

    // scene.add_obj(Sphere::new(
    //     vec3!(0.0, 15.0, -6.0),
    //     5.0,
    //     Material::matte(color!(1.0, 1.0, 1.0), 0.58),
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(-30.0, 20.0, -2.0),
    //     rgb!(255.0, 0.0, 0.0),
    //     20000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(-20.0, 20.0, -2.0),
    //     rgb!(252.0, 30.0, 38.0),
    //     20000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(-10.0, 20.0, -2.0),
    //     rgb!(255.0, 214.0, 64.0),
    //     20000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(0.0, 20.0, -2.0),
    //     rgb!(0.0, 255.0, 0.0),
    //     20000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(10.0, 20.0, -2.0),
    //     rgb!(0.0, 50.0, 255.0),
    //     20000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(20.0, 20.0, -2.0),
    //     rgb!(189.0, 56.0, 255.0),
    //     20000.0,
    // ));
    // scene.add_light(SphericalLight::new(
    //     vec3!(30.0, 20.0, -2.0),
    //     rgb!(255.0, 84.0, 232.0),
    //     20000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(8.0, 20.0, -2.0),
    //     color!(0.0, 1.0, 1.0),
    //     60000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(-30.0, 20.0, -2.0),
    //     color!(1.0, 0.2, 0.0),
    //     60000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(0.0, 2.0, 0.0),
    //     color!(1.0, 1.0, 1.0),
    //     2000.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(-10.0, 10.0, 6.0),
    //     color!(1.0, 1.0, 1.0),
    //     10000.0,
    // ));

    // dis da one

    scene.add_light(SphericalLight::new(
        vec3!(0.0, 32.0, -2.0),
        WHITE,
        //rgb!(255.0, 245.0, 207.0),
        18000.0,
    ));

    scene.add_light(SphericalLight::new(
        vec3!(0.0, 5.0, 20.0),
        WHITE,
        //rgb!(255.0, 245.0, 207.0),
        10000.0,
    ));

    // scene.add_light(DirectionalLight::new(
    //     vec3!(0.0, 0.0, -0.5),
    //     color!(1.0, 1.0, 1.0),
    //     10.0,
    // ));

    // scene.add_light(SphericalLight::new(
    //     vec3!(0.0, 5.0, -6.0),
    //     color!(1.0, 1.0, 1.0),
    //     5000.0,
    // ));
    // scene.add_light(SphericalLight::new(vec3!(0.25, 1.0, -2.0), WHITE, 4000.0));
    //scene.add_light(SphericalLight::new(vec3!(0.25, 1.0, -2.0), RED, 500.0));

    // scene.add_light(SphericalLight::new(
    //     vec3!(-2.0, 10.0, -3.0),
    //     color!(0.3, 0.8, 0.3),
    //     10000.0,
    // ));
    // let light3 = SphericalLight::new(vec3!(0.25, 0.0, -2.0), color!(0.8, 0.3, 0.3), 250.0);

    // let sphere = Sphere::new(vec3!(0.0, 0.0, -5.0), 1.0, color!(0.2, 1.0, 0.2), 0.18);
    // let sphere1 = Sphere::new(vec3!(-3.0, 1.0, -6.0), 2.0, BLUE, 0.58);
    // let sphere2 = Sphere::new(vec3!(2.0, 1.0, -3.0), 1.5, color!(1.0, 1.0, 1.0), 0.18);

    // let plane = Plane::new(vec3!(0.0, -2.0, -5.0), vec3!(0.0, -1.0, 0.0), WHITE, 0.18);
    // let plane2 = Plane::new(
    //     vec3!(0.0, 0.0, -20.0),
    //     vec3!(0.0, 0.0, -1.0),
    //     color!(0.2, 0.3, 1.0),
    //     0.38,
    // );

    // //scene.add_light(light);
    // scene.add_light(light1);
    // scene.add_light(light3);

    // scene.add_obj(sphere);
    // scene.add_obj(sphere1);
    // scene.add_obj(sphere2);
    // scene.add_obj(plane);
    // scene.add_obj(plane2);
    //scene.render();
    let rendered = scene.threaded_render(90);
    rendered.save("test.png").unwrap();
}
