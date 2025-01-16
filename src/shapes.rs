
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
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        let oc = &self.center - ray.origin();
        let a = ray.direction().len_sqr();
        let h = ray.direction().dot(&oc);
        let c = oc.len_sqr() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f32::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if root <= t_min || t_max <= root {
            root = (h + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }

        let p = ray.at(root);
        let mut hit_rec = HitRecord::new(p.clone(), (&p - &self.center) / self.radius, root);
        let outward_normal = (&p - &self.center) / self.radius;
            // TODO: consider doing on init.
        hit_rec.set_face_normal(ray, &outward_normal);

        Some(hit_rec)
    }
}

