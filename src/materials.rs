
use crate::datatypes::Ray;
use crate::datatypes::Vec3;
use crate::datatypes::Color3;
use crate::datatypes::HitRecord;



#[derive(Clone)]
pub enum Materials {
    DifuseLamb(MatLambertian),
    Metal(MatMetal),
    Dielectric(MatDielectric)
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
        Some((self.albedo.clone(), sc_ray))
    }
}


#[derive(Clone)]
pub struct MatMetal {
    pub albedo: Color3,
    pub fuzz: f64
}
impl MatMetal {
    pub fn new(albedo: Color3, _fuzz: f64) -> Self {
        let fuzz = if _fuzz < 1.0 { _fuzz } else { 1.0 };
        MatMetal { albedo, fuzz }
    }
}
impl Material for MatMetal {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)> {
        let mut reflected = ray.direction().reflect(&hit_rec.normal);
        reflected = reflected.unit() + (self.fuzz * Vec3::random_unit());
        let sc_ray = Ray::new(hit_rec.p.clone(), reflected);
        if sc_ray.direction().dot(&hit_rec.normal) < 0.0 {
            None
        }
        else {
            Some((self.albedo.clone(), sc_ray))
        }
    }
}


#[derive(Clone)]
pub struct MatDielectric {
    pub ir: f64
}
impl MatDielectric {
    pub fn new(ir: f64) -> Self {
        MatDielectric { ir }
    }
}
impl Material for MatDielectric {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)> {
        let attenuation = Color3::one();
        let ri = if hit_rec.is_front_face { 1.0 / self.ir } else { self.ir };
        let unit_dir = ray.direction().unit();
        let refracted = unit_dir.refract(&hit_rec.normal, ri);
        let sc_ray = Ray::new( hit_rec.p.clone(), refracted);
        Some((attenuation, sc_ray))
    }
}