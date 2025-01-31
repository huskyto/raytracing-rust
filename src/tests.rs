#[cfg(test)]
mod vec3_tests {
    use crate::datatypes::*;

    #[test]
    fn test_zero() {
        let v = Vec3::zero();
        assert_eq!(v.x, 0.0);
        assert_eq!(v.y, 0.0);
        assert_eq!(v.z, 0.0);
    }

    #[test]
    fn test_new() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        assert_eq!(v.x, 1.0);
        assert_eq!(v.y, 2.0);
        assert_eq!(v.z, 3.0);
    }

    #[test]
    fn test_len_sqr() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.len_sqr(), 9.0);
    }

    #[test]
    fn test_len() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        assert_eq!(v.len(), 3.0);
    }

    #[test]
    fn test_dot() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        assert_eq!(v1.dot(&v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let cross = v1.cross(&v2);
        assert_eq!(cross.x, -3.0);
        assert_eq!(cross.y, 6.0);
        assert_eq!(cross.z, -3.0);
    }

    #[test]
    fn test_unit() {
        let v = Vec3::new(1.0, 2.0, 2.0);
        let unit = v.unit();
        assert!((unit.x - 1.0 / 3.0).abs() < 1e-6);
        assert!((unit.y - 2.0 / 3.0).abs() < 1e-6);
        assert!((unit.z - 2.0 / 3.0).abs() < 1e-6);
    }

    #[test]
    fn test_add() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let sum = v1 + v2;
        assert_eq!(sum.x, 5.0);
        assert_eq!(sum.y, 7.0);
        assert_eq!(sum.z, 9.0);
    }

    #[test]
    fn test_sub() {
        let v1 = Vec3::new(4.0, 5.0, 6.0);
        let v2 = Vec3::new(1.0, 2.0, 3.0);
        let diff = v1 - v2;
        assert_eq!(diff.x, 3.0);
        assert_eq!(diff.y, 3.0);
        assert_eq!(diff.z, 3.0);
    }

    #[test]
    fn test_neg() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let neg = -&v;
        assert_eq!(neg.x, -1.0);
        assert_eq!(neg.y, -2.0);
        assert_eq!(neg.z, -3.0);
    }

    #[test]
    fn test_mul_f64() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let prod = v * 2.0;
        assert_eq!(prod.x, 2.0);
        assert_eq!(prod.y, 4.0);
        assert_eq!(prod.z, 6.0);
    }

    #[test]
    fn test_div_f64() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let div = v / 2.0;
        assert_eq!(div.x, 1.0);
        assert_eq!(div.y, 2.0);
        assert_eq!(div.z, 3.0);
    }
}

#[cfg(test)]
mod color_util_tests {
    use crate::datatypes::*;
    use crate::utils::ColorUtil;

    #[test]
    fn test_get_color_str() {
        let color = Color3::new(0.5, 0.7, 0.9);
        let color_str = ColorUtil::get_color_str(&color);
        let values: Vec<&str> = color_str.split_whitespace().collect();
        assert_eq!(values.len(), 3);
        assert!((values[0].parse::<u32>().unwrap() as f64 - 127.0).abs() < 2.0);
        assert!((values[1].parse::<u32>().unwrap() as f64 - 179.0).abs() < 2.0);
        assert!((values[2].parse::<u32>().unwrap() as f64 - 230.0).abs() < 2.0);
    }

    #[test]
    fn test_get_pixel() {
        let color = Color3::new(0.5, 0.7, 0.9);
        let pixel = ColorUtil::get_pixel(&color);
        assert!((pixel[0] as f64 - 181.0).abs() < 2.0);
        assert!((pixel[1] as f64 - 214.0).abs() < 2.0);
        assert!((pixel[2] as f64 - 242.0).abs() < 2.0);
    }
}

#[cfg(test)]
mod ray_tests {
    use crate::datatypes::*;

    #[test]
    fn test_ray_at() {
        let origin = Point3::new(1.0, 2.0, 3.0);
        let direction = Vec3::new(4.0, 5.0, 6.0);
        let ray = Ray::new(origin, direction);
        let point = ray.at(2.0);
        assert_eq!(point.x, 9.0);
        assert_eq!(point.y, 12.0);
        assert_eq!(point.z, 15.0);
    }
}

#[cfg(test)]
mod hit_record_tests {
    use crate::{datatypes::*, materials::{MatLambertian, Materials}};

    #[test]
    fn test_set_face_normal() {
        let p = Point3::new(1.0, 2.0, 3.0);
        let normal = Vec3::new(4.0, 5.0, 6.0);
        let t = 7.0;
        let mut hit_record = HitRecord::new(p, normal, t, Materials::DifuseLamb(MatLambertian::GRAY));
        let ray = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::new(1.0, 0.0, 0.0));
        let outward_normal = Vec3::new(-1.0, 0.0, 0.0);
        hit_record.set_face_normal(&ray, &outward_normal);
        assert!(hit_record.is_front_face);
        assert_eq!(hit_record.normal, outward_normal);
    }
}

#[cfg(test)]
mod interval_tests {
    use crate::datatypes::*;

    #[test]
    fn test_interval_len() {
        let interval = Interval::new(1.0, 2.0);
        assert_eq!(interval.len(), 1.0);
    }

    #[test]
    fn test_interval_contains() {
        let interval = Interval::new(1.0, 2.0);
        assert!(interval.contains(1.5));
        assert!(!interval.contains(2.5));
    }

    #[test]
    fn test_interval_surrounds() {
        let interval = Interval::new(1.0, 2.0);
        assert!(interval.surrounds(1.5));
        assert!(!interval.surrounds(1.0));
    }
}

#[cfg(test)]
mod math_util_tests {
    use crate::utils::MathUtil;

    #[test]
    fn test_degrees_to_radians() {
        let degrees = 180.0;
        let radians = MathUtil::degrees_to_radians(degrees);
        assert!((radians - std::f64::consts::PI).abs() < 1e-6);
    }
}

#[cfg(test)]
mod camera_tests {
    use crate::camera::Camera;
    use crate::datatypes::{Color3, Point3, Ray, Vec3};
    use crate::materials::{MatLambertian, Materials};
    use crate::shapes::{HittableList, Hittables, Sphere};

    #[test]
    fn test_camera_ray_color_empty_world() {
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let world = HittableList::new();
        let color = Camera::ray_color(&ray, 10, &world);
        assert_eq!(color, Color3::new(0.75, 0.85, 1.0)); // Background color
    }

    #[test]
    fn test_camera_ray_color_with_sphere() {
        let ray = Ray::new(Point3::zero(), Vec3::new(0.0, 0.0, -1.0));
        let mut world = HittableList::new();
        world.add(Hittables::Sphere(Sphere::new(0.5, 0.0, 0.0, -1.0, Materials::DifuseLamb(MatLambertian::GRAY))));
        let color = Camera::ray_color(&ray, 10, &world);
        assert!((color.x - 0.42).abs() < 0.02);
        assert!((color.y - 0.45).abs() < 0.02);
        assert!((color.z - 0.5).abs() < 0.02);
    }
}