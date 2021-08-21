use crate::Scene;
use crate::Vec3;

pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
}

impl Ray {
    pub fn prime(x: u32, y: u32, scene: &Scene) -> Ray {
        let fov_adjustment = (scene.fov.to_radians() / 2.0).tan();
        let aspect_ratio = (scene.width as f64) / (scene.height as f64);

        let prime_x =
            ((((x as f64 + 0.5) / scene.width as f64) * 2.0 - 1.0) * aspect_ratio) * fov_adjustment;
        let prime_y = (1.0 - ((y as f64 + 0.5) / scene.height as f64) * 2.0) * fov_adjustment;

        Ray {
            origin: scene.cam_pos,
            direction: Vec3 {
                x: prime_x,
                y: prime_y,
                z: -1.0,
            }
            .normalize(),
        }
    }

    pub fn reflection(normal: Vec3, incident: Vec3, point: Vec3, shadow_bias: f64) -> Ray {
        Ray {
            // add shadow bias
            origin: point + (normal * shadow_bias),
            direction: incident - (2.0 * incident.dot(&normal) * normal),
        }
    }

    // pub fn transmission(
    //     normal: Vec3,
    //     incident: Vec3,
    //     point: Vec3,
    //     shadow_bias: f64,
    //     refractive_index: f64,
    // ) -> Option<Ray> {
    //     let mut ref_n = normal;
    //     let mut eta_t = refractive_index;
    //     let mut eta_i = 1.0;
    //     let mut i_dot_n = incident.dot(&normal);

    //     if i_dot_n < 0.0 {
    //         i_dot_n = -i_dot_n;
    //     } else {
    //         ref_n = -normal;
    //         eta_i = eta_t;
    //         eta_t = 1.0;
    //     }

    //     let eta = eta_i / eta_t;
    //     let k = 1.0 - (eta * eta) * (1.0 - i_dot_n * i_dot_n);
    //     if k < 0.0 {
    //         None
    //     } else {
    //         Some(Ray {
    //             origin: point + (ref_n * -shadow_bias),
    //             direction: (incident + i_dot_n * ref_n) * eta - ref_n * k.sqrt(),
    //         })
    //     }
    // }

    // pub fn fresnel(incident: Vec3, normal: Vec3, refractive_index: f64) -> f64 {
    //     let i_dot_n = incident.dot(&normal);
    //     let mut eta_i = 1.0;
    //     let mut eta_t = refractive_index;
    //     if i_dot_n > 0.0 {
    //         eta_i = eta_t;
    //         eta_t = 1.0;
    //     }

    //     let sin_t = eta_i / eta_t * (1.0 - i_dot_n * i_dot_n).max(0.0).sqrt();
    //     if sin_t > 1.0 {
    //         return 1.0;
    //     } else {
    //         let cos_t = (1.0 - sin_t * sin_t).max(0.0).sqrt();
    //         let cos_i = cos_t.abs();
    //         let r_s = ((eta_t * cos_i) - (eta_i * cos_t)) / ((eta_t * cos_i) + (eta_i * cos_t));
    //         let r_p = ((eta_i * cos_i) - (eta_t * cos_t)) / ((eta_i * cos_i) + (eta_t * cos_t));
    //         return (r_s * r_s + r_p * r_p) / 2.0;
    //     }
    // }
}
