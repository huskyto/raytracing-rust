
use crate::{datatypes::{HitRecord, Hittable, Point3, Ray}, utils::HitUtil};

#[derive(Clone)]
pub enum Hittables {
    Sphere(Sphere),
    HittableList(HittableList)
}

#[derive(Clone)]
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


#[derive(Clone)]
pub struct HittableList {
    pub objects: Vec<Hittables>
}
impl HittableList {
    pub fn new() -> Self {
        Self { objects: Vec::new() }
    }
    pub fn add(&mut self, object: Hittables) {
        self.objects.push(object);
    }
}
impl Hittable for HittableList {
    fn hit(&self, ray: &crate::datatypes::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        // let mut tmp_rec: HitRecord;
        // let mut hit_anything = false;
        let mut closest_so_far = t_max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hr) = HitUtil::hit(object, ray, t_min, t_max) {
            // if let Some(hr) = object.hit(ray, t_min, t_max) {
                // hit_anything = true;
                if hr.t < closest_so_far {
                    closest_so_far = hr.t;
                    rec = Some(hr);
                }
            }
        }

        rec
    }
}
