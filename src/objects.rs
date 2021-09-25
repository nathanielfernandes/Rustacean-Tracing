use bvh::aabb::{Bounded, AABB};
use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
use bvh::bvh::BVH;
use bvh::{Point3, Vector3};

// use std::cell::RefCell;
// use std::cmp::Ordering;
use std::f32::consts::PI;
use std::ops::Bound;
// use std::ptr::null;

// use crate::color::Color;
// use crate::intersection::Intersection;
//use crate::color::Color;
use crate::materials::Material;
use crate::ray::Ray;

// use crate::rendering::random_int;
// use crate::texture::SolidColor;
use crate::Vec3;

const PI_F32: f64 = PI as f64;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Object {
    Sphere(Sphere),
    // MovingSphere(MovingSphere),
    // Plane(Plane),
}
pub trait Intersectable {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64>;
    //fn bounding_box(&self, )

    fn surface_normal(&self, point: &Vec3, ray: &Ray) -> Vec3;
    fn outward_normal(&self, point: &Vec3, time: f64) -> Vec3;

    fn surface_uv(&self, point: &Vec3) -> (f64, f64);
    //fn bounding_box(&self, time_0: f64, time_1: f64) -> AABB;
}

impl Object {
    pub fn material(&self) -> &Material {
        match *self {
            Object::Sphere(ref obj) => &obj.material,
            // Object::MovingSphere(ref obj) => &obj.material,
            //Object::BVHNode(ref _obj) => None,
            // Object::Plane(ref obj) => &obj.material,
        }
    }
    // }

    // impl Intersectable for Object {
    pub fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        match *self {
            Object::Sphere(ref obj) => obj.intersects(ray, t_min, t_max),
            // Object::MovingSphere(ref obj) => obj.intersects(ray, t_min, t_max),
            // Object::Plane(ref obj) => obj.intersects(ray, t_min, t_max),
        }
        // let intersect = match *self {
        //     Object::Sphere(ref obj) => obj.intersects(ray, t_min, t_max),
        //     Object::MovingSphere(ref obj) => obj.intersects(ray, t_min, t_max),
        //     Object::BVHNode(ref obj) => return obj.intersects(ray, t_min, t_max, world),
        //     // Object::Plane(ref obj) => obj.intersects(ray, t_min, t_max),
        // };

        // match intersect {
        //     Some(distance) => Some(Intersection::new(distance, self)),
        //     None => None,
        // }
    }

    pub fn surface_normal(&self, point: &Vec3, ray: &Ray) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.surface_normal(point, ray),
            // Object::MovingSphere(ref obj) => obj.surface_normal(point, ray),
            //    Object::BVHNode(ref obj) => obj.surface_normal(point),
            // Object::Plane(ref obj) => obj.surface_normal(point, ray),
        }
    }

    pub fn surface_uv(&self, point: &Vec3) -> (f64, f64) {
        match *self {
            Object::Sphere(ref obj) => obj.surface_uv(point),
            // Object::MovingSphere(ref obj) => obj.surface_uv(point),
            //    Object::BVHNode(ref obj) => obj.surface_normal(point),
            // Object::Plane(ref obj) => obj.surface_uv(point),
        }
    }

    pub fn outward_normal(&self, point: &Vec3, time: f64) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.outward_normal(point, time),
            // Object::MovingSphere(ref obj) => obj.outward_normal(point, time),
            //   Object::BVHNode(ref obj) => obj.outward_normal(point, time),
            // Object::Plane(ref obj) => obj.outward_normal(point, time),
        }
    }

    // fn bounding_box(&self, time_0: f64, time_1: f64) -> AABB {
    //     match *self {
    //         Object::Sphere(ref obj) => obj.bounding_box(time_0, time_1),
    //         Object::MovingSphere(ref obj) => obj.bounding_box(time_0, time_1),
    //         Object::BVHNode(ref obj) => obj.bounding_box(time_0, time_1),
    //         //  Object::Plane(ref obj) => obj.outward_normal(point),
    //     }
    // }
}

