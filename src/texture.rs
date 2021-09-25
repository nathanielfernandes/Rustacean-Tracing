use std::sync::Arc;

use crate::{color::Color, vec3::Vec3};
use image::{DynamicImage, GenericImageView};

pub fn clamp(value: f32, lower: f32, upper: f32) -> f32 {
    value.min(upper).max(lower)
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub enum Texture {
    SolidColor(SolidColor),
    CheckerBoard(CheckerBoard),
    Image(Image),
}

impl Texture {
    pub fn get_color_uv(&self, uv: (f32, f32), point: Vec3) -> Color {
        match *self {
            Texture::SolidColor(ref tex) => tex.get_color_uv(uv, point),
            Texture::CheckerBoard(ref tex) => tex.get_color_uv(uv, point), //Texture::(ref obj) => obj.outward_normal(point, time),
            Texture::Image(ref tex) => tex.get_color_uv(uv, point),
        }
    }
}

pub trait UvMappable {
    fn get_color_uv(&self, uv: (f32, f32), point: Vec3) -> Color;
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct SolidColor {
    color: Color,
}

impl SolidColor {
    pub fn new(color: Color) -> Texture {
        Texture::SolidColor(SolidColor { color })
    }
}

impl UvMappable for SolidColor {
    fn get_color_uv(&self, _uv: (f32, f32), _point: Vec3) -> Color {
        self.color
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct CheckerBoard {
    color_1: Color,
    color_2: Color,
    scale: f32,
}

impl CheckerBoard {
    pub fn new(color_1: Color, color_2: Color, scale: f32) -> Texture {
        Texture::CheckerBoard(CheckerBoard {
            color_1,
            color_2,
            scale,
        })
    }
}

impl UvMappable for CheckerBoard {
    fn get_color_uv(&self, _uv: (f32, f32), point: Vec3) -> Color {
        let sin_v = (self.scale * point.x).sin()
            * (self.scale * point.y).sin()
            * (self.scale * point.z).sin();
        if sin_v < 0.0 {
            self.color_1
        } else {
            self.color_2
        }
    }
}

#[allow(dead_code)]
#[derive(Clone, Debug)]
pub struct Image {
    img: Arc<DynamicImage>,
    width: u32,
    height: u32,
}

impl Image {
    pub fn new(img: Arc<DynamicImage>) -> Texture {
        let width = img.width();
        let height = img.height();

        Texture::Image(Image { img, width, height })
    }
}

impl UvMappable for Image {
    fn get_color_uv(&self, uv: (f32, f32), _point: Vec3) -> Color {
        let u = clamp(uv.0, 0.0, 1.0);
        let v = 1.0 - clamp(uv.1, 0.0, 1.0);

        let mut i = (u * self.width as f32) as u32;
        let mut j = (v * self.height as f32) as u32;

        i = if i >= self.width { self.width - 1 } else { i };
        j = if j >= self.height { self.height - 1 } else { j };

        // const color_scale: f32 = 1.0 / 255.0;

        let pixel = self.img.get_pixel(j, i);

        Color::from_rgb(pixel[0], pixel[1], pixel[2])
    }
}
