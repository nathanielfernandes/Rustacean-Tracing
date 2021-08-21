use crate::color::Color;

pub struct Material {
    // add textures
    pub color: Color,
    pub albedo: f64,
    pub reflectivity: f64,
    pub phong_n: f64,
    pub phong_ks: f64,
    // pub transparency: f64,
    // pub refractive_index: f64,
}

impl Material {
    pub fn new(
        color: Color,
        albedo: f64,
        reflectivity: f64,
        phong_n: f64,
        phong_ks: f64,
        // transparency: f64,
        // refractive_index: f64,
    ) -> Material {
        Material {
            color,
            albedo,
            reflectivity,
            phong_n,
            phong_ks,
            // transparency,
            // refractive_index,
        }
    }

    pub fn shiny(color: Color, albedo: f64, reflectivity: f64) -> Material {
        Material {
            color,
            albedo,
            reflectivity,
            phong_n: 1250.0,
            phong_ks: 0.2,
            // transparency: 0.0,
            // refractive_index: 0.0,
        }
    }

    pub fn matte(color: Color, albedo: f64) -> Material {
        Material {
            color,
            albedo,
            reflectivity: 0.0,
            phong_n: 0.0,
            phong_ks: 0.0,
            // transparency: 0.0,
            // refractive_index: 0.0,
        }
    }
}
