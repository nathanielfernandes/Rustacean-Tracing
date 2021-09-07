use crate::intersection::Intersection;
use crate::materials::Tracable;
use crate::objects::Intersectable;
use crate::objects::Object;
use crate::Color;
use crate::Vec3;
use std::cmp::Ordering;

//use crate::rendering::random_hemisphere_distribution;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f64,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f64) -> Ray {
        Ray {
            origin,
            direction,
            time,
        }
    }

    pub fn front_face(&self, outward_normal: &Vec3) -> bool {
        self.direction.dot(&outward_normal) < 0.0
    }

    pub fn trace<'traced>(
        &self,
        objects: &'traced Vec<Object>,
        t_min: f64,
        t_max: f64,
    ) -> Option<Intersection<'traced>> {
        objects
            .into_iter()
            .filter_map(|obj| {
                obj.intersects(self, t_min, t_max)
                    .map(|distance| Intersection::new(distance, &obj))
            })
            .min_by(|inter1, inter2| {
                inter1
                    .distance
                    .partial_cmp(&inter2.distance)
                    .unwrap_or(Ordering::Equal)
            })
    }

    //    pub fn color(&self, objects: &Vec<Object>, depth: u32) -> Color {
    pub fn color(&self, world: &Vec<Object>, background: &Color, depth: u32) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        const TEMP_UV: (f64, f64) = (0.0, 0.0);

        match self.trace(world, 0.001, ::std::f64::INFINITY) {
            Some(i) => {
                let point = self.at(i.distance);
                let mat = i.object.material();
                let emitted = mat.emitted(TEMP_UV, point, i.object);

                return match mat.scatter(self, point, i.object) {
                    Some((attenuation, scattered)) => {
                        emitted + attenuation * scattered.color(world, background, depth - 1)
                    }

                    None => emitted,
                };

                //return Color::new(1.0, 1.0, 1.0);
                // let target = point + random_hemisphere_distribution(surf_norm);
                // let nr = Ray::new(point, target - point);
                // 0.5 * nr.color(objects, depth - 1)
            }
            None => {
                *background
                // let t = 0.5 * (self.direction.normalize().y + 1.0);
                // (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
            }
        }
    }

    pub fn buffer(&self, world: &Vec<Object>, background: &Color) -> (Vec<f32>, Vec<f32>) {
        match self.trace(world, 0.001, ::std::f64::INFINITY) {
            Some(i) => {
                let point = self.at(i.distance);
                let mat = i.object.material();
                // let emitted = mat.emitted(TEMP_UV, point, i.object);
                let normal = i.object.surface_normal(&point, self);

                let outward_normal = i.object.outward_normal(&point, 0.0);
                let uv = i.object.surface_uv(&outward_normal);

                (
                    mat.albedo(uv, outward_normal).to_vec_f32(),
                    normal.to_vec_f32(),
                )
            }
            None => (background.to_vec_f32(), Vec3::zero().to_vec_f32()),
        }
    }

    pub fn at(&self, t: f64) -> Vec3 {
        self.origin + t * self.direction
    }

    // pub fn prime(x: u32, y: u32, scene: &Scene) -> Ray {
    //     let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
    //     let aspect_ratio = (scene.width as f64) / (scene.height as f64);

    //     let prime_x =
    //         ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
    //     let prime_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

    //     Ray {
    //         origin: scene.cam_pos,
    //         direction: Vec3 {
    //             x: prime_x,
    //             y: prime_y,
    //             z: -1.0,
    //         }
    //         .normalize(),
    //     }
    // }

    // pub fn reflection(normal: Vec3, incident: Vec3, point: Vec3, shadow_bias: f64) -> Ray {
    //     Ray {
    //         origin: point + (normal * shadow_bias),
    //         direction: incident - (2.0 * incident.dot(&normal) * normal),
    //     }
    // }
}

// let obj = &world[0];

// //let mut intersected_obj: Object;
// match obj.intersects(self, 0.001, ::std::f64::INFINITY, world) {
//     Some(i) => {
//         //   println!("meep");
//         let point = self.at(i.distance);

//         let mat = i.object.material().unwrap();
//         // if let Some(mat) = i.object.material() {
//         if let Some((attenuation, scattered)) = mat.scatter(self, point, i.object) {
//             return attenuation * scattered.color(world, depth - 1);
//         }
//         //}

//         return Color::new(0.0, 0.0, 0.0);
//         // let target = point + random_hemisphere_distribution(surf_norm);
//         // let nr = Ray::new(point, target - point);
//         // 0.5 * nr.color(objects, depth - 1)
//     }
//     None => {
//         let t = 0.5 * (self.direction.normalize().y + 1.0);
//         (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
//     }
// }
