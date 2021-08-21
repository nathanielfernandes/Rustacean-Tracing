use crate::color::Color;
use crate::Vec3;
use ::std::f32::consts::PI;
pub struct DirectionalLight {
    pub direction: Vec3,
    pub color: Color,
    pub intensity: f64,
}

impl DirectionalLight {
    pub fn new(direction: Vec3, color: Color, intensity: f64) -> Light {
        Light::DirectionalLight(DirectionalLight {
            direction,
            color,
            intensity,
        })
    }
}

pub struct SphericalLight {
    pub pos: Vec3,
    pub color: Color,
    pub intensity: f64,
}

impl SphericalLight {
    pub fn new(pos: Vec3, color: Color, intensity: f64) -> Light {
        Light::SphericalLight(SphericalLight {
            pos,
            color,
            intensity,
        })
    }
}

pub enum Light {
    DirectionalLight(DirectionalLight),
    SphericalLight(SphericalLight),
}

impl Light {
    pub fn color(&self) -> Color {
        match *self {
            Light::DirectionalLight(ref d) => d.color,
            Light::SphericalLight(ref d) => d.color,
        }
    }

    pub fn rel_direction(&self, point: &Vec3) -> Vec3 {
        match *self {
            Light::DirectionalLight(ref d) => -d.direction,
            Light::SphericalLight(ref d) => (d.pos - *point).normalize(),
        }
    }

    pub fn intensity(&self, point: &Vec3) -> f64 {
        match *self {
            Light::DirectionalLight(ref d) => d.intensity,
            Light::SphericalLight(ref s) => {
                let r2 = (s.pos - *point).norm();
                s.intensity / (4.0 * PI as f64 * r2)
            }
        }
    }

    pub fn rel_distance(&self, point: &Vec3) -> f64 {
        match *self {
            Light::DirectionalLight(_) => ::std::f64::INFINITY,
            Light::SphericalLight(ref s) => (s.pos - *point).length(),
        }
    }
}
