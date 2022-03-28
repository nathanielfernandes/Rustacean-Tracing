use crate::bvh::BvhTree;
use crate::bvh2::BVH;
use crate::color::BLACK;
// use crate::intersection;
use crate::intersection::Intersection;
use crate::materials::Tracable;
// use crate::objects::Intersectable;
use crate::objects::Object;
use crate::Color;
use crate::Vec3;
use std::cmp::Ordering;

const TEMP_UV: (f32, f32) = (0.0, 0.0);
//use crate::rendering::random_hemisphere_distribution;
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub time: f32,
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3, time: f32) -> Ray {
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
        t_min: f32,
        t_max: f32,
    ) -> Option<Intersection<'traced>> {
        objects
            .into_iter()
            .filter_map(|obj| obj.intersects(self, t_min, t_max))
            .min_by(|inter1, inter2| {
                inter1
                    .distance
                    .partial_cmp(&inter2.distance)
                    .unwrap_or(Ordering::Equal)
            })
    }

    // pub fn bvh2_trace<'traced>(
    //     &self,
    //     objects: BVH,
    //     t_min: f32,
    //     t_max: f32,
    // ) -> Option<Intersection<'traced>> {
    //     objects
    //         .into_iter()
    //         .filter_map(|obj| obj.intersects(self, t_min, t_max))
    //         .min_by(|inter1, inter2| {
    //             inter1
    //                 .distance
    //                 .partial_cmp(&inter2.distance)
    //                 .unwrap_or(Ordering::Equal)
    //         })
    // }

    // pub fn trace<'traced>(
    //     &self,
    //     objects: &'traced Vec<Object>,
    //     t_min: f32,
    //     t_max: f32,
    // ) -> Option<Intersection<'traced>> {
    //     objects
    //         .into_iter()
    //         .filter_map(|obj| {
    //             obj.intersects(self, t_min, t_max)
    //                 .map(|distance| Intersection::new(distance, &obj))
    //         })
    //         .min_by(|inter1, inter2| {
    //             inter1
    //                 .distance
    //                 .partial_cmp(&inter2.distance)
    //                 .unwrap_or(Ordering::Equal)
    //         })
    // }

    //    pub fn color(&self, objects: &Vec<Object>, depth: u32) -> Color {
    pub fn color(&self, world: &Vec<Object>, background: &Color, depth: u32) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        match self.trace(world, 0.001, ::std::f32::MAX) {
            Some(i) => {
                let mat = &i.mat;
                let emitted = mat.emitted(TEMP_UV, &i);
                return match mat.scatter(self, &i) {
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

    pub fn bvh_color(&self, world: &BvhTree, background: &Color, depth: u32) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        match world.hit(self, 0.001, ::std::f32::MAX) {
            Some(i) => {
                let mat = &i.mat;
                let emitted = mat.emitted(TEMP_UV, &i);
                return match mat.scatter(self, &i) {
                    Some((attenuation, scattered)) => {
                        emitted + attenuation * scattered.bvh_color(world, background, depth - 1)
                    }

                    None => emitted,
                };

                // let point = self.at(i.distance);
                // let mat = i.object.material();
                // let emitted = mat.emitted(TEMP_UV, point, i.object);

                // return match mat.scatter(self, point, i.object) {
                //     Some((attenuation, scattered)) => {
                //         emitted + attenuation * scattered.bvh_color(world, background, depth - 1)
                //     }

                //     None => emitted,
                // };

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

    pub fn bvh2_color(&self, world: &BVH, background: &Color, depth: u32) -> Color {
        if depth <= 0 {
            return BLACK;
        }

        match world.intersects(self, 0.001, ::std::f32::MAX) {
            Some(i) => {
                let mat = &i.mat;
                let emitted = mat.emitted(TEMP_UV, &i);
                return match mat.scatter(self, &i) {
                    Some((attenuation, scattered)) => {
                        emitted + attenuation * scattered.bvh2_color(world, background, depth - 1)
                    }

                    None => emitted,
                };
            }
            None => *background,
        }
    }

    // pub fn bvh_both(
    //     &self,
    //     world: &BvhTree,
    //     background: &Color,
    //     depth: u32,
    // ) -> (Color, Vec<f32>, Vec<f32>) {
    //     const TEMP_UV: (f32, f32) = (0.0, 0.0);

    //     match world.hit(self, 0.001, ::std::f32::MAX) {
    //         Some(i) => {
    //             let mat = &i.mat;
    //             let emitted = mat.emitted(TEMP_UV, &i);
    //             return match mat.scatter(self, &i) {
    //                 Some((attenuation, scattered)) => (
    //                     emitted + attenuation * scattered.bvh_color(world, background, depth - 1),
    //                     mat.albedo(i.uv, i.outward_normal).to_vec_f32(),
    //                     i.normal.to_vec_f32(),
    //                 ),

    //                 None => (emitted, m,
    //             };
    //         }
    //         None => (
    //             *background,
    //             background.to_vec_f32(),
    //             Vec3::zero().to_vec_f32(),
    //         ),
    //     }
    // }

    pub fn bvh_buffer(&self, world: &BvhTree, background: &Color) -> (Vec<f32>, Vec<f32>) {
        match world.hit(self, 0.001, ::std::f32::MAX) {
            Some(i) => {
                // let point = self.at(i.distance);
                let mat = i.mat;
                // let emitted = mat.emitted(TEMP_UV, point, i.object);
                let normal = i.normal;

                // let outward_normal = i.outward_normal;
                let uv = i.uv;

                // changed from outward normal to point
                (mat.albedo(uv, i.point).to_vec_f32(), normal.to_vec_f32())
            }
            None => (background.to_vec_f32(), Vec3::zero().to_vec_f32()),
        }
    }

    pub fn bvh2_buffer(&self, world: &BVH, background: &Color) -> (Vec<f32>, Vec<f32>) {
        match world.intersects(self, 0.001, ::std::f32::MAX) {
            Some(i) => {
                // let point = self.at(i.distance);
                let mat = i.mat;
                // let emitted = mat.emitted(TEMP_UV, point, i.object);
                let normal = i.normal;

                // let outward_normal = i.outward_normal;
                let uv = i.uv;

                // changed from outward normal to point
                (mat.albedo(uv, i.point).to_vec_f32(), normal.to_vec_f32())
            }
            None => (background.to_vec_f32(), Vec3::zero().to_vec_f32()),
        }
    }

    pub fn buffer(&self, world: &Vec<Object>, background: &Color) -> (Vec<f32>, Vec<f32>) {
        match self.trace(world, 0.001, ::std::f32::INFINITY) {
            Some(i) => {
                // let point = self.at(i.distance);
                let mat = i.mat;
                // let emitted = mat.emitted(TEMP_UV, point, i.object);
                let normal = i.normal;

                let outward_normal = i.outward_normal;
                let uv = i.uv;

                (
                    mat.albedo(uv, outward_normal).to_vec_f32(),
                    normal.to_vec_f32(),
                )
            }
            None => (background.to_vec_f32(), Vec3::zero().to_vec_f32()),
        }
    }

    pub fn at(&self, t: f32) -> Vec3 {
        self.origin + t * self.direction
    }
}
