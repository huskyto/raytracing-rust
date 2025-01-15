
    
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
        assert_eq!(v1.dot(v2), 32.0);
    }

    #[test]
    fn test_cross() {
        let v1 = Vec3::new(1.0, 2.0, 3.0);
        let v2 = Vec3::new(4.0, 5.0, 6.0);
        let cross = v1.cross(v2);
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
        let neg = -v;
        assert_eq!(neg.x, -1.0);
        assert_eq!(neg.y, -2.0);
        assert_eq!(neg.z, -3.0);
    }

    #[test]
    fn test_mul_f32() {
        let v = Vec3::new(1.0, 2.0, 3.0);
        let prod = v * 2.0;
        assert_eq!(prod.x, 2.0);
        assert_eq!(prod.y, 4.0);
        assert_eq!(prod.z, 6.0);
    }

    #[test]
    fn test_div_f32() {
        let v = Vec3::new(2.0, 4.0, 6.0);
        let div = v / 2.0;
        assert_eq!(div.x, 1.0);
        assert_eq!(div.y, 2.0);
        assert_eq!(div.z, 3.0);
    }
}