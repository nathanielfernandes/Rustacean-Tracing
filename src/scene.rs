extern crate image;
use crate::color::*;
use crate::intersection::Intersection;
use crate::lights::Light;
use crate::materials::Material;
use crate::objects::{Object, Tracable};
use crate::vec3::Vec3;
use crate::Ray;
use image::{ImageBuffer, RgbImage};
use rayon::prelude::*;
use std::f32::consts::PI;
use std::time::Instant;

pub struct Scene {
    pub width: u32,
    pub height: u32,
    pub fov: f64,
    pub cam_pos: Vec3,
    pub max_depth: u32,
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
        shadow_bias: f64,
    ) -> Scene {
        Scene {
            width,
            height,
            fov,
            cam_pos,
            max_depth,
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
            .min_by(|inter1, inter2| inter1.distance.partial_cmp(&inter2.distance).unwrap())
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

    fn diffuse_shading(&self, material: &Material, point: Vec3, surf_norm: Vec3) -> Color {
        let mut diffuse_color = BLACK;
        let mut specular_highlight = BLACK;

        let reflected = material.albedo / PI as f64;

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

    fn compute_color(&self, ray: &Ray, intersection: &Intersection, depth: u32) -> Color {
        let point = ray.origin + (ray.direction * intersection.distance);
        let surf_norm = intersection.object.surface_normal(&point);

        let material = intersection.object.material();

        let mut final_color = self.diffuse_shading(material, point, surf_norm);

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
