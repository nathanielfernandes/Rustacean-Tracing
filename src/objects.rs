extern crate tobj;
// use bvh::aabb::{Bounded, AABB};
// use bvh::bounding_hierarchy::{BHShape, BoundingHierarchy};
// use bvh::bvh::BVH;
// use bvh::{Point3, Vector3};

// use std::default;
// use std::cell::RefCell;
// use std::cmp::Ordering;
use std::f32::consts::PI;
use std::path::Path;
// use std::sync::Arc;

// use std::ptr::null;

use crate::aabb::Aabb;
use crate::bvh2::BVH;
// use crate::color::BLACK;
use crate::intersection::Intersection;
// use crate::color::Color;
// use crate::intersection::Intersection;
//use crate::color::Color;
use crate::materials::{Dielectric, Isotropic, Lambertian, Material, Metal, Tracable};
use crate::ray::Ray;

use crate::texture::SolidColor;
// use crate::rendering::random_int;
// use crate::texture::SolidColor;
use crate::{Color, Vec3};

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Object {
    Sphere(Sphere),
    // MovingSphere(MovingSphere),
    Plane(Plane),
    Box(BoxObj),
    ConstantMedium(ConstantMedium),
    Triangle(Triangle),
    BigObject(BigObject),
}
pub trait Intersectable {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection>;
    //fn bounding_box(&self, )

    fn surface_normal(&self, point: &Vec3, ray: &Ray) -> Vec3;
    fn outward_normal(&self, point: &Vec3, time: f32) -> Vec3;

    fn surface_uv(&self, point: &Vec3) -> (f32, f32);
    fn bounding_box(&self) -> Option<Aabb>;
}

