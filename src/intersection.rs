use crate::objects::Object;

pub struct Intersection<'trace> {
    pub distance: f64,
    pub object: &'trace Object,
}

impl<'trace> Intersection<'trace> {
    pub fn new<'traced>(distance: f64, object: &'traced Object) -> Intersection<'traced> {
        Intersection { distance, object }
    }
}
