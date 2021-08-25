use crate::color::*;
//use crate::intersection::Intersection;
//use crate::objects::Intersectable;
use crate::ray::Ray;
use crate::Object;
use crate::Vec3;
use image::imageops::flip_vertical;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
use rand::Rng;
use rayon::prelude::*;
// /use std::cmp::Ordering;
use std::time::Instant;

pub fn random_float(i: f64, j: f64) -> f64 {
    let mut rng = rand::thread_rng();
    rng.gen_range(i..j)
}

pub fn random_distribution() -> f64 {
    random_float(0.0_f64, 1.0_f64)
}

pub fn random_sphere_distribution() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_distribution(),
            random_distribution(),
            random_distribution(),
        );
        if p.norm() >= 1.0 {
            continue;
        }
        return p;
    }
}

pub fn random_hemisphere_distribution(normal: Vec3) -> Vec3 {
    let us = random_sphere_distribution();
    if us.dot(&normal) > 0.0 {
        us
    } else {
        -us
    }
}

pub fn random_in_unit_disk() -> Vec3 {
    loop {
        let p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
        if p.norm() >= 1.0 {
            continue;
        };
        return p;
    }
}

pub struct Camera {
    pub origin: Vec3,
    pub llc: Vec3,
    pub horizontal: Vec3,
    pub vertical: Vec3,
    pub u: Vec3,
    pub v: Vec3,
    pub w: Vec3,
    pub lens_radius: f64,
    pub aspect_ratio: f64,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f64,
        aspect_ratio: f64,
        aperture: f64,
        focus_dist: f64,
    ) -> Camera {
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();

        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(&w).normalize();
        let v = w.cross(&u);

        let origin = lookfrom;
        let horizontal = focus_dist * viewport_width * u;
        let vertical = focus_dist * viewport_height * v;
        let llc = origin - horizontal / 2.0 - vertical / 2.0 - focus_dist * w;

        let lens_radius = aperture / 2.0;
        Camera {
            origin,
            llc,
            horizontal,
            vertical,
            u,
            v,
            w,
            lens_radius,
            aspect_ratio,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        Ray {
            origin: self.origin + offset,
            direction: self.llc + s * self.horizontal + t * self.vertical - self.origin - offset,
        }
    }

    pub fn render(
        &self,
        objects: &Vec<Object>,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f64 / self.aspect_ratio) as u32;

        let mut img = RgbImage::new(width, height);

        let bar = ProgressBar::new((width * height) as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} pixels"),
        );

        println!(
            "Rendering {}x{} at {} samples per pixel with a max depth of {}",
            width, height, samples_per_pixel, max_depth
        );
        let t1 = Instant::now();
        (0..height).for_each(|y| {
            (0..width).for_each(|x| {
                let mut final_color = Color::new(0.0, 0.0, 0.0);

                (0..samples_per_pixel).for_each(|_| {
                    let u = (random_distribution() + x as f64) / (width - 1) as f64;
                    let v = (random_distribution() + y as f64) / (height - 1) as f64;

                    let r = self.get_ray(u, v);

                    final_color = final_color + r.color(objects, max_depth);
                });
                img.put_pixel(
                    x,
                    height - 1 - y,
                    (final_color / samples_per_pixel as f64).sqrt().to_rgb(),
                );

                bar.inc(1);
            });
        });

        bar.finish();
        println!("Took {:?}", t1.elapsed());
        img
    }

    pub fn threaded_render_v2(
        &self,
        objects: &Vec<Object>,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f64 / self.aspect_ratio) as u32;
        let mut img = RgbImage::new(width, height);

        let bar = ProgressBar::new((height * width) as u64 + 1);
        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} pixel"),
        );

        println!(
            "Rendering {}x{} at {} samples per pixel with a max depth of {}",
            width, height, samples_per_pixel, max_depth
        );
        let t1 = Instant::now();

        img.par_chunks_mut(3).enumerate().for_each(|(i, slab)| {
            let mut final_color = Color::new(0.0, 0.0, 0.0);

            (0..samples_per_pixel).for_each(|_| {
                let u = (random_distribution() + (i as u32 % width) as f64) / (width - 1) as f64;
                let v = (random_distribution() + (i as u32 / width) as f64) / (height - 1) as f64;

                let r = self.get_ray(u, v);

                final_color = final_color + r.color(objects, max_depth);
            });
            slab.copy_from_slice(&(final_color / samples_per_pixel as f64).sqrt().to_slice());

            bar.inc(1);
        });

        img = flip_vertical(&img);

        bar.finish();
        println!("Took {:?}", t1.elapsed());
        img
    }

    pub fn threaded_render(
        &self,
        row_h: u32,
        objects: &Vec<Object>,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f64 / self.aspect_ratio) as u32;
        let chunk_size = width * 3 * row_h;

        let mut img = RgbImage::new(width, height);

        let bar = ProgressBar::new((height / row_h) as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.cyan/blue} {pos:>7}/{len:7} chunk"),
        );

        println!(
            "Rendering {}x{} at {} samples per pixel with a max depth of {}",
            width, height, samples_per_pixel, max_depth
        );
        let t1 = Instant::now();

        bar.inc(0);
        img.par_chunks_mut(chunk_size as usize)
            .enumerate()
            .for_each(|(i, slab)| {
                slab.copy_from_slice(&self.render_slab(
                    objects,
                    row_h * i as u32,
                    row_h,
                    width,
                    height,
                    samples_per_pixel,
                    max_depth,
                ));

                bar.inc(1);
            });

        img = flip_vertical(&img);

        bar.finish();
        println!("Took {:?}", t1.elapsed());
        img
    }

    pub fn render_slab(
        &self,
        objects: &Vec<Object>,
        j: u32,
        h: u32,
        width: u32,
        height: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> Vec<u8> {
        let mut pixels: Vec<u8> = Vec::new();
        for y in j..(j + h) {
            for x in 0..width {
                let mut final_color = Color::new(0.0, 0.0, 0.0);

                (0..samples_per_pixel).for_each(|_| {
                    let u = (random_distribution() + x as f64) / (width - 1) as f64;
                    let v = (random_distribution() + y as f64) / (height - 1) as f64;

                    let r = self.get_ray(u, v);

                    final_color = final_color + r.color(objects, max_depth);
                });

                pixels
                    .extend_from_slice(&(final_color / samples_per_pixel as f64).sqrt().to_slice());
            }
        }
        pixels
    }
}
