use crate::datatypes::Color3;


pub struct ColorUtil;
impl ColorUtil {
    pub fn get_color_str(color: &Color3) -> String {
        let ir = (255.999 * color.x) as i32;
        let ig = (255.999 * color.y) as i32;
        let ib = (255.999 * color.z) as i32;

        format!("{ir} {ig} {ib}\n")
    }
}