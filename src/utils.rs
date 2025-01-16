
use std::f32::consts::PI;

use image::RgbImage;

use crate::datatypes::Interval;
use crate::shapes::Hittables;
use crate::datatypes::Hittable;
use crate::datatypes::HitRecord;
use crate::datatypes::Color3;


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
        let ir = (256.0 * ColorUtil::INTENSITY.clamp(color.x)) as u8;
        let ig = (256.0 * ColorUtil::INTENSITY.clamp(color.y)) as u8;
        let ib = (256.0 * ColorUtil::INTENSITY.clamp(color.z)) as u8;
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


pub struct MathUtil;
impl MathUtil {
    pub fn degrees_to_radians(degrees: f32) -> f32 {
        degrees * PI / 180.0
    }
    pub fn rand() -> f32 {
        rand::random::<f32>()
    }
    pub fn rand_ran(min: f32, max: f32) -> f32 {
        min + (max - min) * MathUtil::rand()
    }
}