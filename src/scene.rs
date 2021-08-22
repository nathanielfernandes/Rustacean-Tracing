extern crate image;
use crate::color::*;
use crate::intersection::Intersection;
use crate::lights::Light;
use crate::materials::Material;
use crate::objects::{Object, Tracable};
use crate::vec3::Vec3;
use crate::Ray;
use image::{ImageBuffer, RgbImage};
use rand::Rng;
use rayon::prelude::*;
use std::cmp::Ordering;
use std::f32::consts::PI;
use std::time::Instant;

const PDF: f64 = 1.0 / (2.0 * PI as f64);

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub cam_pos: Vec3,
    pub max_depth: u32,
    pub samples: u32,
    pub shadow_bias: f64,
    pub objects: Vec<Object>,
    pub lights: Vec<Light>, //pub canvas: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
}

impl Scene {
    pub fn new(
        width: u32,
        height: u32,
        fov: f64,
        cam_pos: Vec3,
        max_depth: u32,
        samples: u32,
        shadow_bias: f64,
    ) -> Scene {
        Scene {
            width,
            height,
            fov,
            cam_pos,
            max_depth,
            samples,
            shadow_bias,
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    pub fn add_obj(&mut self, object: Object) {
        self.objects.push(object)
    }

    pub fn add_light(&mut self, light: Light) {
        self.lights.push(light)
    }

    fn trace(&self, ray: &Ray) -> Option<Intersection> {
        self.objects
            .iter()
            .filter_map(|obj| {
                obj.intersects(ray)
                    .map(|distance| Intersection::new(distance, obj))
            })
            .min_by(|inter1, inter2| {
                inter1
                    .distance
                    .partial_cmp(&inter2.distance)
                    .unwrap_or(Ordering::Equal)
            })
    }

    fn cast(&self, ray: &Ray, curr_depth: u32) -> Color {
        if curr_depth >= self.max_depth {
            return BLACK;
        }
        match self.trace(ray) {
            Some(intersection) => self.compute_color(ray, &intersection, curr_depth),
            None => BLACK,
        }
    }

    fn direct_lighting(&self, material: &Material, point: Vec3, surf_norm: Vec3) -> Color {
        let mut diffuse_color = BLACK;
        let mut specular_highlight = BLACK;

        let reflected = material.albedo; // / PI as f64;

        for light in &self.lights {
            let light_dir = light.rel_direction(&point);

            let shadow_ray = Ray {
                origin: point + (surf_norm * self.shadow_bias),
                direction: light_dir,
            };

            let is_lit = self.trace(&shadow_ray);

            let intensity =
                if is_lit.is_none() || is_lit.unwrap().distance > light.rel_distance(&point) {
                    light.intensity(&point)
                } else {
                    0.0
                };

            let power = surf_norm.dot(&light_dir).max(0.0) * intensity;

            diffuse_color = diffuse_color + light.color() * power * reflected;

            let r = light_dir - 2.0 * light_dir.dot(&surf_norm) * surf_norm;
            specular_highlight = specular_highlight
                + WHITE
                    * intensity
                    * (r.dot(&-light_dir)).max(0.0).powf(material.phong_n)
                    * material.phong_ks;
        }

        material.color * (diffuse_color + specular_highlight)
    }

    fn indirect_lighting(
        &self,
        direct_lighting: Color,
        material: &Material,
        surf_norm: Vec3,
        point: Vec3,
        depth: u32,
    ) -> Color {
        // diffuse GI
        let mut indirect_lighting = BLACK;
        let mut rng = rand::thread_rng();

        let (norm_t, norm_b) = Scene::local_coord_sytem(surf_norm);

        (0..self.samples).for_each(|_| {
            let r1: f64 = rng.gen_range(0.0_f64..1.0_f64);
            let r2: f64 = rng.gen_range(0.0_f64..1.0_f64);

            let sample = Scene::uniform_sample_hemisphere(r1, r2);
            let sample_world = Vec3 {
                x: sample.x * norm_b.x + sample.y * surf_norm.x + sample.z * norm_t.x,
                y: sample.x * norm_b.y + sample.y * surf_norm.y + sample.z * norm_t.y,
                z: sample.x * norm_b.z + sample.y * surf_norm.z + sample.z * norm_t.z,
            };

            let s_ray = Ray {
                origin: point + sample_world * self.shadow_bias,
                direction: sample_world,
            };
            indirect_lighting = indirect_lighting + r1 * self.cast(&s_ray, depth + 1) / PDF;
        });

        indirect_lighting = indirect_lighting / self.samples as f64;

        material.color * (direct_lighting / PI as f64 + 2.0 * indirect_lighting) * material.albedo
    }

    fn uniform_sample_hemisphere(r1: f64, r2: f64) -> Vec3 {
        let sin_theta = (1.0 - r1 * r1).sqrt();
        let phi = 2.0 * PI as f64 * r2;
        let x = sin_theta * phi.cos();
        let z = sin_theta * phi.sin();

        Vec3 { x, y: r1, z }
    }

    fn local_coord_sytem(surf_norm: Vec3) -> (Vec3, Vec3) {
        let norm_t;
        if (surf_norm.x > surf_norm.z || surf_norm.y > surf_norm.x) && !(surf_norm.y.abs() > 0.0) {
            norm_t = Vec3 {
                x: surf_norm.z,
                y: 0.0,
                z: -surf_norm.x,
            } / (surf_norm.x * surf_norm.x + surf_norm.z * surf_norm.z).sqrt();
        } else {
            norm_t = Vec3 {
                x: 0.0,
                y: -surf_norm.z,
                z: surf_norm.y,
            } / (surf_norm.y * surf_norm.y + surf_norm.z * surf_norm.z).sqrt();
        }

        let norm_b = surf_norm.cross(&norm_t);

        (norm_t, norm_b)
    }

    fn compute_color(&self, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
        let point = ray.origin + (ray.direction * intersection.distance);
        let surf_norm = intersection.object.surface_normal(&point);

        let material = intersection.object.material();

        let direct_lighting = self.direct_lighting(material, point, surf_norm);

        let mut final_color = if self.samples > 0 {
            self.indirect_lighting(direct_lighting, material, surf_norm, point, depth)
        } else {
            direct_lighting
        };

        if material.reflectivity > 0.0 {
            let reflect_ray = Ray::reflection(surf_norm, ray.direction, point, self.shadow_bias);
            final_color = final_color * (1.0 - material.reflectivity);
            final_color =
                final_color + (self.cast(&reflect_ray, depth + 1) * material.reflectivity);
        }

        final_color
    }

    pub fn render(&self) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut canvas = RgbImage::new(self.width, self.height);

        let t1 = Instant::now();

        (0..self.width).for_each(|x| {
            (0..self.height).for_each(|y| {
                let ray = Ray::prime(x, y, &self);
                canvas.put_pixel(x, y, self.cast(&ray, 0).to_rgb())
            });
        });

        println!("Took {:?}", t1.elapsed());

        canvas
    }

    pub fn threaded_render(&self, row_h: u32) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let mut canvas = RgbImage::new(self.width, self.height);

        let chunk_size = self.width * 3 * row_h;

        let t1 = Instant::now();

        canvas
            .par_chunks_mut(chunk_size as usize)
            .enumerate()
            .for_each(|(i, slab)| {
                slab.copy_from_slice(&self.render_slab(row_h * i as u32, row_h));
            });

        println!("Took {:?}", t1.elapsed());

        canvas
    }

    pub fn render_slab(&self, j: u32, h: u32) -> Vec<u8> {
        let mut pixels: Vec<u8> = Vec::new();
        for y in j..(j + h) {
            for x in 0..self.width {
                let ray = Ray::prime(x, y, &self);
                pixels.extend_from_slice(&self.cast(&ray, 0).to_slice());
            }
        }
        pixels
    }
}
