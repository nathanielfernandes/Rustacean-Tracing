use crate::bvh::BvhTree;
use crate::color::*;
//use crate::intersection::Intersection;
//use crate::objects::Intersectable;
use crate::ray::Ray;
use crate::Object;
use crate::Vec3;
use image::imageops::flip_vertical;
use image::Rgb;
use image::{ImageBuffer, RgbImage};
use indicatif::{ProgressBar, ProgressStyle};
// use rand::distributions::{Distribution, Uniform};
use rand::Rng;
use rayon::prelude::*;
// /use std::cmp::Ordering;
use std::time::Instant;

// const BETWEEN: Uniform<f32> = Uniform::from(0.0_f32..1.0_f32);

pub fn random_int(i: u32, j: u32) -> u32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(i..j)
}

pub fn random_float(i: f32, j: f32) -> f32 {
    let mut rng = rand::thread_rng();
    rng.gen_range(i..j)
}

pub fn random_distribution() -> f32 {
    rand::random()
    // let mut rng = rand::thread_rng();
    // BETWEEN.sample(&mut rng)
}

// pub fn random_in_unit_disk() -> Vec3 {
//     let mut p;
//     loop {
//         p = Vec3::new(random_float(-1.0, 1.0), random_float(-1.0, 1.0), 0.0);
//         if p.norm() >= 1.0 {
//             continue;
//         }
//         return p;
//     }
// }

// pub fn random_unit_in_sphere() {
//     let mut p;
//     loop {
//         p = Vec3::new(
//             random_distribution(),
//             random_distribution(),
//             random_distribution(),
//         );

//     }
// }

