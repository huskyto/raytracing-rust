
mod datatypes;
mod utils;
mod tests;
mod shapes;

use std::thread;
use std::time;
use datatypes::Color3;
use datatypes::Point3;
use datatypes::Ray;
use datatypes::Vec3;
use indicatif::ProgressIterator;
use shapes::Sphere;
use utils::ColorUtil;

fn main() {
    
        // Setup
    let mut content = String::new();
    let aspect_ratio = 16.0 / 9.0;
    let im_width = 400;
    let im_height = i32::max((im_width as f32 / aspect_ratio) as i32, 1);

        // Camera
    let focal_len = 1.0;
    let vp_height = 2.0;
    let vp_width = vp_height * (im_width as f32 / im_height as f32);
    let camera_center = Point3::zero();

    let vp_u = Vec3::new(vp_width, 0.0, 0.0);
    let vp_v = Vec3:: new(0.0, -vp_height, 0.0);

    let px_delta_u = &vp_u / im_width as f32;
    let px_delta_v = &vp_v / im_height as f32;

    let vp_upper_left = &camera_center - &Vec3::new(0.0, 0.0, focal_len) - &vp_u / 2.0 - &vp_v / 2.0;
    let px_00_loc = &vp_upper_left + &(0.5 * (&px_delta_u + &px_delta_v));

        // Render
    content.push_str("P3\n");   // Define ASCII color mode.
    content.push_str(&format!("{im_width} {im_height}\n"));     // Dimensions.
    content.push_str("255\n");  // Set max color
    
    for j in (0..im_height).progress() {
        for i in 0..im_width {
            let px_center= &px_00_loc + &(i as f32 * &px_delta_u) + (j as f32 * &px_delta_v);
            let ray_dir = &px_center - &camera_center;
            let ray = Ray::new(camera_center.clone(), ray_dir);

            let pixel = ray_color(&ray);
            content.push_str(&ColorUtil::get_color_str(&pixel));
        }
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }

    println!("{content}");
}

fn ray_color(ray: &Ray) -> Color3 {
    if hit_sphere(&Sphere::new(0.5, 0.0, 0.0, -1.0), ray) {
        Color3::new(1.0, 0.0, 0.0)
    }
    else {
        let unit_dir = ray.direction().unit();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color3::one() + (a * Color3::new(0.5, 0.7, 1.0))
    }
}

fn hit_sphere(sphere: &Sphere, ray: &Ray) -> bool {
    let oc = &sphere.center - ray.origin();
    let a = ray.direction().dot(ray.direction());
    let b = -2.0 * ray.direction().dot(&oc);
    let c = oc.dot(&oc) - sphere.radius * sphere.radius;
    let discriminant = b * b - 4.0 * a * c;
    discriminant > 0.0
}