impl Object {
    pub fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        match *self {
            Object::Sphere(ref obj) => obj.intersects(ray, t_min, t_max),
            // Object::MovingSphere(ref obj) => obj.intersects(ray, t_min, t_max),
            Object::Plane(ref obj) => obj.intersects(ray, t_min, t_max),
            Object::Box(ref obj) => obj.intersects(ray, t_min, t_max),
            Object::ConstantMedium(ref obj) => obj.intersects(ray, t_min, t_max),
            Object::Triangle(ref obj) => obj.intersects(ray, t_min, t_max),
            Object::BigObject(ref obj) => obj.intersects(ray, t_min, t_max),
        }
    }

    pub fn surface_normal(&self, point: &Vec3, ray: &Ray) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.surface_normal(point, ray),
            // Object::MovingSphere(ref obj) => obj.surface_normal(point, ray),
            //    Object::BVHNode(ref obj) => obj.surface_normal(point),
            Object::Plane(ref obj) => obj.surface_normal(point, ray),
            Object::Box(ref _obj) => Vec3::zero(),
            Object::ConstantMedium(ref obj) => obj.surface_normal(point, ray),
            Object::Triangle(ref obj) => obj.surface_normal(point, ray),
            Object::BigObject(ref _obj) => Vec3::zero(),
        }
    }

    pub fn surface_uv(&self, point: &Vec3) -> (f32, f32) {
        match *self {
            Object::Sphere(ref obj) => obj.surface_uv(point),
            // Object::MovingSphere(ref obj) => obj.surface_uv(point),
            //    Object::BVHNode(ref obj) => obj.surface_normal(point),
            Object::Plane(ref obj) => obj.surface_uv(point),
            Object::Box(ref _obj) => (0.0, 0.0),
            Object::ConstantMedium(ref obj) => obj.surface_uv(point),
            Object::Triangle(ref obj) => obj.surface_uv(point),
            Object::BigObject(ref _obj) => (0.0, 0.0),
        }
    }

    pub fn outward_normal(&self, point: &Vec3, time: f32) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.outward_normal(point, time),
            // Object::MovingSphere(ref obj) => obj.outward_normal(point, time),
            //   Object::BVHNode(ref obj) => obj.outward_normal(point, time),
            Object::Plane(ref obj) => obj.outward_normal(point, time),
            Object::Box(ref _obj) => Vec3::zero(),
            Object::ConstantMedium(ref obj) => obj.outward_normal(point, time),
            Object::Triangle(ref obj) => obj.outward_normal(point, time),
            Object::BigObject(ref _obj) => Vec3::zero(),
        }
    }

    pub fn bounding_box(&self) -> Option<Aabb> {
        match *self {
            Object::Sphere(ref obj) => obj.bounding_box(),
            // Object::MovingSphere(ref obj) => obj.bounding_box(time_0, time_1),
            // Object::BVHNode(ref obj) => obj.bounding_box(time_0, time_1),
            Object::Plane(ref obj) => obj.bounding_box(),
            Object::Box(ref obj) => obj.bounding_box(),
            Object::ConstantMedium(ref obj) => obj.bounding_box(),
            Object::Triangle(ref obj) => obj.bounding_box(),
            Object::BigObject(ref obj) => obj.bounding_box(),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub center: Vec3,
    pub radius: f32,
    pub material: Material,
}

impl Sphere {
    pub fn new(center: Vec3, radius: f32, material: Material) -> Object {
        Object::Sphere(Sphere {
            center,
            radius,
            material,
        })
    }
}

impl Intersectable for Sphere {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
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

        let point = ray.at(root);
        let outward_normal = self.outward_normal(&point, ray.time);

        Some(Intersection::new(
            root,
            point,
            self.surface_normal(&point, ray),
            outward_normal,
            &self.material,
            self.surface_uv(&outward_normal),
        ))
    }

    fn surface_normal(&self, point: &Vec3, _ray: &Ray) -> Vec3 {
        (*point - self.center).normalize()
    }

    fn outward_normal(&self, point: &Vec3, _time: f32) -> Vec3 {
        (*point - self.center) / self.radius
    }

    fn surface_uv(&self, outward_noraml: &Vec3) -> (f32, f32) {
        let theta = (-outward_noraml.y).acos();
        let phi = (-outward_noraml.z).atan2(outward_noraml.x) + PI;

        (phi / (2.0 * PI), theta / PI)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: self.center - Vec3::new(self.radius, self.radius, self.radius),
            max: self.center + Vec3::new(self.radius, self.radius, self.radius),
        })
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum PlaneType {
    YZ,
    ZX,
    XY,
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Plane {
    plane_type: PlaneType,
    a0: f32,
    a1: f32,
    b0: f32,
    b1: f32,
    k: f32,
    material: Material,
}

impl Plane {
    pub fn new(
        plane_type: PlaneType,
        a0: f32,
        a1: f32,
        b0: f32,
        b1: f32,
        k: f32,
        material: Material,
    ) -> Object {
        Object::Plane(Plane {
            plane_type,
            a0,
            a1,
            b0,
            b1,
            k,
            material,
        })
    }

    pub fn get_axis(plane_type: &PlaneType) -> (usize, usize, usize) {
        match plane_type {
            PlaneType::YZ => (0, 1, 2),
            PlaneType::ZX => (1, 2, 0),
            PlaneType::XY => (2, 0, 1),
        }
    }
}

impl Intersectable for Plane {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let (k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);

        // let ray_origin = ray.origin.to_vec();
        // let ray_dir = ray.direction.to_vec();

        let t = (self.k - ray.origin[k_axis]) / ray.direction[k_axis];

        if t < t_min || t > t_max {
            None
        } else {
            let a = ray.origin[a_axis] + t * ray.direction[a_axis];
            let b = ray.origin[b_axis] + t * ray.direction[b_axis];
            if a < self.a0 || a > self.a1 || b < self.b0 || b > self.b1 {
                None
            } else {
                let point = ray.at(t);
                let u = (a - self.a0) / (self.a1 - self.a0);
                let v = (b - self.b0) / (self.b1 - self.b0);

                Some(Intersection::new(
                    t,
                    point,
                    self.surface_normal(&point, ray),
                    point,
                    &self.material,
                    (u, v),
                ))
            }
        }
    }

    fn surface_normal(&self, _point: &Vec3, ray: &Ray) -> Vec3 {
        let (k_axis, _a_axis, _b_axis) = Plane::get_axis(&self.plane_type);
        // let p = point.to_vec();
        let mut normal = Vec3::zero();
        // normal[k_axis] = 1.0;

        if ray.origin[k_axis] > self.k {
            normal[k_axis] = 1.0;
        } else {
            normal[k_axis] = -1.0;
        }

        //normal[k_axis] = 1.0;
        normal
    }

    fn outward_normal(&self, point: &Vec3, _time: f32) -> Vec3 {
        *point
    }

    fn surface_uv(&self, point: &Vec3) -> (f32, f32) {
        let (_k_axis, a_axis, b_axis) = Plane::get_axis(&self.plane_type);

        // let point = point.to_vec();
        let a = point[a_axis];
        let b = point[b_axis];

        (
            (a - self.a0) / (self.a1 - self.a0),
            (b - self.b0) / (self.b1 - self.b0),
        )
    }

    fn bounding_box(&self) -> Option<Aabb> {
        match self.plane_type {
            PlaneType::YZ => Some(Aabb {
                min: Vec3::new(self.k - 1e-4, self.a0, self.b0),
                max: Vec3::new(self.k + 1e-4, self.a1, self.b1),
            }),
            PlaneType::ZX => Some(Aabb {
                min: Vec3::new(self.b0, self.k - 1e-4, self.a0),
                max: Vec3::new(self.b1, self.k + 1e-4, self.a1),
            }),
            PlaneType::XY => Some(Aabb {
                min: Vec3::new(self.a0, self.b0, self.k - 1e-4),
                max: Vec3::new(self.a1, self.b1, self.k + 1e-4),
            }),
        }
    }
}

