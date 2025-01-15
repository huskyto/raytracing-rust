
use crate::datatypes::Point3;

pub struct Sphere {
    pub radius: f32,
    pub center: Point3
}
impl Sphere {
    pub fn new(radius: f32, x: f32, y: f32, z: f32) -> Sphere {
        Sphere {
            radius,
            center: Point3::new(x, y, z)
        }
    }
}

