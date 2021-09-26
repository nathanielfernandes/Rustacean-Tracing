use crate::{materials::Material, objects::Object, ray::Ray, vec3::Vec3};

#[derive(Clone, Debug, Copy)]
pub struct Intersection<'trace> {
    pub distance: f32,
    pub point: Vec3,
    pub normal: Vec3,
    pub outward_normal: Vec3,
    pub mat: &'trace Material,
    pub uv: (f32, f32),
}

impl<'trace> Intersection<'trace> {
    pub fn new<'traced>(
        distance: f32,
        point: Vec3,
        normal: Vec3,
        outward_normal: Vec3,
        mat: &'traced Material,
        uv: (f32, f32),
    ) -> Intersection<'traced> {
        Intersection {
            distance,
            point,
            normal,
            outward_normal,
            mat: &mat,
            uv,
        }
    }
}

// #[derive(Clone, Debug, Copy)]
// pub struct Intersection<'trace> {
//     pub distance: f32,
//     pub object: &'trace Object,
// }

// impl<'trace> Intersection<'trace> {
//     pub fn new<'traced>(distance: f32, object: &'traced Object) -> Intersection<'traced> {
//         Intersection { distance, object }
//     }
// }
