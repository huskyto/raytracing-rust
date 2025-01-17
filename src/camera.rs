
use indicatif::ProgressBar;
use indicatif::ProgressIterator;
use rayon::prelude::*;

use crate::datatypes::Ray;
use crate::datatypes::Vec3;
use crate::datatypes::Point3;
use crate::datatypes::Color3;
use crate::datatypes::Interval;
use crate::datatypes::Hittable;
use crate::shapes::HittableList;
use crate::utils::MatUtil;
use crate::utils::MathUtil;

pub struct Camera {
    aspect_ratio: f64,  // Aspect ratio
    pixel_samples: u32, // Number of samples per pixel
    pixel_sample_scale: f64, // Scale factor for pixel samples
    max_bounces: u32,   // Maximum number of bounces
    im_width: u32,      // Rendered image width
    im_height: u32,     // Rendered image height
    center: Point3,     // Camera center
    px_00_loc: Point3,  // Location of pixel (0, 0)
    px_delta_u: Vec3,   // Pixel offset to right
    px_delta_v: Vec3,   // Pixel offset down
    vfov: f64,          // Vertical field of view
    u: Vec3,            // Camera basis vectors
    v: Vec3,
    w: Vec3,
    defocus_angle: f64, // Defocus disk angle
    defocus_dsk_u: Vec3,// Defocus disk vectors
    defocus_dsk_v: Vec3,
}
impl Camera {
    // pub fn new(aspect_ratio: f64, im_width: u32, pixel_samples: u32, max_bounces: u32, vfov: f64) -> Self {
    pub fn new(aspect_ratio: f64, im_width: u32, pixel_samples: u32, max_bounces: u32,
                    vfov: f64, lookfrom: Point3, lookat: Point3, vup: Vec3, defocus_angle: f64, focus_dist: f64) -> Self {
        let im_height = u32::max((im_width as f64 / aspect_ratio) as u32, 1);
        // let center = Point3::zero();
        let center = lookfrom.clone();      // TODO maybe remove assign

                // Viewport Dimensions
        // let focal_len = 1.0;
        // let focal_len = (&lookfrom - &lookat).len();
        let theta = MathUtil::degrees_to_radians(vfov);
        let h = f64::tan(theta / 2.0);
        // let vp_height = 2.0 * h * focal_len;
        let vp_height = 2.0 * h * focus_dist;
        // let vp_height = 2.0;
        let vp_width = vp_height * (im_width as f64 / im_height as f64);

        let w = (lookfrom - lookat).unit();
        let u = vup.cross(&w).unit();
        let v = w.cross(&u);

                // Viewport Dimensions
        // let vp_u = Vec3::new(vp_width, 0.0, 0.0);
        // let vp_v = Vec3:: new(0.0, -vp_height, 0.0);
        let vp_u = vp_width * &u;
        let vp_v = vp_height * -&v;

                // Pixel Delta Vectors
        let px_delta_u = &vp_u / im_width as f64;
        let px_delta_v = &vp_v / im_height as f64;

        
        // let vp_upper_left = &center - &Vec3::new(0.0, 0.0, focal_len) - &vp_u / 2.0 - &vp_v / 2.0;
        // let vp_upper_left = &center - &(focal_len * &w) - vp_u / 2.0 - vp_v / 2.0;
        let vp_upper_left = &center - &(focus_dist * &w) - vp_u / 2.0 - vp_v / 2.0;
        let px_00_loc = &vp_upper_left + &(0.5 * (&px_delta_u + &px_delta_v));

                // Calculate defocus disk vectors
        let defocus_radius = focus_dist * f64::tan(MathUtil::degrees_to_radians(defocus_angle / 2.0));
        let defocus_dsk_u = defocus_radius * &u;
        let defocus_dsk_v = defocus_radius * &v;

        let pixel_sample_scale = 1.0 / pixel_samples as f64;

        Self {
            aspect_ratio,
            pixel_samples,
            pixel_sample_scale,
            max_bounces,
            im_width,
            im_height,
            center,
            px_00_loc,
            px_delta_u,
            px_delta_v,
            vfov,
            u,
            v,
            w,
            defocus_angle,
            defocus_dsk_u,
            defocus_dsk_v
        }
    }
    pub fn ray_color(&self, ray: &Ray, bounces: u32, world: &HittableList) -> Color3 {
        if bounces == 0 {
            return Color3::zero();
        }

        let interval = Interval::new(0.001, f64::INFINITY);
        match world.hit(ray, &interval) {
            Some(hr) => {
                match MatUtil::scatter(&hr.material, ray, &hr) {
                    Some((att, sc_ray)) => {
                        att * self.ray_color(&sc_ray, bounces - 1, world)
                    },
                    None => Color3::zero(),
                }
            },
            None => {
                    // Sky box
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
                    pixel += self.ray_color(&ray, self.max_bounces, world);
                }
                pixels.push(pixel * self.pixel_sample_scale);
            }
            // let ten_millis = time::Duration::from_millis(2);
            // thread::sleep(ten_millis);
        }

