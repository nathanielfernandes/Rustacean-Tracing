//use crate::color::Color;
use crate::materials::Material;
use crate::ray::Ray;

use crate::Vec3;

pub enum Object {
    Sphere(Sphere),
    Plane(Plane),
}
pub trait Intersectable {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64>;
    fn surface_normal(&self, point: &Vec3) -> Vec3;
    fn outward_normal(&self, point: &Vec3) -> Vec3;
}

impl Object {
    pub fn material(&self) -> &Material {
        match *self {
            Object::Sphere(ref obj) => &obj.material,
            Object::Plane(ref obj) => &obj.material,
        }
    }
}

impl Intersectable for Object {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        match *self {
            Object::Sphere(ref obj) => obj.intersects(ray, t_min, t_max),
            Object::Plane(ref obj) => obj.intersects(ray, t_min, t_max),
        }
    }

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.surface_normal(point),
            Object::Plane(ref obj) => obj.surface_normal(point),
        }
    }

    fn outward_normal(&self, point: &Vec3) -> Vec3 {
        match *self {
            Object::Sphere(ref obj) => obj.outward_normal(point),
            Object::Plane(ref obj) => obj.outward_normal(point),
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

    fn surface_normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.center).normalize()
    }

    fn outward_normal(&self, point: &Vec3) -> Vec3 {
        (*point - self.center) / self.radius
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

impl Intersectable for Plane {
    fn intersects(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<f64> {
        // remove
        let _temp = (t_min, t_max);
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

    fn outward_normal(&self, _point: &Vec3) -> Vec3 {
        // implement ples
        Vec3::zero()
    }
}
