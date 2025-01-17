
pub mod datatypes;
mod utils;
mod tests;
mod shapes;
mod camera;
mod materials;

use std::time::Instant;

use datatypes::Color3;
use datatypes::Point3;
use datatypes::Vec3;
use materials::MatDielectric;
use materials::MatMetal;
use materials::MaterialFactory;
use materials::Materials;
use materials::MatLambertian;
use shapes::ShapeFactory;
use shapes::Sphere;
use shapes::Hittables;
use shapes::HittableList;
use camera::Camera;
use utils::MathUtil;
use utils::ImageUtil;

fn main() {
    // make_cover();
    dev_scene();
    return;
    let aspect_ratio = 16.0 / 9.0;
    let im_width: u32 = 800;

    let start = Instant::now();

    let mut world = HittableList::new();

    let mat_ground = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.8, 0.8, 0.0)));
    let mat_center = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.1, 0.2, 0.5)));
    // let mat_left = Materials::Metal(MatMetal::new(Color3::new(0.8, 0.8, 0.8), 0.0));
    let mat_left = Materials::Dielectric(MatDielectric::new(1.5));
    let mat_bubble = Materials::Dielectric(MatDielectric::new(1.0 / 1.5));
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
    world.add(Hittables::Sphere(Sphere::new(0.4, -1.0, 0.0, -1.0, mat_bubble)));
    world.add(Hittables::Sphere(Sphere::new(0.5,  1.0, 0.0, -1.0, mat_right)));

    let camera = Camera::new(aspect_ratio, im_width, 100, 50);
    let pixels = camera.render_par(&world);
    // let pixels = camera.render(&world);

    let elapsed = start.elapsed();
    println!("Run time: {}", elapsed.as_millis());

    let image = ImageUtil::get_rgb_image(pixels, camera.im_width(), camera.im_height());
    let _ = image.save("output.png");
}

fn make_cover() {
    let mut world = HittableList::new();

    let ground_material = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.5, 0.5, 0.5)));
    world.add(Hittables::Sphere(Sphere::new(1000.0, 0.0, -1000.0, -1.0, ground_material)));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = MathUtil::rand();
            let center = Point3::new(a as f64 + 0.9 * MathUtil::rand(), 0.2, b as f64 + 0.9 * MathUtil::rand());

            if (&center - &Point3::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere_material: Materials;
                if choose_mat < 0.8 {
                    // Diffuse
                    let albedo = Color3::random() * Color3::random();
                    sphere_material = Materials::DifuseLamb(MatLambertian::new(albedo));
                    world.add(Hittables::Sphere(Sphere::new(0.2, center.x, center.y, center.z, sphere_material)));
                } else if choose_mat < 0.95 {
                    // Metal
                    let albedo = Color3::random_ran(0.5, 1.0);
                    let fuzz = MathUtil::rand_ran(0.0, 0.5);
                    sphere_material = Materials::Metal(MatMetal::new(albedo, fuzz));
                    world.add(Hittables::Sphere(Sphere::new(0.2, center.x, center.y, center.z, sphere_material)));
                } else {
                    // Glass
                    sphere_material = Materials::Dielectric(MatDielectric::new(1.5));
                    world.add(Hittables::Sphere(Sphere::new(0.2, center.x, center.y, center.z, sphere_material)));
                }
            }
        }
    }

    let material1 = Materials::Dielectric(MatDielectric::new(1.5));
    world.add(Hittables::Sphere(Sphere::new(1.0, 0.0, 1.0, 0.0, material1)));

    let material2 = Materials::DifuseLamb(MatLambertian::new(Color3::new(0.4, 0.2, 0.1)));
    world.add(Hittables::Sphere(Sphere::new(1.0, -4.0, 1.0, 0.0, material2)));

    let material3 = Materials::Metal(MatMetal::new(Color3::new(0.7, 0.6, 0.5), 0.0));
    world.add(Hittables::Sphere(Sphere::new(1.0, 4.0, 1.0, 0.0, material3)));

    let camera = Camera::new(16.0 / 9.0, 1200, 500, 50, 20.0, Point3::new(13.0, 2.0, 3.0), Point3::zero(), Point3::y_u(), 0.6, 10.0);

    let pixels = camera.render_par(&world);

    let image = ImageUtil::get_rgb_image(pixels, camera.im_width(), camera.im_height());
    let _ = image.save("out-cover.png");
}

fn dev_scene() {
    let aspect_ratio = 16.0 / 9.0;
    let im_width: u32 = 400;

    let start = Instant::now();

    let mut world = HittableList::new();

    let mat_ground = MaterialFactory::make_lambertian(Color3::new(0.7, 0.7, 0.2));
    let mat_center = MaterialFactory::make_lambertian(Color3::new(0.1, 0.2, 0.5));
    let mat_left = MaterialFactory::make_dielectric(1.5);
    let mat_bubble = MaterialFactory::make_dielectric(1.0 / 1.5);
    let mat_right = MaterialFactory::make_metal(Color3::new(0.8, 0.8, 0.8), 0.1);
    let mat_front = MaterialFactory::make_emitter(Color3::one(), 20.0);

    for _ in 0..10 {
        let color = Color3::random();
        let material = MaterialFactory::make_lambertian(color);
        let radius = MathUtil::rand_ran(0.25, 1.5);
        let x = MathUtil::rand_ran(-20.0, 20.0);
        let y = MathUtil::rand_ran(5.0, 10.0);
        let z = MathUtil::rand_ran(20.0, -20.0);
        let sphere = ShapeFactory::make_sphere(radius, x, y, z, material);
        world.add(sphere);
    }

    world.add(ShapeFactory::make_sphere(100.0, 0.0, -100.5, -1.0, mat_ground));
    world.add(ShapeFactory::make_sphere(0.5, 0.0, 0.0, -1.2, mat_center));
    world.add(ShapeFactory::make_sphere(0.5, -1.0, 0.0, -1.0, mat_left));
    world.add(ShapeFactory::make_sphere(0.4, -1.0, 0.0, -1.0, mat_bubble));
    world.add(ShapeFactory::make_sphere(0.5, 1.0, 0.0, -1.0, mat_right));
    world.add(ShapeFactory::make_sphere(0.25, -0.25, 1.0, -0.5, mat_front));

    let camera = Camera::new(aspect_ratio, im_width, 10000, 50,
            90.0, Point3::zero(), -&Point3::z_u(), Vec3::y_u(),
            // 50.0,
            // Point3::new(-2.0, 2.0, 1.0),
            // Point3::new(0.0, 0.0, -1.0),
            // Vec3::new(0.0, 1.0, 0.0),
            // 2.0, 3.4);
            2.0, 1.0);
    let pixels = camera.render_par(&world);

    let elapsed = start.elapsed();
    println!("Run time: {}", elapsed.as_millis());

    let image = ImageUtil::get_rgb_image(pixels, camera.im_width(), camera.im_height());
    let _ = image.save("out-dev.png");
}