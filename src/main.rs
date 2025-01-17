
pub mod datatypes;
mod utils;
mod tests;
mod shapes;
mod camera;
mod materials;

use std::time::Instant;

use datatypes::Color3;
use materials::MatDielectric;
use materials::MatMetal;
use materials::Materials;
use materials::MatLambertian;
use shapes::Sphere;
use shapes::Hittables;
use shapes::HittableList;
use camera::Camera;
use utils::MathUtil;
use utils::ImageUtil;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let im_width: u32 = 800;

    let start = Instant::now();

    let mut world = HittableList::new();

    let mat_ground = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.8, 0.8, 0.0)));
    let mat_center = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.1, 0.2, 0.5)));
    // let mat_left = Materials::Metal(MatMetal::new(Color3::new(0.8, 0.8, 0.8), 0.0));
    let mat_left = Materials::Dielectric(MatDielectric::new(1.0 / 1.33));
    // let mat_right = Materials::Metal(MatMetal::new(Color3::new(0.8, 0.6, 0.2), 0.15));
    let mat_right = Materials::Metal(MatMetal::new(Color3::new(0.8, 0.6, 0.2), 1.0));

    for _ in 0..10 {
        let color = Color3::random();
        // let material = Materials::Metal(MatMetal::new(color));
        let material = Materials::DifuseLamb(MatLambertian::new(color));
        let radius = MathUtil::rand_ran(0.25, 1.5);
        let x = MathUtil::rand_ran(-20.0, 20.0);
        let y = MathUtil::rand_ran(5.0, 10.0);
        let z = MathUtil::rand_ran(20.0, -20.0);
        let sphere = Sphere::new(radius, x, y, z, material);
        world.add(Hittables::Sphere(sphere));
    }

    world.add(Hittables::Sphere(Sphere::new(100.0, 0.0, -100.5, -1.0, mat_ground)));
    world.add(Hittables::Sphere(Sphere::new(0.5,  0.0, 0.0, -1.2, mat_center)));
    world.add(Hittables::Sphere(Sphere::new(0.5, -1.0, 0.0, -1.0, mat_left)));
    world.add(Hittables::Sphere(Sphere::new(0.5,  1.0, 0.0, -1.0, mat_right)));

    let camera = Camera::new(aspect_ratio, im_width, 100, 50);
    let pixels = camera.render_par(&world);
    // let pixels = camera.render(&world);

    let elapsed = start.elapsed();
    println!("Run time: {}", elapsed.as_millis());

    let image = ImageUtil::get_rgb_image(pixels, camera.im_width(), camera.im_height());
    let _ = image.save("output.png");
}
