
pub mod datatypes;
mod utils;
mod tests;
mod shapes;
mod camera;
mod materials;

use camera::Camera;
use datatypes::Color3;
use materials::MatLambertian;
use materials::MatMetal;
use materials::Materials;
use shapes::HittableList;
use shapes::Hittables;
use shapes::Sphere;
use utils::ImageUtil;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let im_width: u32 = 400;

    let mut world = HittableList::new();

    let mat_ground = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.8, 0.8, 0.0)));
    let mat_center = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.1, 0.2, 0.5)));
    let mat_left = Materials::Metal(MatMetal::new(Color3::new(0.8, 0.8, 0.8)));
    let mat_right = Materials::Metal(MatMetal::new(Color3::new(0.8, 0.6, 0.2)));

    world.add(Hittables::Sphere(Sphere::new(100.0, 0.0, -100.5, -1.0, mat_ground)));
    world.add(Hittables::Sphere(Sphere::new(0.5,  0.0, 0.0, -1.2, mat_center)));
    world.add(Hittables::Sphere(Sphere::new(0.5, -1.0, 0.0, -1.0, mat_left)));
    world.add(Hittables::Sphere(Sphere::new(0.5,  1.0, 0.0, -1.0, mat_right)));

    let camera = Camera::new(aspect_ratio, im_width, 100, 50);
    let pixels = camera.render(&world);

    let image = ImageUtil::get_rgb_image(pixels, camera.im_width(), camera.im_height());
    let _ = image.save("output.png");
}
