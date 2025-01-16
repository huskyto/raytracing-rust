
pub mod datatypes;
mod utils;
mod tests;
mod shapes;
mod camera;

use camera::Camera;
use shapes::HittableList;
use shapes::Hittables;
use shapes::Sphere;
use utils::ImageUtil;

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let im_width: u32 = 400;

    let mut world = HittableList::new();
    world.add(Hittables::Sphere(Sphere::new(0.5, 0.0, 0.0, -1.0)));
    world.add(Hittables::Sphere(Sphere::new(100.0, 0.0, -100.5, -1.0)));

    let camera = Camera::new(aspect_ratio, im_width);
    let pixels = camera.render(&world);

    let image = ImageUtil::get_rgb_image(pixels, camera.im_width(), camera.im_height());
    let _ = image.save("output.png");
}
