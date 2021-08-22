extern crate image;

//use crate::color::Color;
use crate::materials::Material;
use crate::ray::Ray;

use crate::Vec3;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}
pub trait Tracable {
    fn intersects(&self, ray: &Ray) -> Option<f64>;
    fn surface_normal(&self, point: &Vec3) -> Vec3;
}

impl Object {
    // pub fn color(&self) -> Color {
    //     match *self {
    //         Object::Sphere(ref obj) => obj.color,
    //         Object::Plane(ref obj) => obj.color,
    //     }
    // }

    pub fn material(&self) -> &Material {
        match *self {
            Object::Sphere(ref obj) => &obj.material,
            Object::Plane(ref obj) => &obj.material,
        }
    }

    // pub fn albedo(&self) -> f64 {
    //     match *self {
    //         Object::Sphere(ref obj) => obj.albedo,
    //         Object::Plane(ref obj) => obj.albedo,
    //     }
    // }

    // pub fn reflectivity(&self) -> f64 {
    //     match *self {
    //         Object::Sphere(ref obj) => obj.reflectivity,
    //         Object::Plane(ref obj) => obj.reflectivity,
    //     }
    // }
}

impl Tracable for Object {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        match *self {
            Object::Sphere(ref obj) => obj.intersects(ray),
            Object::Plane(ref obj) => obj.intersects(ray),
        }
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.surface_normal(point),
            Object::Plane(ref obj) => obj.surface_normal(point),
        }
    }
}

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

impl Tracable for Sphere {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let line_segment = self.center - ray.origin;
        let adjacent = line_segment.dot(&ray.direction);
        let distance = line_segment.dot(&line_segment) - (adjacent * adjacent);

        let r2 = self.radius * self.radius;

        if distance > r2 {
            return None;
        }

        let bounds = (r2 - distance).sqrt();
        let t0 = adjacent - bounds;
        let t1 = adjacent + bounds;

        if t0 < 0.0 && t1 < 0.0 {
            return None;
        }

        if t0 < t1 {
            Some(t0)
        } else {
            Some(t1)
        }
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.center).normalize()
    }
}

pub struct Plane {
    pub origin: Vec3,
    pub normal: Vec3,
    pub material: Material,
}

impl Plane {
    pub fn new(origin: Vec3, normal: Vec3, material: Material) -> Object {
        Object::Plane(Plane {
            origin,
            normal,
            material,
        })
    }
}

impl Tracable for Plane {
    fn intersects(&self, ray: &Ray) -> Option<f64> {
        let denominator = self.normal.dot(&ray.direction);
        if denominator.abs() > 1e-6 {
            let difference = self.origin - ray.origin;
            let t = difference.dot(&self.normal) / denominator;

            if t > 1e-6 {
                return Some(t);
            }
        }
        None
    }

    fn surface_normal(&self, _point: &Vec3) -> Vec3 {
        -self.normal
    }
}