pub fn random_sphere_distribution() -> Vec3 {
    loop {
        let p = Vec3::new(
            random_float(-1.0, 1.0),
            random_float(-1.0, 1.0),
            random_float(-1.0, 1.0),
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
    pub lens_radius: f32,
    pub aspect_ratio: f32,
    pub time_0: f32,
    pub time_1: f32,
}

impl Camera {
    pub fn new(
        lookfrom: Vec3,
        lookat: Vec3,
        vup: Vec3,
        vfov: f32,
        aspect_ratio: f32,
        aperture: f32,
        focus_dist: f32,
        time_0: f32, // shutter open
        time_1: f32, // shutter close
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
            time_0,
            time_1,
        }
    }

    pub fn get_ray(&self, s: f32, t: f32) -> Ray {
        let rd = self.lens_radius * random_in_unit_disk();
        let offset = self.u * rd.x + self.v * rd.y;

        // TODO
        Ray {
            origin: self.origin + offset,
            direction: self.llc + s * self.horizontal + t * self.vertical - self.origin - offset,
            time: random_float(self.time_0, self.time_1),
        }
    }

    pub fn render(
        &self,
        world: &Vec<Object>,
        background: &Color,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f32 / self.aspect_ratio) as u32;

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

        for y in 0..height {
            for x in 0..width {
                let mut final_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..samples_per_pixel {
                    let u = (random_distribution() + x as f32) / (width - 1) as f32;
                    let v = (random_distribution() + y as f32) / (height - 1) as f32;

                    let r = self.get_ray(u, v);

                    final_color = final_color + r.color(world, background, max_depth);
                }
                img.put_pixel(
                    x,
                    height - 1 - y,
                    (final_color / samples_per_pixel as f32).sqrt().to_rgb(),
                );

                bar.inc(1);
            }
        }

        bar.finish();
        println!("Took {:?}", t1.elapsed());
        img
    }
    pub fn bvh_render(
        &self,
        world: &BvhTree,
        background: &Color,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        denoise_settings: Option<DenoiseSettings>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f32 / self.aspect_ratio) as u32;

        let mut img = RgbImage::new(width, height);

        let bar = &Box::new(ProgressBar::new((width * height / 64) as u64));
        bar.set_prefix("Rendering");
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%"),
        );
        let start = Instant::now();

        let pixels: Vec<u8> = (0..height)
            .into_par_iter()
            .rev()
            .flat_map(|j| {
                (0..width).into_par_iter().flat_map(move |i| {
                    let mut col = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = ((i as f32) + rand::random::<f32>()) / (width as f32);
                        let v = ((j as f32) + rand::random::<f32>()) / (height as f32);

                        let r = self.get_ray(u, v);
                        col = col + r.bvh_color(world, background, max_depth);
                    }

                    if i % 64 == 0 {
                        bar.inc(1);
                    }

                    col = (col / samples_per_pixel as f32).sqrt();
                    let v_col = col.to_vec_f32();
                    (0..3)
                        .into_par_iter()
                        .map(move |k| (255.99 * v_col[k as usize]).min(255.0) as u8)
                })
            })
            .collect();

        bar.finish();

        img.copy_from_slice(&pixels);

        println!("Finished in {:?}", start.elapsed());

        match denoise_settings {
            Some(dns) => {
                println!("Starting Denoising");
                let (albedo_buffer, normal_buffer) =
                    self.bvh_calculate_buffers(world, background, width);
                dns.denoise(img, albedo_buffer, normal_buffer)
            }
            None => img,
        }
    }

    pub fn pog_render(
        &self,
        world: &Vec<Object>,
        background: &Color,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        denoise_settings: Option<DenoiseSettings>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f32 / self.aspect_ratio) as u32;

        let mut img = RgbImage::new(width, height);

        let bar = &Box::new(ProgressBar::new((width * height / 64) as u64));
        bar.set_prefix("Rendering");
        bar.set_style(
            ProgressStyle::default_bar()
                .template("{prefix:.white} [{eta_precise}] {bar:40.cyan/blue} {percent}%"),
        );
        let start = Instant::now();

        let pixels: Vec<u8> = (0..height)
            .into_par_iter()
            .rev()
            .flat_map(|j| {
                (0..width).into_par_iter().flat_map(move |i| {
                    let mut col = Color::new(0.0, 0.0, 0.0);
                    for _s in 0..samples_per_pixel {
                        let u = ((i as f32) + rand::random::<f32>()) / (width as f32);
                        let v = ((j as f32) + rand::random::<f32>()) / (height as f32);

                        let r = self.get_ray(u, v);
                        col = col + r.color(world, background, max_depth);
                    }

                    if i % 64 == 0 {
                        bar.inc(1);
                    }

                    col = (col / samples_per_pixel as f32).sqrt();
                    let v_col = col.to_vec_f32();
                    (0..3)
                        .into_par_iter()
                        .map(move |k| (255.99 * v_col[k as usize]).min(255.0) as u8)
                })
            })
            .collect();

        bar.finish();

        img.copy_from_slice(&pixels);

        println!("Finished in {:?}", start.elapsed());

        match denoise_settings {
            Some(dns) => {
                println!("Starting Denoising");
                let (albedo_buffer, normal_buffer) =
                    self.calculate_buffers(world, background, width);
                dns.denoise(img, albedo_buffer, normal_buffer)
            }
            None => img,
        }
    }

    pub fn bvh_calculate_buffers(
        &self,
        world: &BvhTree,
        background: &Color,
        width: u32,
    ) -> (Vec<f32>, Vec<f32>) {
        let height = (width as f32 / self.aspect_ratio) as u32;

        let bar = ProgressBar::new((width * height) as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.yellow/yellow} {pos:>7}/{len:7} pixels"),
        );

        println!("Rendering Buffers {}x{}", width, height);

        let t1 = Instant::now();

        let mut normal_buffer: Vec<f32> = Vec::new();
        let mut albedo_buffer: Vec<f32> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / (width - 1) as f32;
                let v = (height - 1 - y) as f32 / (height - 1) as f32;
                let r = self.get_ray(u, v);

                let (albedo, normal) = r.bvh_buffer(world, background);
                (albedo).into_iter().for_each(|a| albedo_buffer.push(a));
                (normal).into_iter().for_each(|n| normal_buffer.push(n));
                //  albedo_buffer.copy_from_slice(&albedo);
                //normal_buffer.copy_from_slice(&normal);

                bar.inc(1);
            }
        }

        bar.finish();
        println!("Took {:?}", t1.elapsed());

        (albedo_buffer, normal_buffer)
    }

    pub fn calculate_buffers(
        &self,
        world: &Vec<Object>,
        background: &Color,
        width: u32,
    ) -> (Vec<f32>, Vec<f32>) {
        let height = (width as f32 / self.aspect_ratio) as u32;

        let bar = ProgressBar::new((width * height) as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.yellow/yellow} {pos:>7}/{len:7} pixels"),
        );

        println!("Rendering Buffers {}x{}", width, height);

        let t1 = Instant::now();

        let mut normal_buffer: Vec<f32> = Vec::new();
        let mut albedo_buffer: Vec<f32> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / (width - 1) as f32;
                let v = (height - 1 - y) as f32 / (height - 1) as f32;
                let r = self.get_ray(u, v);

                let (albedo, normal) = r.buffer(world, background);
                (albedo).into_iter().for_each(|a| albedo_buffer.push(a));
                (normal).into_iter().for_each(|n| normal_buffer.push(n));
                //  albedo_buffer.copy_from_slice(&albedo);
                //normal_buffer.copy_from_slice(&normal);

                bar.inc(1);
            }
        }

        bar.finish();
        println!("Took {:?}", t1.elapsed());

        (albedo_buffer, normal_buffer)
    }

    pub fn render_buffers(
        &self,
        world: &Vec<Object>,
        background: &Color,
        width: u32,
    ) -> (
        ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) {
        let height = (width as f32 / self.aspect_ratio) as u32;

        let mut normals = RgbImage::new(width, height);
        let mut albedos = RgbImage::new(width, height);

        let bar = ProgressBar::new((width * height) as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.yellow/yellow} {pos:>7}/{len:7} pixels"),
        );

        println!("Rendering Buffers {}x{}", width, height);

        let t1 = Instant::now();

        // let mut normal_buffer: Vec<f32> = Vec::new();
        // let mut albedo_buffer: Vec<f32> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / (width - 1) as f32;
                let v = y as f32 / (height - 1) as f32;
                let r = self.get_ray(u, v);

                let (albedo, normal) = r.buffer(world, background);

                albedos.put_pixel(
                    x,
                    height - 1 - y,
                    Rgb::from([
                        (albedo[0] * 255.0) as u8,
                        (albedo[1] * 255.0) as u8,
                        (albedo[2] * 255.0) as u8,
                    ]),
                );

                normals.put_pixel(
                    x,
                    height - 1 - y,
                    Rgb::from([
                        (0.5 * ((normal[0] + 1.0) * 255.0)) as u8,
                        (0.5 * ((normal[1] + 1.0) * 255.0)) as u8,
                        (0.5 * ((normal[2] + 1.0) * 255.0)) as u8,
                    ]),
                );

                // (albedo).into_iter().for_each(|a| albedo_buffer.push(a));
                // (normal).into_iter().for_each(|n| normal_buffer.push(n));
                //  albedo_buffer.copy_from_slice(&albedo);
                //normal_buffer.copy_from_slice(&normal);

                bar.inc(1);
            }
        }

        bar.finish();
        println!("Took {:?}", t1.elapsed());

        (albedos, normals)
    }

    pub fn bvh_render_buffers(
        &self,
        world: &BvhTree,
        background: &Color,
        width: u32,
    ) -> (
        ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        ImageBuffer<image::Rgb<u8>, Vec<u8>>,
    ) {
        let height = (width as f32 / self.aspect_ratio) as u32;

        let mut normals = RgbImage::new(width, height);
        let mut albedos = RgbImage::new(width, height);

        let bar = ProgressBar::new((width * height) as u64);

        bar.set_style(
            ProgressStyle::default_bar()
                .template("[{elapsed_precise}] {bar:50.yellow/yellow} {pos:>7}/{len:7} pixels"),
        );

        println!("Rendering Buffers {}x{}", width, height);

        let t1 = Instant::now();

        // let mut normal_buffer: Vec<f32> = Vec::new();
        // let mut albedo_buffer: Vec<f32> = Vec::new();

        for y in 0..height {
            for x in 0..width {
                let u = x as f32 / (width - 1) as f32;
                let v = y as f32 / (height - 1) as f32;
                let r = self.get_ray(u, v);

                let (albedo, normal) = r.bvh_buffer(world, background);

                albedos.put_pixel(
                    x,
                    height - 1 - y,
                    Rgb::from([
                        (albedo[0] * 255.0) as u8,
                        (albedo[1] * 255.0) as u8,
                        (albedo[2] * 255.0) as u8,
                    ]),
                );

                normals.put_pixel(
                    x,
                    height - 1 - y,
                    Rgb::from([
                        (0.5 * ((normal[0] + 1.0) * 255.0)) as u8,
                        (0.5 * ((normal[1] + 1.0) * 255.0)) as u8,
                        (0.5 * ((normal[2] + 1.0) * 255.0)) as u8,
                    ]),
                );

                // (albedo).into_iter().for_each(|a| albedo_buffer.push(a));
                // (normal).into_iter().for_each(|n| normal_buffer.push(n));
                //  albedo_buffer.copy_from_slice(&albedo);
                //normal_buffer.copy_from_slice(&normal);

                bar.inc(1);
            }
        }

        bar.finish();
        println!("Took {:?}", t1.elapsed());

        (albedos, normals)
    }

    pub fn threaded_render_v2(
        &self,
        objects: &Vec<Object>,
        background: &Color,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f32 / self.aspect_ratio) as u32;
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
                let u = (random_distribution() + (i as u32 % width) as f32) / (width - 1) as f32;
                let v = (random_distribution() + (i as u32 / width) as f32) / (height - 1) as f32;

                let r = self.get_ray(u, v);

                final_color = final_color + r.color(objects, background, max_depth);
            });
            slab.copy_from_slice(&(final_color / samples_per_pixel as f32).sqrt().to_slice());

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
        background: &Color,
        width: u32,
        samples_per_pixel: u32,
        max_depth: u32,
        denoise_settings: Option<DenoiseSettings>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let height = (width as f32 / self.aspect_ratio) as u32;
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
                    background,
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

        match denoise_settings {
            Some(dns) => {
                println!("Starting Denoising");
                let (albedo_buffer, normal_buffer) =
                    self.calculate_buffers(objects, background, width);
                dns.denoise(img, albedo_buffer, normal_buffer)
            }
            None => img,
        }
    }

    pub fn render_slab(
        &self,
        objects: &Vec<Object>,
        background: &Color,
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
                    let u = (random_distribution() + x as f32) / (width - 1) as f32;
                    let v = (random_distribution() + y as f32) / (height - 1) as f32;

                    let r = self.get_ray(u, v);

                    final_color = final_color + r.color(objects, background, max_depth);
                });

                pixels
                    .extend_from_slice(&(final_color / samples_per_pixel as f32).sqrt().to_slice());
            }
        }
        pixels
    }
}

