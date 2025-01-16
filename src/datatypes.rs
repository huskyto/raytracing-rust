
use core::f32;
use std::fmt::Display;
use std::ops::Sub;
use std::ops::Neg;
use std::ops::Index;
use std::ops::Add;
use std::ops::AddAssign;
use std::ops::Mul;
use std::ops::MulAssign;
use std::ops::Div;
use std::ops::DivAssign;

pub type Point3 = Vec3;
pub type Color3 = Vec3;

#[derive(Clone, PartialEq, Debug)]
pub struct Vec3 { 
    pub x: f32,
    pub y: f32,
    pub z: f32,
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
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }
    pub fn len_sqr(&self) -> f32 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
    pub fn len(&self) -> f32 {
        f32::sqrt(self.len_sqr())
    }
    pub fn dot(&self, rhs: &Vec3) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
    pub fn cross(&self, rhs: Vec3) -> Vec3 {
        Vec3::new(self.y * rhs.z - self.z * rhs.y,
                  self.z * rhs.x - self.x * rhs.z,
                  self.x * rhs.y - self.y * rhs.x)
    }
    pub fn unit(&self) -> Vec3 {
        self / self.len()
    }
}

impl Index<usize> for Vec3 {
    type Output = f32;

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
impl Mul<f32> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<f32> for &Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: f32) -> Self::Output {
        Vec3::new(self.x * rhs, self.y * rhs, self.z * rhs)
    }
}
impl Mul<&Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: &Vec3) -> Self::Output {
        rhs * self
    }
}
impl Mul<Vec3> for f32 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        rhs * self
    }
}
impl MulAssign<f32> for Vec3 {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
    }
}

impl Div<f32> for Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl Div<f32> for &Vec3 {
    type Output = Vec3;

    fn div(self, rhs: f32) -> Self::Output {
        self * (1.0 / rhs)
    }
}
impl DivAssign<f32> for Vec3 {
    fn div_assign(&mut self, rhs: f32) {
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
    pub fn at(&self, t: f32) -> Point3 {
        &self.origin + &(&self.direction * t)
    }
}


pub trait Hittable {
    fn hit(&self, ray: &Ray, t_i: &Interval) -> Option<HitRecord>;
}

pub struct HitRecord {
    pub p: Point3,
    pub normal: Vec3,
    pub t: f32,
    pub is_front_face: bool
}
impl HitRecord {
    pub fn new(p: Point3, normal: Vec3, t: f32) -> Self {
        Self { p, normal, t, is_front_face: false }
    }
    pub fn set_face_normal(&mut self, ray: &Ray, outward_normal: &Vec3) {
                // Sets the hit record normal vector.
                // outward_normal is assumed to have unit length.
        self.is_front_face = ray.direction().dot(outward_normal) < 0.0;
        self.normal = if self.is_front_face { outward_normal.clone() } else { -outward_normal };
    }
}


pub struct Interval {
    pub min: f32,
    pub max: f32
}
impl Interval {
    pub fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }
    pub fn len(&self) -> f32 {
        self.max - self.min
    }
    pub fn contains(&self, x: f32) -> bool {
        self.min <= x && x <= self.max
    }
    pub fn surrounds(&self, x: f32) -> bool {
        self.min < x && x < self.max
    }
    const EMPTY: Interval = Self { min: f32::INFINITY, max: f32::NEG_INFINITY };
    const UNIVERSE: Interval = Self { min: f32::NEG_INFINITY, max: f32::INFINITY };
}