#[derive(Clone, Debug)]
pub struct BoxObj {
    pub min: Vec3,
    pub max: Vec3,
    pub faces: Vec<Object>,
}

impl BoxObj {
    pub fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        ray.trace(&self.faces, t_min, t_max)
    }

    pub fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: self.min,
            max: self.max,
        })
    }

    pub fn new(min: Vec3, max: Vec3, material: Material) -> Object {
        Object::Box(BoxObj {
            min,
            max,
            faces: create_box(min, max, material),
        })
    }
}

pub fn create_box(min: Vec3, max: Vec3, material: Material) -> Vec<Object> {
    vec![
        Plane::new(
            PlaneType::ZX,
            min.z,
            max.z,
            min.x,
            max.x,
            min.y,
            material.clone(),
        ),
        Plane::new(
            PlaneType::XY,
            min.x,
            max.x,
            min.y,
            max.y,
            max.z,
            material.clone(),
        ),
        Plane::new(
            PlaneType::XY,
            min.x,
            max.x,
            min.y,
            max.y,
            min.z,
            material.clone(),
        ),
        // bruh
        Plane::new(
            PlaneType::ZX,
            min.z,
            max.z,
            min.x,
            max.x,
            max.y,
            material.clone(),
        ),
        // bruh
        Plane::new(
            PlaneType::YZ,
            min.y,
            max.y,
            min.z,
            max.z,
            max.x,
            material.clone(),
        ),
        Plane::new(
            PlaneType::YZ,
            min.y,
            max.y,
            min.z,
            max.z,
            min.x,
            material.clone(),
        ),
    ]
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct ConstantMedium {
    boundary: BVH,
    density: f32,
    phase_function: Material,
}

impl ConstantMedium {
    pub fn new(boundary: BVH, density: f32, phase_function: Material) -> Object {
        Object::ConstantMedium(ConstantMedium {
            boundary,
            density,
            phase_function,
        })
    }
}

const ARBITRARY_NORM: Vec3 = Vec3 {
    x: 1.0,
    y: 0.0,
    z: 0.0,
};

impl Intersectable for ConstantMedium {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        if let Some(mut hit1) = self.boundary.intersects(&ray, -f32::MAX, f32::MAX) {
            if let Some(mut hit2) = self
                .boundary
                .intersects(&ray, hit1.distance + 0.0001, f32::MAX)
            {
                if hit1.distance < t_min {
                    hit1.distance = t_min
                }
                if hit2.distance > t_max {
                    hit2.distance = t_max
                }
                if hit1.distance < hit2.distance {
                    let distance_inside_boundary =
                        (hit2.distance - hit1.distance) * ray.direction.norm();
                    let hit_distance = -(1.0 / self.density) * rand::random::<f32>().ln();
                    if hit_distance < distance_inside_boundary {
                        let distance = hit1.distance + hit_distance / ray.direction.norm();
                        let point = ray.at(distance);
                        let outward_normal = hit2.outward_normal;
                        return Some(Intersection {
                            distance,
                            point,
                            normal: ARBITRARY_NORM, // Arbitrary
                            outward_normal,
                            mat: &self.phase_function,
                            uv: hit2.uv,
                        });
                    }
                }
            }
        }
        None
    }

    fn outward_normal(&self, _point: &Vec3, _time: f32) -> Vec3 {
        Vec3::zero()
    }

    fn surface_normal(&self, _point: &Vec3, _ray: &Ray) -> Vec3 {
        Vec3::zero()
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(self.boundary.bbox)
    }

    fn surface_uv(&self, _point: &Vec3) -> (f32, f32) {
        (0.0, 0.0)
    }
}
#[derive(Clone, Debug)]
pub struct Triangle {
    v0: Vec3,
    v1: Vec3,
    v2: Vec3,
    normal: Vec3,
    material: Material,
}

