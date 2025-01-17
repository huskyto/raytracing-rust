
use crate::datatypes::Ray;
use crate::datatypes::Vec3;
use crate::datatypes::Color3;
use crate::datatypes::HitRecord;
use crate::utils::MathUtil;



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
    fn reflectance(cos: f64, ir: f64) -> f64 {
        let mut r0 = (1.0 - ir) / (1.0 + ir);
        r0 = r0 * r0;
        r0 + (1.0 - r0) * f64::powi(1.0 - cos, 5)
    }
}
impl Material for MatDielectric {
    fn scatter(&self, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)> {
        let attenuation = Color3::one();
        let ri = if hit_rec.is_front_face { 1.0 / self.ir } else { self.ir };
        let unit_dir = ray.direction().unit();

        let cos_theta = f64::min(1.0, hit_rec.normal.dot(&-&unit_dir));
        let sin_theta = f64::sqrt(1.0 - cos_theta * cos_theta);

        let cannot_refract = ri * sin_theta > 1.0;

        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > MathUtil::rand() {
            unit_dir.reflect(&hit_rec.normal)
        }
        else {
            unit_dir.refract(&hit_rec.normal, ri)
        };

        let sc_ray = Ray::new( hit_rec.p.clone(), direction);
        Some((attenuation, sc_ray))
    }
}