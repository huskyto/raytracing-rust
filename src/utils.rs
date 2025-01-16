use image::RgbImage;

use crate::{datatypes::{Color3, HitRecord, Hittable}, shapes::Hittables};


pub struct ColorUtil;
impl ColorUtil {
    pub fn get_color_str(color: &Color3) -> String {
        let ir = (255.999 * color.x) as i32;
        let ig = (255.999 * color.y) as i32;
        let ib = (255.999 * color.z) as i32;

        format!("{ir} {ig} {ib}\n")
    }

    pub fn get_pixel(color: &Color3) -> image::Rgb<u8> {
        let ir = (255.999 * color.x) as u8;
        let ig = (255.999 * color.y) as u8;
        let ib = (255.999 * color.z) as u8;
        image::Rgb([ir, ig, ib])
        // image::Rgb([pixel[0] as u8, pixel[1] as u8, pixel[2] as u8])
    }
}

pub struct ImageUtil;
impl ImageUtil {
    pub fn get_rgb_image(pixels: Vec<Color3>, width: u32, height: u32) -> RgbImage {
        let mut image: RgbImage = RgbImage::new(width, height);
        for j in 0..height {
            for i in 0..width {
                // let pixel = &pixels[j * im_width as usize + i];
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
    pub fn hit(hittable: &Hittables, ray: &crate::datatypes::Ray, t_min: f32, t_max: f32) -> Option<HitRecord> {
        match hittable {
            Hittables::Sphere(sphere) => sphere.hit(ray, t_min, t_max),
            Hittables::HittableList(list) => list.hit(ray, t_min, t_max),
        }
    }
}