        pixels
    }
    pub fn render_par(&self, world: &HittableList) -> Vec<Color3> {
        let mut pixels: Vec<Color3> = Vec::new();
        let mut points: Vec<(u32, u32)> = Vec::new();

        for j in 0..self.im_height {
            for i in 0..self.im_width {
                points.push((i, j));
            }
        }

        let progress_bar = ProgressBar::new(points.len() as u64);
        points.par_iter().map(|(i, j)| {
            let mut pixel = Color3::zero();
            for _sample in 0..self.pixel_samples {
                let ray = self.get_ray(*i, *j);
                pixel += self.ray_color(&ray, self.max_bounces, world);
            }
            progress_bar.inc(1);
            pixel * self.pixel_sample_scale
        }).collect_into_vec(&mut pixels);

        pixels
    }
    pub fn render_par_lar(&self, world: &HittableList) -> Vec<Color3> {
        let mut is: Vec<u32> = Vec::new();

        for j in 0..self.im_height {
            is.push(j);
        }

        let progress_bar = ProgressBar::new(is.len() as u64);
        let pixels = is.par_iter().map(|j| {
            let mut local_v = Vec::new();
            for i in 0..self.im_width {
                let mut pixel = Color3::zero();
                for _sample in 0..self.pixel_samples {
                    let ray = self.get_ray(i, *j);
                    pixel += self.ray_color(&ray, self.max_bounces, world);
                }
                local_v.push(pixel * self.pixel_sample_scale)
            }
            progress_bar.inc(1);
            local_v
        }).flatten().collect::<Vec<Color3>>();

        pixels
    }
        // TODO I think there's a better way to do this.
    pub fn get_ray(&self, i: u32, j: u32) -> Ray {
        let offset = Self::sample_square();
        let pixel_sample = &self.px_00_loc
                    + &((i as f64 + offset.x) * &self.px_delta_u)
                    + (j as f64 + offset.y) * &self.px_delta_v;

        // let ray_origin = &self.center;
        let ray_origin = if self.defocus_angle <= 0.0 { &self.center } else { &self.defocus_disk_sample() };
        let ray_dir = &pixel_sample - &ray_origin;

        Ray::new(ray_origin.clone(), ray_dir)
    }
    fn sample_square() -> Vec3 {
        Vec3::new(MathUtil::rand() - 0.5, MathUtil::rand() - 0.5, 0.0)
    }
    fn defocus_disk_sample(&self) -> Point3 {
        let p = Vec3::random_in_unit_disk();
        &self.center + &(p.x * &self.defocus_dsk_u) + (p.y * &self.defocus_dsk_v)
    }
    pub fn im_width(&self) -> u32 {
        self.im_width
    }
    pub fn im_height(&self) -> u32 {
        self.im_height
    }
}