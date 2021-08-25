use crate::color::Color;
use crate::objects::{Intersectable, Object};
use crate::ray::Ray;
use crate::rendering::{random_distribution, random_sphere_distribution};
use crate::vec3::Vec3;

pub enum Material {
    Labertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
}

pub trait Tracable {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)>;
}

impl Tracable for Material {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        match *self {
            Material::Labertian(ref mat) => mat.scatter(ray, point, object),
            Material::Metal(ref mat) => mat.scatter(ray, point, object),
            Material::Dielectric(ref mat) => mat.scatter(ray, point, object),
        }
    }
}

pub struct Lambertian {
    pub albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Material {
        Material::Labertian(Lambertian { albedo })
    }
}

impl Tracable for Lambertian {
    fn scatter(&self, _ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        let normal = object.surface_normal(&point);
        let mut scatter_dir = normal + random_sphere_distribution().normalize();

        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }

        Some((self.albedo, Ray::new(point, scatter_dir)))
    }
}

pub struct Metal {
    pub albedo: Color,
    pub fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Material {
        Material::Metal(Metal { albedo, fuzz })
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(&n) * n
    }
}

impl Tracable for Metal {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        let normal = object.surface_normal(&point);
        let reflected = Metal::reflect(ray.direction.normalize(), normal);

        if reflected.dot(&normal) > 0.0 {
            Some((
                self.albedo,
                Ray::new(
                    point,
                    reflected + self.fuzz * random_sphere_distribution().normalize(),
                ),
            ))
        } else {
            None
        }
    }
}

pub struct Dielectric {
    pub ir: f64,
}

impl Dielectric {
    pub fn new(index_of_refraction: f64) -> Material {
        Material::Dielectric(Dielectric {
            ir: index_of_refraction,
        })
    }

    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = (-uv).dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
        let r_our_parallel = -((1.0 - r_out_perp.norm()).abs().sqrt()) * normal;
        r_out_perp + r_our_parallel
    }

    pub fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Tracable for Dielectric {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        let outward_norm = object.outward_normal(&point);
        let normal;

        let attenuation = Color::new(1.0, 1.0, 1.0);
        let refraction_r;

        if ray.front_face(&outward_norm) {
            refraction_r = 1.0 / self.ir;
            normal = outward_norm;
        } else {
            refraction_r = self.ir;
            normal = -outward_norm;
        };

        let unit_direction = ray.direction.normalize();
        let cos_theta = (-unit_direction).dot(&normal).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = refraction_r * sin_theta > 1.0;
        let direction = if cannot_refract
            || Dielectric::reflectance(cos_theta, refraction_r) > random_distribution()
        {
            Metal::reflect(unit_direction, normal)
        } else {
            Dielectric::refract(unit_direction, normal, refraction_r)
        };
        Some((attenuation, Ray::new(point, direction)))
    }
}
