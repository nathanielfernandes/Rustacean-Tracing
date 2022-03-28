use crate::color::{Color, BLACK, WHITE};
use crate::intersection::Intersection;
// use crate::objects::Object;
use crate::ray::Ray;
use crate::rendering::{random_distribution, random_sphere_distribution};
use crate::texture::Texture;
use crate::vec3::Vec3;

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub enum Material {
    Labertian(Lambertian),
    Metal(Metal),
    Dielectric(Dielectric),
    EmissiveDiffuse(EmissiveDiffuse),
    Isotropic(Isotropic),
    Glossy(Glossy),
}

pub trait Tracable {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)>;
    fn emitted(&self, uv: (f32, f32), inter: &Intersection) -> Color;
    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color;
}

impl Tracable for Material {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        match *self {
            Material::Labertian(ref mat) => mat.scatter(ray, inter),
            Material::Metal(ref mat) => mat.scatter(ray, inter),
            Material::Dielectric(ref mat) => mat.scatter(ray, inter),
            Material::EmissiveDiffuse(ref mat) => mat.scatter(ray, inter),
            Material::Isotropic(ref mat) => mat.scatter(ray, inter),
            Material::Glossy(ref mat) => mat.scatter(ray, inter),
        }
    }

    fn emitted(&self, uv: (f32, f32), inter: &Intersection) -> Color {
        match *self {
            Material::Labertian(ref _mat) => BLACK, // mat.emitted(uv, inter),
            Material::Metal(ref _mat) => BLACK,     //mat.emitted(uv, inter),
            Material::Dielectric(ref _mat) => BLACK, //mat.emitted(uv, inter),
            Material::EmissiveDiffuse(ref mat) => mat.emitted(uv, inter),
            Material::Isotropic(ref _mat) => BLACK, //mat.emitted(uv, inter),
            Material::Glossy(ref _mat) => BLACK,
        }
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        match *self {
            Material::Labertian(ref mat) => mat.albedo(uv, point),
            Material::Metal(ref mat) => mat.albedo(uv, point),
            Material::Dielectric(ref _mat) => BLACK, //mat.albedo(uv, point),
            Material::EmissiveDiffuse(ref mat) => mat.albedo(uv, point),
            Material::Isotropic(ref mat) => mat.albedo(uv, point),
            Material::Glossy(ref mat) => mat.albedo(uv, point),
        }
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Lambertian {
    pub texture: Texture,
}

impl Lambertian {
    pub fn new(texture: Texture) -> Material {
        Material::Labertian(Lambertian { texture })
    }
}

impl Tracable for Lambertian {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let normal = inter.normal;
        let mut scatter_dir = inter.point + normal + random_sphere_distribution().normalize();
        // let outward_normal = inter.outward_normal;
        let uv = inter.uv;

        if scatter_dir.near_zero() {
            scatter_dir = normal;
        }

        Some((
            self.texture.get_color_uv(uv, inter.point),
            Ray::new(inter.point, scatter_dir - inter.point, ray.time),
        ))
    }

    fn emitted(&self, _uv: (f32, f32), _inter: &Intersection) -> Color {
        BLACK
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
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
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let normal = inter.normal;
        let reflected = Metal::reflect(ray.direction.normalize(), normal);
        // let outward_normal = inter.outward_normal;
        let uv = inter.uv;

        if reflected.dot(&normal) > 0.0 {
            Some((
                self.texture.get_color_uv(uv, inter.point),
                Ray::new(
                    inter.point,
                    reflected + self.fuzz * random_sphere_distribution().normalize(),
                    ray.time,
                ),
            ))
        } else {
            None
        }
    }

    fn emitted(&self, _uv: (f32, f32), _inter: &Intersection) -> Color {
        BLACK
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
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
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        let outward_norm = inter.outward_normal;
        let normal;

        let attenuation = WHITE;
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
        Some((attenuation, Ray::new(inter.point, direction, ray.time)))
    }

    fn emitted(&self, _uv: (f32, f32), _inter: &Intersection) -> Color {
        BLACK
    }

    fn albedo(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        BLACK
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct EmissiveDiffuse {
    texture: Texture,
}

impl EmissiveDiffuse {
    pub fn new(texture: Texture) -> Material {
        Material::EmissiveDiffuse(EmissiveDiffuse { texture })
    }
}

impl Tracable for EmissiveDiffuse {
    fn scatter(&self, _ray: &Ray, _inter: &Intersection) -> Option<(Color, Ray)> {
        None
    }

    fn emitted(&self, _uv: (f32, f32), inter: &Intersection) -> Color {
        // let outward_normal = inter.outward_normal;
        let uv = inter.uv;
        self.texture.get_color_uv(uv, inter.point)
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

#[allow(dead_code)]
#[derive(Copy, Clone, Debug)]
pub struct Isotropic {
    texture: Texture,
}

impl Isotropic {
    pub fn new(texture: Texture) -> Material {
        Material::Isotropic(Isotropic { texture })
    }
}

impl Tracable for Isotropic {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        Some((
            self.albedo(inter.uv, inter.point),
            Ray::new(
                inter.point,
                random_sphere_distribution().normalize(),
                ray.time,
            ),
        ))
    }

    fn emitted(&self, _uv: (f32, f32), _inter: &Intersection) -> Color {
        BLACK
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Glossy {
    pub texture: Texture,
    pub roughness: f32,
}

impl Glossy {
    pub fn new(texture: Texture, roughness: f32) -> Material {
        Material::Glossy(Glossy { texture, roughness })
    }
}

// not working :(
impl Tracable for Glossy {
    fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
        // let outward_normal = inter.outward_normal;

        let attenuation = self.texture.get_color_uv(inter.uv, inter.point);

        // let mut scatter_dir = inter.point + inter.normal + random_sphere_distribution().normalize();
        // if scatter_dir.near_zero() {
        //     scatter_dir = inter.normal;
        // }

        // let reflected = Metal::reflect(ray.direction.normalize(), inter.normal);

        // let combined = (scatter_dir + reflected / self.roughness).normalize();
        Some((attenuation, Ray::new(inter.point, inter.normal, ray.time)))
        // if random_distribution() > 0.5 {
        //     // let direction = reflected + self.roughness * random_sphere_distribution().normalize();

        //     Some((attenuation, Ray::new(inter.point, reflected, ray.time)))
        // } else {
        //     let normal = inter.normal;

        //     Some((
        //         attenuation,
        //         Ray::new(inter.point, scatter_dir - inter.point, ray.time),
        //     ))
        // }
    }

    fn emitted(&self, _uv: (f32, f32), _inter: &Intersection) -> Color {
        BLACK
    }

    fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
        self.texture.get_color_uv(uv, point)
    }
}

// #[allow(dead_code)]
// #[derive(Copy, Clone, Debug)]
// pub struct Reflective {
//     texture: Texture,
// }

// impl Reflective {
//     pub fn new(texture: Texture) -> Material {
//         Material::Isotropic(Isotropic { texture })
//     }
// }

// impl Tracable for Reflective {
//     fn scatter(&self, ray: &Ray, inter: &Intersection) -> Option<(Color, Ray)> {
//         let normal = inter.normal;

//         let reflected = Metal::reflect(ray.direction.normalize(), normal);
//         // let outward_normal = inter.outward_normal;
//         let uv = inter.uv;

//         if reflected.dot(&normal) > 0.0 {
//             Some((
//                 self.texture.get_color_uv(uv, inter.point),
//                 Ray::new(inter.point, reflected, ray.time),
//             ))
//         } else {
//             None
//         }
//     }

//     fn emitted(&self, _uv: (f32, f32), _inter: &Intersection) -> Color {
//         Color::new(0.0, 0.0, 0.0)
//     }

//     fn albedo(&self, uv: (f32, f32), point: Vec3) -> Color {
//         self.texture.get_color_uv(uv, point)
//     }
// }