impl Bounded for Object {
    fn aabb(&self) -> AABB {
        match *self {
            Object::Sphere(ref obj) => obj.aabb(),
            // Object::MovingSphere(ref obj) => obj.aabb(point, time),
            //   Object::BVHNode(ref obj) => obj.outward_normal(point, time),
            // Object::Plane(ref obj) => obj.aabb(),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f64,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f64, material: Material) -> Object {
        Object::Sphere(Sphere {
            center,
            radius,
            material,
        })
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        let oc = ray.origin - self.center;
        let a = ray.direction.norm();
        let half_b = oc.dot(&ray.direction);
        let c = oc.norm() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        Some(root)
    }

    fn surface_normal(&self, point: &Vec3, _ray: &Ray) -> Vec3 {
        (*point - self.center).normalize()
    }

    fn outward_normal(&self, point: &Vec3, _time: f64) -> Vec3 {
        (*point - self.center) / self.radius
    }

    fn surface_uv(&self, outward_noraml: &Vec3) -> (f64, f64) {
        let theta = (-outward_noraml.y).acos();
        let phi = (-outward_noraml.z).atan2(outward_noraml.x) + PI_F32;

        (phi / (2.0 * PI_F32), theta / PI_F32)
    }

    // fn bounding_box(&self, _time_0: f64, _time_1: f64) -> AABB {
    //     AABB {
    //         min: self.center - Vec3::new(self.radius, self.radius, self.radius),
    //         max: self.center + Vec3::new(self.radius, self.radius, self.radius),
    //     }
    // }
}

impl Bounded for Sphere {
    fn aabb(&self) -> AABB {
        let half_size = Vector3::new(self.radius, self.radius, self.radius);
        let min = self.center - half_size;
        let max = self.center + half_size;
        AABB::with_bounds(min, max)
    }
}

// #[allow(dead_code)]
// #[derive(Clone, Debug)]
// pub struct MovingSphere {
//     pub center_0: Vec3,
//     pub center_1: Vec3,
//     pub time_0: f64,
//     pub time_1: f64,
//     pub radius: f64,
//     pub material: Material,
// }

// impl MovingSphere {
//     pub fn new(
//         center_0: Vec3,
//         center_1: Vec3,
//         time_0: f64,
//         time_1: f64,
//         radius: f64,
//         material: Material,
//     ) -> Object {
//         Object::MovingSphere(MovingSphere {
//             center_0,
//             center_1,
//             time_0,
//             time_1,
//             radius,
//             material,
//         })
//     }

//     pub fn center(&self, time: f64) -> Vec3 {
//         self.center_0
//             + ((time - self.time_0) / (self.time_1 - self.time_0)) * (self.center_1 - self.center_0)
//     }
// }

// impl Intersectable for MovingSphere {
//     fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
//         let oc = ray.origin - self.center(ray.time);
//         let a = ray.direction.norm();
//         let half_b = oc.dot(&ray.direction);
//         let c = oc.norm() - self.radius * self.radius;
//         let discriminant = half_b * half_b - a * c;

//         if discriminant < 0.0 {
//             return None;
//         }

//         let sqrtd = discriminant.sqrt();
//         let mut root = (-half_b - sqrtd) / a;
//         if root < t_min || t_max < root {
//             root = (-half_b + sqrtd) / a;
//             if root < t_min || t_max < root {
//                 return None;
//             }
//         }
//         Some(root)
//     }

//     fn surface_normal(&self, point: &Vec3, _ray: &Ray) -> Vec3 {
//         (*point - self.center(0.0)).normalize()
//     }

//     fn outward_normal(&self, point: &Vec3, time: f64) -> Vec3 {
//         (*point - self.center(time)) / self.radius
//     }

//     fn surface_uv(&self, point: &Vec3) -> (f64, f64) {
//         let theta = (-point.y).acos();
//         let phi = (-point.z).atan2(point.x) + PI_F32;

//         (phi / (2.0 * PI_F32), theta / PI_F32)
//     }

// fn bounding_box(&self, time_0: f64, time_1: f64) -> AABB {
//     let box_0 = AABB::new(
//         self.center(time_0) - Vec3::new(self.radius, self.radius, self.radius),
//         self.center(time_0) + Vec3::new(self.radius, self.radius, self.radius),
//     );

//     let box_1 = AABB::new(
//         self.center(time_1) - Vec3::new(self.radius, self.radius, self.radius),
//         self.center(time_1) + Vec3::new(self.radius, self.radius, self.radius),
//     );

//     AABB::surrounding_box(box_0, box_1)
// }
// }

// #[allow(dead_code)]
// #[derive(Clone, Debug)]
// pub enum PlaneType {
//     YZ,
//     ZX,
//     XY,
// }

// #[allow(dead_code)]
// #[derive(Clone, Debug)]
// pub struct Plane {
//     plane_type: PlaneType,
//     a0: f64,
//     a1: f64,
//     b0: f64,
//     b1: f64,
//     k: f64,
//     material: Material,
// }

// impl Plane {
//     pub fn new(
//         plane_type: PlaneType,
//         a0: f64,
//         a1: f64,
//         b0: f64,
//         b1: f64,
//         k: f64,
//         material: Material,
//     ) -> Object {
//         Object::Plane(Plane {
//             plane_type,
//             a0,
//             a1,
//             b0,
//             b1,
//             k,
//             material,
//         })
//     }

//     pub fn get_axis(plane_type: &PlaneType) -> (usize, usize, usize) {
//         match plane_type {
//             PlaneType::YZ => (0, 1, 2),
//             PlaneType::ZX => (1, 2, 0),
//             PlaneType::XY => (2, 0, 1),
//         }
//     }
// }

// impl Intersectable for Plane {
//     fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
//         let (k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);

//         let ray_origin = ray.origin.to_vec();
//         let ray_dir = ray.direction.to_vec();

//         let t = (self.k - ray_origin[k_axis]) / ray_dir[k_axis];

//         if t < t_min || t > t_max {
//             None
//         } else {
//             let a = ray_origin[a_axis] + t * ray_dir[a_axis];
//             let b = ray_origin[b_axis] + t * ray_dir[b_axis];
//             if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
//                 None
//             } else {
//                 Some(t)

//                 // let u = (a - self.a0) / (self.a1 - self.a0);
//                 // let v = (b - self.b0) / (self.b1 - self.b0);
//                 // let p = ray.at(t);

//                 // let mut normal = Vec3::zero().to_vec();
//                 // normal[k_axis] = 1.0;

//                 // Some(HitRecord {
//                 //     t,
//                 //     u,
//                 //     v,
//                 //     p,
//                 //     normal,
//                 //     material: &self.material,
//                 // })
//             }
//         }
//     }

//     fn surface_normal(&self, _point: &Vec3, ray: &Ray) -> Vec3 {
//         let (k_axis, _a_axis, _b_axis) = Plane::get_axis(&self.plane_type);
//         // let p = point.to_vec();
//         let mut normal = Vec3::zero().to_vec();
//         normal[k_axis] = 1.0;

//         let p = ray.origin.to_vec();

//         if p[k_axis] > self.k {
//             normal[k_axis] = 1.0;
//         } else {
//             normal[k_axis] = -1.0;
//         }

//         //normal[k_axis] = 1.0;
//         Vec3::new(normal[0], normal[1], normal[2])
//     }

//     fn outward_normal(&self, point: &Vec3, _time: f64) -> Vec3 {
//         // let (k_axis, _a_axis, _b_axis) = Plane::get_axis(&self.plane_type);
//         // let mut normal = Vec3::zero().to_vec();
//         // normal[k_axis] = 1.0;

//         // Vec3::new(normal[0], normal[1], normal[2])
//         *point
//     }

//     fn surface_uv(&self, point: &Vec3) -> (f64, f64) {
//         let (_k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);

//         let point = point.to_vec();
//         let a = point[a_axis];
//         let b = point[b_axis];

//         (
//             (a - self.a0) / (self.a1 - self.a0),
//             (b - self.b0) / (self.b1 - self.b0),
//         )
//     }
// }

// // #[derive(Clone, Debug)]
// // pub struct Box {
// //     pub min: Vec3,
// //     pub max: Vec3,
// //     pub faces: Vec<Object>,
// //     pub default_mat: Material,
// // }

// pub fn create_box(min: Vec3, max: Vec3, material: Material) -> Vec<Object> {
//     vec![
//         Plane::new(
//             PlaneType::XY,
//             min.x,
//             max.x,
//             min.y,
//             max.y,
//             max.z,
//             material.clone(),
//         ),
//         Plane::new(
//             PlaneType::XY,
//             min.x,
//             max.x,
//             min.y,
//             max.y,
//             min.z,
//             material.clone(),
//         ),
//         // bruh
//         Plane::new(
//             PlaneType::ZX,
//             min.z,
//             max.z,
//             min.x,
//             max.x,
//             max.y,
//             material.clone(),
//         ),
//         Plane::new(
//             PlaneType::ZX,
//             min.z,
//             max.z,
//             min.x,
//             max.x,
//             min.y,
//             material.clone(),
//         ),
//         // bruh
//         Plane::new(
//             PlaneType::YZ,
//             min.y,
//             max.y,
//             min.z,
//             max.z,
//             max.x,
//             material.clone(),
//         ),
//         Plane::new(
//             PlaneType::YZ,
//             min.y,
//             max.y,
//             min.z,
//             max.z,
//             min.x,
//             material.clone(),
//         ),
//     ]
// }