pub struct DenoiseSettings {
    pub srgb: bool,
    pub hdr: bool,
    pub clean_aux: bool,
}

impl DenoiseSettings {
    pub fn denoise(
        &self,
        image: ImageBuffer<image::Rgb<u8>, Vec<u8>>,
        albedo_buffer: Vec<f32>,
        normal_buffer: Vec<f32>,
    ) -> ImageBuffer<image::Rgb<u8>, Vec<u8>> {
        let device = oidn::Device::new();

        let (width, height) = (image.width(), image.height());

        let input_img: Vec<f32> = image
            .into_raw()
            .into_iter()
            .map(|p| p as f32 / 255.0)
            .collect();

        let mut filter_output = vec![0.0f32; input_img.len()];

        oidn::RayTracing::new(&device)
            .srgb(self.srgb)
            .hdr(self.hdr)
            .clean_aux(self.clean_aux)
            .albedo_normal(&albedo_buffer, &normal_buffer)
            .image_dimensions(width as usize, height as usize)
            .filter(&input_img[..], &mut filter_output[..])
            .expect("Filter config error!");

        let out: Vec<u8> = filter_output
            .iter_mut()
            .map(|p| (*p * 255.0) as u8)
            .collect();

        RgbImage::from_vec(width, height, out).unwrap()
    }
}
