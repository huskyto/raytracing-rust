
use crate::datatypes::Ray;
use crate::datatypes::Vec3;
use crate::datatypes::Color3;
use crate::datatypes::HitRecord;



#[derive(Clone)]
pub enum Materials {
    DifuseLamb(MatLambertian),
    Metal(MatMetal)
}


pub trait Material {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)>;
}

#[derive(Clone)]
pub struct MatLambertian {
    pub albedo: Color3
}
impl MatLambertian {
    pub fn new(albedo: Color3) -> Self {
        MatLambertian { albedo }
    }
    pub const GRAY: MatLambertian = MatLambertian { albedo: Color3 { x: 0.5, y: 0.5, z: 0.5 } };
}
impl Material for MatLambertian {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)> {
        let mut sc_direction = &hit_rec.normal + &Vec3::random_unit();
        if sc_direction.is_near_zero() {
            sc_direction = hit_rec.normal.clone();
        }
        let sc_ray = Ray::new(hit_rec.p.clone(), sc_direction);
        Option::Some((self.albedo.clone(), sc_ray))
    }
}


#[derive(Clone)]
pub struct MatMetal {
    pub albedo: Color3
}
impl MatMetal {
    pub fn new(albedo: Color3) -> Self {
        MatMetal { albedo }
    }
}
impl Material for MatMetal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)> {
        let reflected = ray.direction().reflect(&hit_rec.normal);
        let sc_ray = Ray::new(hit_rec.p.clone(), reflected);
        Some((self.albedo.clone(), sc_ray))
    }
}