impl Triangle {
    pub fn new(v0: Vec3, v1: Vec3, v2: Vec3, material: Material) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            normal: (v1 - v0).cross(&(v2 - v0)).normalize(),
            material,
        }
    }

    pub fn new_with_normal(
        v0: Vec3,
        v1: Vec3,
        v2: Vec3,
        normal: Vec3,
        material: Material,
    ) -> Triangle {
        Triangle {
            v0,
            v1,
            v2,
            normal,
            material,
        }
    }
}

impl Intersectable for Triangle {
    fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        let v0v1 = self.v1 - self.v0;
        let v0v2 = self.v2 - self.v0;
        let pvec = ray.direction.cross(&v0v2);
        let det = v0v1.dot(&pvec);

        if det.abs() < 1e-4 {
            return None;
        }
        let inv_det = 1. / det;

        let tvec = ray.origin - self.v0;
        let u = tvec.dot(&pvec) * inv_det;
        if u < 0. || u > 1. {
            return None;
        }

        let qvec = tvec.cross(&v0v1);
        let v = ray.direction.dot(&qvec) * inv_det;
        if v < 0. || u + v > 1. {
            return None;
        }

        let t = v0v2.dot(&qvec) * inv_det;

        if t < t_min || t > t_max {
            return None;
        }

        let p = ray.at(t);

        return Some(Intersection {
            distance: t,
            point: p,
            normal: self.normal,
            outward_normal: self.outward_normal(&p, 0.0),
            mat: &self.material,
            uv: (u, v),
        });
    }

    fn surface_normal(&self, _point: &Vec3, _ray: &Ray) -> Vec3 {
        self.normal
    }

    fn surface_uv(&self, _point: &Vec3) -> (f32, f32) {
        (0.0, 0.0)
    }

    fn bounding_box(&self) -> Option<Aabb> {
        Some(Aabb {
            min: Vec3::new(
                self.v0.x.min(self.v1.x.min(self.v2.x)),
                self.v0.y.min(self.v1.y.min(self.v2.y)),
                self.v0.z.min(self.v1.z.min(self.v2.z)),
            ),
            max: Vec3::new(
                self.v0.x.max(self.v1.x.max(self.v2.x)),
                self.v0.y.max(self.v1.y.max(self.v2.y)),
                self.v0.z.max(self.v1.z.max(self.v2.z)),
            ),
        })
    }

    fn outward_normal(&self, _point: &Vec3, _time: f32) -> Vec3 {
        self.normal
    }
}

