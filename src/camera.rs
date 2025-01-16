
use indicatif::ProgressIterator;

use crate::datatypes::Point3;
use crate::datatypes::Vec3;
use crate::shapes::HittableList;
use crate::datatypes::Interval;
use crate::datatypes::Color3;
use crate::datatypes::Ray;
use crate::datatypes::Hittable;
use crate::utils::MathUtil;

pub struct Camera {
    aspect_ratio: f32,  // Aspect ratio
    pixel_samples: u32, // Number of samples per pixel
    pixel_sample_scale: f32, // Scale factor for pixel samples
    im_width: u32,      // Rendered image width
    im_height: u32,     // Rendered image height
    center: Point3,     // Camera center
    px_00_loc: Point3,  // Location of pixel (0, 0)
    px_delta_u: Vec3,   // Pixel offset to right
    px_delta_v: Vec3,   // Pixel offset down

}
impl Camera {
    pub fn new(aspect_ratio: f32, im_width: u32, pixel_samples: u32) -> Self {
        let im_height = u32::max((im_width as f32 / aspect_ratio) as u32, 1);
        let center = Point3::zero();

                // Viewport Dimensions
        let focal_len = 1.0;
        let vp_height = 2.0;
        let vp_width = vp_height * (im_width as f32 / im_height as f32);

                // Viewport Dimensions
        let vp_u = Vec3::new(vp_width, 0.0, 0.0);
        let vp_v = Vec3:: new(0.0, -vp_height, 0.0);

                // Pixel Delta Vectors
        let px_delta_u = &vp_u / im_width as f32;
        let px_delta_v = &vp_v / im_height as f32;

        
        let vp_upper_left = &center - &Vec3::new(0.0, 0.0, focal_len) - &vp_u / 2.0 - &vp_v / 2.0;
        let px_00_loc = &vp_upper_left + &(0.5 * (&px_delta_u + &px_delta_v));

        let pixel_sample_scale = 1.0 / pixel_samples as f32;

        Self {
            aspect_ratio,
            pixel_samples,
            pixel_sample_scale,
            im_width,
            im_height,
            center,
            px_00_loc,
            px_delta_u,
            px_delta_v,
        }
    }
    pub fn ray_color(&self, ray: &Ray, world: &HittableList) -> Color3 {
        let interval = Interval::new(0.0, f32::INFINITY);
        match world.hit(ray, &interval) {
            Some(hr) => {
                0.5 * (hr.normal + Color3::one())
            },
            None => {
                let unit_dir = ray.direction().unit();
                let a = 0.5 * (unit_dir.y + 1.0);
                (1.0 - a) * Color3::one() + (a * Color3::new(0.5, 0.7, 1.0))
            },
        }
    }
    pub fn render(&self, world: &HittableList) -> Vec<Color3> {
        let mut pixels: Vec<Color3> = Vec::new();
        for j in (0..self.im_height).progress() {
            for i in 0..self.im_width {
                let mut pixel = Color3::zero();
                for _sample in 0..self.pixel_samples {
                    let ray = self.get_ray(i, j);
                    pixel += self.ray_color(&ray, world);
                }
                pixels.push(pixel * self.pixel_sample_scale);
            }
            // let ten_millis = time::Duration::from_millis(2);
            // thread::sleep(ten_millis);
        }

        pixels
    }
        // TODO I think there's a better way to do this.
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = &self.px_00_loc
                    + &((i as f32 + offset.x) * &self.px_delta_u)
                    + (j as f32 + offset.y) * &self.px_delta_v;

        let ray_origin = &self.center;
        let ray_dir = &pixel_sample - ray_origin;

        Ray::new(ray_origin.clone(), ray_dir)
    }
    pub fn sample_square() -> Vec3 {
        Vec3::new(MathUtil::rand() - 0.5, MathUtil::rand() - 0.5, 0.0)
    }
    pub fn im_width(&self) -> u32 {
        self.im_width
    }
    pub fn im_height(&self) -> u32 {
        self.im_height
    }
}