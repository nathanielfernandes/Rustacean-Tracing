use crate::color::Color;
use crate::objects::Object;
use crate::ray::Ray;
use crate::rendering::{random_distribution, random_sphere_distribution};
use crate::texture::Texture;
use crate::vec3::Vec3;

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Material {
    Labertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    EmissiveDiffuse(EmissiveDiffuse),
}

pub trait Tracable {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)>;
    fn emitted(&self, uv: (f32, f32), point: Vec3, object: &Object) -> Color;
    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color;
}

impl Tracable for Material {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        match *self {
            Material::Labertian(ref mat) => mat.scatter(ray, point, object),
            Material::Metal(ref mat) => mat.scatter(ray, point, object),
            Material::Dielectric(ref mat) => mat.scatter(ray, point, object),
            Material::EmissiveDiffuse(ref mat) => mat.scatter(ray, point, object),
        }
    }

    fn emitted(&self, uv: (f32, f32), point: Vec3, object: &Object) -> Color {
        match *self {
            Material::Labertian(ref mat) => mat.emitted(uv, point, object),
            Material::Metal(ref mat) => mat.emitted(uv, point, object),
            Material::Dielectric(ref mat) => mat.emitted(uv, point, object),
            Material::EmissiveDiffuse(ref mat) => mat.emitted(uv, point, object),
        }
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        match *self {
            Material::Labertian(ref mat) => mat.albedo(uv, point),
            Material::Metal(ref mat) => mat.albedo(uv, point),
            Material::Dielectric(ref mat) => mat.albedo(uv, point),
            Material::EmissiveDiffuse(ref mat) => mat.albedo(uv, point),
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Lambertian {
    pub texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Material {
        Material::Labertian(Lambertian { texture })
    }
}

impl Tracable for Lambertian {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        let normal = object.surface_normal(&point, ray);
        let mut scatter_dir = normal + random_sphere_distribution().normalize();
        let outward_normal = object.outward_normal(&point, 0.0);
        let uv = object.surface_uv(&outward_normal);

        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }

        Some((
            self.texture.get_color_uv(uv, point),
            Ray::new(point, scatter_dir, ray.time),
        ))
    }

    fn emitted(&self, _uv: (f32, f32), _point: Vec3, _object: &Object) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Metal {
    pub texture: Texture,
    pub fuzz: f32,
}

impl Metal {
    pub fn new(texture: Texture, fuzz: f32) -> Material {
        Material::Metal(Metal { texture, fuzz })
    }

    pub fn reflect(v: Vec3, n: Vec3) -> Vec3 {
        v - 2.0 * v.dot(&n) * n
    }
}

impl Tracable for Metal {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        let normal = object.surface_normal(&point, ray);
        let reflected = Metal::reflect(ray.direction.normalize(), normal);
        let outward_normal = object.outward_normal(&point, 0.0);
        let uv = object.surface_uv(&outward_normal);

        if reflected.dot(&normal) > 0.0 {
            Some((
                self.texture.get_color_uv(uv, point),
                Ray::new(
                    point,
                    reflected + self.fuzz * random_sphere_distribution().normalize(),
                    ray.time,
                ),
            ))
        } else {
            None
        }
    }

    fn emitted(&self, _uv: (f32, f32), _point: Vec3, _object: &Object) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Dielectric {
    pub ir: f32,
}

impl Dielectric {
    pub fn new(index_of_refraction: f32) -> Material {
        Material::Dielectric(Dielectric {
            ir: index_of_refraction,
        })
    }

    pub fn refract(uv: Vec3, normal: Vec3, etai_over_etat: f32) -> Vec3 {
        let cos_theta = (-uv).dot(&normal).min(1.0);
        let r_out_perp = etai_over_etat * (uv + cos_theta * normal);
        let r_our_parallel = -((1.0 - r_out_perp.norm()).abs().sqrt()) * normal;
        r_out_perp + r_our_parallel
    }

    pub fn reflectance(cosine: f32, ref_idx: f32) -> f32 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Tracable for Dielectric {
    fn scatter(&self, ray: &Ray, point: Vec3, object: &Object) -> Option<(Color, Ray)> {
        let outward_norm = object.outward_normal(&point, ray.time);
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
        Some((attenuation, Ray::new(point, direction, ray.time)))
    }

    fn emitted(&self, _uv: (f32, f32), _point: Vec3, _object: &Object) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn albedo(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct EmissiveDiffuse {
    texture: Texture,
}

impl EmissiveDiffuse {
    pub fn new(texture: Texture) -> Material {
        Material::EmissiveDiffuse(EmissiveDiffuse { texture })
    }
}

impl Tracable for EmissiveDiffuse {
    fn scatter(&self, _ray: &Ray, _point: Vec3, _object: &Object) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _uv: (f32, f32), point: Vec3, object: &Object) -> Color {
        let outward_normal = object.outward_normal(&point, 0.0);
        let uv = object.surface_uv(&outward_normal);
        self.texture.get_color_uv(uv, point)
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}
