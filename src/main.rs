
mod datatypes;

use std::thread;
use std::time;
use indicatif::ProgressIterator;

fn main() {
    
        // Setup
    let mut content = String::new();
    let im_width = 256;
    let im_height = 256;

        // Render
    content.push_str("P3\n");   // Define ASCII color mode.
    content.push_str(&format!("{im_width} {im_height}\n"));     // Dimensions.
    content.push_str("255\n");  // Set max color
    
    for j in (0..im_height).progress() {
        for i in 0..im_width {
            let r = i as f32 / (im_width - 1) as f32;
            let g = j as f32 / (im_height - 1) as f32;
            let b = 0.0;

            let ir = (255.999 * r) as i32;
            let ig = (255.999 * g) as i32;
            let ib = (255.999 * b) as i32;

            content.push_str(&format!("{ir} {ig} {ib}\n"));
        }
        let ten_millis = time::Duration::from_millis(10);
        thread::sleep(ten_millis);
    }

    println!("{content}");
}
