
use std::f64::consts::PI;

use image::RgbImage;

use crate::datatypes::Ray;
use crate::datatypes::Color3;
use crate::datatypes::Hittable;
use crate::datatypes::Interval;
use crate::datatypes::HitRecord;
use crate::shapes::Hittables;
use crate::materials::Material;
use crate::materials::Materials;


pub struct ColorUtil;
#[allow(unused)]
impl ColorUtil {
    const INTENSITY: Interval = Interval { min: 0.0, max: 0.999 };
    pub fn get_color_str(color: &Color3) -> String {
        let ir = (256.0 * ColorUtil::INTENSITY.clamp(color.x)) as i32;
        let ig = (256.0 * ColorUtil::INTENSITY.clamp(color.y)) as i32;
        let ib = (256.0 * ColorUtil::INTENSITY.clamp(color.z)) as i32;

        format!("{ir} {ig} {ib}\n")
    }

    pub fn get_pixel(color: &Color3) -> image::Rgb<u8> {
        let gr = MathUtil::linear_to_gamma(color.x);
        let gg = MathUtil::linear_to_gamma(color.y);
        let gb = MathUtil::linear_to_gamma(color.z);
        let ir = (256.0 * ColorUtil::INTENSITY.clamp(gr)) as u8;
        let ig = (256.0 * ColorUtil::INTENSITY.clamp(gg)) as u8;
        let ib = (256.0 * ColorUtil::INTENSITY.clamp(gb)) as u8;
        image::Rgb([ir, ig, ib])
    }
}


pub struct ImageUtil;
#[allow(unused)]
impl ImageUtil {
    pub fn get_rgb_image(pixels: Vec<Color3>, width: u32, height: u32) -> RgbImage {
        let mut image: RgbImage = RgbImage::new(width, height);
        for j in 0..height {
            for i in 0..width {
                let pixel = &pixels[(j * width + i) as usize];
                image.put_pixel(i, j, ColorUtil::get_pixel(pixel));
            }
        }

        image
    }

    pub fn get_ppm_image(pixels: Vec<Color3>, width: u32, height: u32) -> String {
        let mut content = String::new();
        content.push_str("P3\n");   // Define ASCII color mode.
        content.push_str(&format!("{width} {height}\n"));     // Dimensions.
        content.push_str("255\n");  // Set max color

        for j in 0..height {
            for i in 0..width {
                let pixel = &pixels[(j * width + i) as usize];
                content.push_str(&ColorUtil::get_color_str(pixel));
            }
        }

        content
    }

}


pub struct HitUtil;
impl HitUtil {
    pub fn hit(hittable: &Hittables, ray: &crate::datatypes::Ray, t_i: &Interval) -> Option<HitRecord> {
        match hittable {
            Hittables::Sphere(sphere) => sphere.hit(ray, t_i),
            Hittables::HittableList(list) => list.hit(ray, t_i),
        }
    }
}


pub struct MatUtil;
impl MatUtil {
    pub fn scatter(material: &Materials, ray: &Ray, hit_rec: &HitRecord) -> Option<(Color3, Ray)> {
        match material {
            Materials::DifuseLamb(mat) => mat.scatter(ray, hit_rec),
            Materials::Metal(mat) => mat.scatter(ray, hit_rec),
        }
    }
}


pub struct MathUtil;
impl MathUtil {
    pub fn degrees_to_radians(degrees: f64) -> f64 {
        degrees * PI / 180.0
    }
    pub fn rand() -> f64 {
        rand::random::<f64>()
    }
    pub fn rand_ran(min: f64, max: f64) -> f64 {
        min + (max - min) * MathUtil::rand()
    }
    pub fn linear_to_gamma(linear: f64) -> f64 {
        if linear > 0.0 {
            f64::sqrt(linear)
        }
        else {
            0.0
        }
    }
}