pub fn load_obj(path: &Path, origin: Vec3, scale: f32, default_mat: Material) -> Vec<Object> {
    let obj = tobj::load_obj(path, &tobj::LoadOptions::default());
    let (models, mtls) = obj.unwrap();
    let mut world: Vec<Object> = Vec::new();

    // let default_mat: Material = Lambertian::new(SolidColor::new(Color::new(0.6, 0.6, 0.6)));

    let mtls = mtls.unwrap_or_default();

    let materials: Vec<Material> = mtls
        .iter()
        .map(|m| match m.illumination_model {
            Some(7) => Dielectric::new(m.optical_density),
            Some(5) => Metal::new(
                SolidColor::new(Color::new(m.diffuse[0], m.diffuse[1], m.diffuse[2])),
                1.0 / m.shininess,
            ),
            _ => Lambertian::new(SolidColor::new(Color::new(
                m.diffuse[0],
                m.diffuse[1],
                m.diffuse[2],
            ))),
        })
        .collect();

    for m in models.iter() {
        let mesh = &m.mesh;
        for f in 0..mesh.indices.len() / 3 {
            let i0 = mesh.indices[3 * f] as usize;
            let i1 = mesh.indices[3 * f + 1] as usize;
            let i2 = mesh.indices[3 * f + 2] as usize;
            let v0 = Vec3::new(
                (mesh.positions[i0 * 3] * scale) + origin.x,
                (mesh.positions[i0 * 3 + 1] * scale) + origin.y,
                (mesh.positions[i0 * 3 + 2] * scale) + origin.z,
            );
            let v1 = Vec3::new(
                (mesh.positions[i1 * 3] * scale) + origin.x,
                (mesh.positions[i1 * 3 + 1] * scale) + origin.y,
                (mesh.positions[i1 * 3 + 2] * scale) + origin.z,
            );
            let v2 = Vec3::new(
                (mesh.positions[i2 * 3] * scale) + origin.x,
                (mesh.positions[i2 * 3 + 1] * scale) + origin.y,
                (mesh.positions[i2 * 3 + 2] * scale) + origin.z,
            );

            let mat: Material = match mesh.material_id {
                Some(id) => materials[id],
                None => default_mat,
            };

            let tri: Triangle;
            // if mesh.normals.len() > 0 {
            //     let normal = Vec3::new(
            //         mesh.normals[i0 * 3],
            //         mesh.normals[i0 * 3 + 1],
            //         mesh.normals[i0 * 3 + 2],
            //     );
            //     tri = Triangle::new_with_normal(v0, v1, v2, normal, mat)
            // } else {
            tri = Triangle::new(v0, v1, v2, mat);
            // }

            world.push(Object::Triangle(tri));
        }
    }

    world
}

#[derive(Clone, Debug)]
pub struct BigObject {
    pub objects: BVH,
}

impl BigObject {
    pub fn intersects(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<Intersection> {
        self.objects.intersects(ray, t_min, t_max)
    }

    pub fn bounding_box(&self) -> Option<Aabb> {
        Some(self.objects.bbox)
    }

    pub fn new(objects: Vec<Object>) -> Object {
        let b_objects: Vec<Box<Object>> = objects.into_iter().map(|o| Box::new(o)).collect();

        Object::BigObject(BigObject {
            objects: BVH::new(b_objects, 0.0, 1.0),
        })
    }
}

pub fn to_bvh(objects: Vec<Object>) -> BVH {
    let b_objects: Vec<Box<Object>> = objects.into_iter().map(|o| Box::new(o)).collect();

    BVH::new(b_objects, 0.0, 1.0)
}
