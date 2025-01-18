
use crate::datatypes::Ray;
use crate::datatypes::Point3;
use crate::datatypes::Interval;
use crate::datatypes::Hittable;
use crate::datatypes::HitRecord;
use crate::materials::Materials;
use crate::utils::HitUtil;

#[derive(Clone)]
pub enum Hittables {
    Sphere(Sphere),
    HittableList(HittableList)
}


pub struct ShapeFactory;
impl ShapeFactory {
    pub fn make_sphere(radius: f64, x: f64, y: f64, z: f64, material: Materials) -> Hittables {
        Hittables::Sphere(Sphere::new(radius, x, y, z, material))
    }
    pub fn make_hittable_list() -> Hittables {
        Hittables::HittableList(HittableList::new())
    }
}


#[derive(Clone)]
pub struct Sphere {
    pub radius: f64,
    pub center: Point3,
    pub material: Materials
}
impl Sphere {
    pub fn new(radius: f64, x: f64, y: f64, z: f64, material: Materials) -> Sphere {
        Sphere {
            radius,
            center: Point3::new(x, y, z),
            material
        }
    }
}
impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_i: &Interval) -> Option<HitRecord> {
        let oc = &self.center - ray.origin();
        let a = ray.direction().len_sqr();
        let h = ray.direction().dot(&oc);
        let c = oc.len_sqr() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        if discriminant < 0.0 {
            return None;
        }

        let sqrtd = f64::sqrt(discriminant);

        let mut root = (h - sqrtd) / a;
        if !t_i.contains(root) {
            root = (h + sqrtd) / a;
            if !t_i.surrounds(root) {
                return None;
            }
        }

        let p = ray.at(root);
        let mut normal = (&p - &self.center) / self.radius;
        // let outward_normal = (&p - &self.center) / self.radius;

        let is_front_face = ray.direction().dot(&normal) < 0.0;
        if !is_front_face {
            normal.flip() 
        };

        let mut hit_rec = HitRecord::new(p, normal, root, self.material.clone());
        hit_rec.is_front_face = is_front_face;

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
    fn hit(&self, ray: &crate::datatypes::Ray, t_i: &Interval) -> Option<HitRecord> {
        let mut closest_so_far = t_i.max;
        let mut rec: Option<HitRecord> = None;

        for object in &self.objects {
            if let Some(hr) = HitUtil::hit(object, ray, t_i) {
                if hr.t < closest_so_far {
                    closest_so_far = hr.t;
                    rec = Some(hr);
                }
            }
        }

        rec
    }
}
