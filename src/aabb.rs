use crate::ray::Ray;
use crate::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        // switched to indexing vec
        // let r_dir = r.direction.to_vec();
        // let r_origin = r.origin.to_vec();
        // let (minn, maxx) = (self.min.to_vec(), self.max.to_vec());

        for a in 0..3 {
            let mint = (self.min[a] - r.origin[a]) / r.direction[a];
            let maxt = (self.max[a] - r.origin[a]) / r.direction[a];
            let t0 = ffmin(mint, maxt);
            let t1 = ffmax(mint, maxt);

            tmin = ffmax(t0, tmin);
            tmax = ffmin(t1, tmax);

            if tmax <= tmin {
                return false;
            }
        }

        true
    }
}

pub fn surrounding_box(box_0: &Aabb, box_1: &Aabb) -> Aabb {
    let min = Vec3::new(
        (box_0.min.x).min(box_1.min.x),
        (box_0.min.y).min(box_1.min.y),
        (box_0.min.z).min(box_1.min.z),
    );
    let max = Vec3::new(
        (box_0.max.x).max(box_1.max.x),
        (box_0.max.y).max(box_1.max.y),
        (box_0.max.z).max(box_1.max.z),
    );
    Aabb { min, max }
}

pub fn ffmax(a: f32, b: f32) -> f32 {
    if a > b {
        a
    } else {
        b
    }
}

pub fn ffmin(a: f32, b: f32) -> f32 {
    if a < b {
        a
    } else {
        b
    }
}
