use crate::ray::Ray;
use crate::Vec3;

#[derive(Copy, Clone, Debug)]
pub struct Aabb {
    pub min: Vec3,
    pub max: Vec3,
}

impl Aabb {
    pub fn hit(&self, r: &Ray, mut tmin: f32, mut tmax: f32) -> bool {
        for a in 0..3 {
            let mint = (self.min[a] - r.origin[a]) / r.direction[a];
            let maxt = (self.max[a] - r.origin[a]) / r.direction[a];
            let t0 = mint.min(maxt);
            let t1 = mint.max(maxt);

            tmin = t0.max(tmin);
            tmax = t1.min(tmax);

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
