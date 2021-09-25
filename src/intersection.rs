use crate::objects::Object;

#[derive(Clone, Debug, Copy)]
pub struct Intersection<'trace> {
    pub distance: f32,
    pub object: &'trace Object,
}

impl<'trace> Intersection<'trace> {
    pub fn new<'traced>(distance: f32, object: &'traced Object) -> Intersection<'traced> {
        Intersection { distance, object }
    }
}
