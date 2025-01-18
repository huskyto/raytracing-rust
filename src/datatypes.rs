
use core::f64;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Add;
use std::ops::Mul;
use std::ops::Div;
use std::ops::Index;
use std::ops::AddAssign;
use std::ops::MulAssign;
use std::ops::DivAssign;
use std::fmt::Display;

use crate::utils::MathUtil;
use crate::materials::Materials;

pub type Point3 = Vec3;
pub type Color3 = Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Vec3 { 
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn zero() -> Self {
        Self::new(0.0, 0.0, 0.0)
    }
    pub fn one() -> Self {
        Self::new(1.0, 1.0, 1.0)
    }
    pub fn x_u() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }
    pub fn y_u() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }
    pub fn z_u() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Self { x, y, z }
    }
    pub fn random() -> Self {
        Self::new(MathUtil::rand(), MathUtil::rand(), MathUtil::rand())
    }
    pub fn random_ran(min: f64, max: f64) -> Self {
        Self::new(MathUtil::rand_ran(min, max), MathUtil::rand_ran(min, max), MathUtil::rand_ran(min, max))
    }
    pub fn random_unit() -> Self {
        loop {
            let p = Self::random_ran(-1.0, 1.0);
            let lensq = p.len_sqr();
            if 1e-160 < lensq && lensq <= 1.0 {  // Smaller than 1e-160 underflows to zero when squared.
                 return p / f64::sqrt(lensq);
            }
        }
    }
    pub fn random_on_hemisphere(normal: &Vec3) -> Self {
        let on_unit_sphere = Self::random_unit();
        if on_unit_sphere.dot(normal) > 0.0 {
            on_unit_sphere
        } else {
            -&on_unit_sphere
        }
    }
    pub fn random_in_unit_disk() -> Self {
        loop {
            let p = Vec3::new(MathUtil::rand_ran(-1.0, 1.0), MathUtil::rand_ran(-1.0, 1.0), 0.0);
            if p.len_sqr() < 1.0 {
                return p;
            }
        }
    }
    pub fn len_sqr(&self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f64 {
        f64::sqrt(self.len_sqr())
    }
    pub fn dot(&self, rhs: &Vec3) -> f64 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: &Vec3) -> Vec3 {
        Vec3::new(self.y * rhs.z - self.z * rhs.y,
                  self.z * rhs.x - self.x * rhs.z,
                  self.x * rhs.y - self.y * rhs.x)
    }
    pub fn unit(&self) -> Vec3 {
        self / self.len()
    }
    pub fn is_near_zero(&self) -> bool {
        let s = 1e-8;
        self.x.abs() < s && self.y.abs() < s && self.z.abs() < s
    }
    pub fn reflect(&self, normal: &Vec3) -> Vec3 {
        self - &(2.0 * normal * self.dot(normal))
    }
    pub fn refract(&self, n: &Vec3, etai_over_etat: f64) -> Vec3 {
        let cos_theta = f64::min(n.dot(&-self), 1.0);
        let r_out_perp = etai_over_etat * (self + &(cos_theta * n));
        let r_out_parallel = -f64::sqrt(f64::abs(1.0 - r_out_perp.len_sqr())) * n;
        r_out_perp + r_out_parallel
    }
    pub fn flip(&mut self) {
        self.x = -self.x;
        self.y = -self.y;
        self.z = -self.z;
    }
}

impl Index<usize> for Vec3 {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        if index == 0 { &self.x }
        else if index == 1 { &self.y }
        else if index == 2 { &self.z }
        else {
            panic!("Index out of bounds: {index}")
        }
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl Add for &Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x + rhs.x, self.y + rhs.y, self.z + rhs.z)
    }
}
impl AddAssign for Vec3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Self::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Self) -> Self::Output {
        Vec3::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}
impl Neg for &Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3::new(-self.x, -self.y, -self.z)
    }
}

    // TODO: Does this make any sense?
impl Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3::new(self.x * rhs.x, self.y * rhs.y, self.z * rhs.z)
    }
}
impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<f64> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f64) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}
impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, rhs: f64) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl Div<f64> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, rhs: f64) {
        // self + self;
        *self *= 1.0/rhs;
    }
}


impl Display for Vec3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{} {} {}", self.x, self.y, self.z)
    }
}


pub struct Ray {
    origin: Point3,
    direction: Vec3
}
impl Ray {
    pub fn new(origin: Point3, direction: Vec3) -> Self {
        Self { origin, direction }
    }
    pub fn origin(&self) -> &Point3 {
        &self.origin
    }
    pub fn direction(&self) -> &Vec3 {
        &self.direction
    }
    pub fn at(&self, t: f64) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, t_i: &Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub material: Materials,
    pub t: f64,
    pub is_front_face: bool
}
impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f64, material: Materials) -> Self {
        Self { p, normal, material, t, is_front_face: false }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
                // Sets the hit record normal vector.
                // outward_normal is assumed to have unit length.
        self.is_front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.is_front_face { outward_normal.clone() } else { -outward_normal };
    }
}


pub struct Interval {
    pub min: f64,
    pub max: f64
}
impl Interval {
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }
    pub fn len(&self) -> f64 {
        self.max - self.min
    }
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }
    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min { self.min }
        else if x > self.max { self.max }
        else { x }
    }
    const EMPTY: Interval = Self { min: f64::INFINITY, max: f64::NEG_INFINITY };
    const UNIVERSE: Interval = Self { min: f64::NEG_INFINITY, max: f64::INFINITY };
}

