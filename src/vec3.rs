use std::{
    fmt,
    ops::{self},
};

#[derive(Copy, Clone, PartialEq, Debug)]
pub struct Vec3(pub f64, pub f64, pub f64);

impl Vec3 {
    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(self) -> f64 {
        self.0 * self.0 + self.1 * self.1 + self.2 * self.2
    }

    pub fn close_to_zero(&self) -> bool {
        let s = 1e-8;
        f64::abs(self.0) < s && f64::abs(self.1) < s && f64::abs(self.2) < s
    }
}

pub fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    *v - 2. * dot(&v, &n) * n.clone()
}

pub fn cross(a: &Vec3, b: &Vec3) -> Vec3 {
    Vec3(
        a.1 * b.2 - a.2 * b.1,
        a.2 * b.0 - a.0 * b.2,
        a.0 * b.1 - a.1 * b.0,
    )
}

pub fn dot(a: &Vec3, b: &Vec3) -> f64 {
    a.0 * b.0 + a.1 * b.1 + a.2 * b.2
}

pub fn unit_vector(vec: &Vec3) -> Vec3 {
    *vec / vec.length()
}

impl fmt::Display for Vec3 {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "Vector:\n 0:{} 1:{} 2:{}", self.0, self.1, self.2)
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.0 += rhs.0;
        self.1 += rhs.1;
        self.2 += rhs.2;
    }
}

impl ops::Sub<Vec3> for Vec3 {
    type Output = Vec3;

    fn sub(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3(self.0 * -1., self.1 * -1., self.2 * -1.)
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Self::Output {
        Vec3(self.0 * f, self.1 * f, self.2 * f)
    }
}

impl ops::Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(rhs.0 * self, rhs.1 * self, rhs.2 * self)
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        self.0 *= f;
        self.1 *= f;
        self.2 *= f;
    }
}

impl ops::Mul<Vec3> for Vec3 {
    type Output = Vec3;

    fn mul(self, rhs: Vec3) -> Self::Output {
        Vec3(self.0 * rhs.0, self.1 * rhs.1, self.2 * rhs.2)
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, f: f64) -> Self::Output {
        Vec3(self.0 * 1. / f, self.1 * 1. / f, self.2 * 1. / f)
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, f: f64) {
        self.0 /= f;
        self.1 /= f;
        self.2 /= f;
    }
}
#[cfg(test)]
mod vec3_tests {
    use super::*;

    fn basic_vec() -> Vec3 {
        Vec3(2., 2., 2.)
    }

    fn basic_result() -> Vec3 {
        Vec3(4., 4., 4.)
    }

    fn basic_result_div() -> Vec3 {
        Vec3(1., 1., 1.)
    }

    #[test]
    fn test_dot() {
        let a = basic_vec();
        let b = basic_vec();
        assert_eq!(dot(&a, &b), 12.);
    }

    #[test]
    fn test_cross() {
        let a = basic_vec();
        let b = basic_vec();
        assert_eq!(cross(&a, &b), Vec3(0., 0., 0.));
    }

    #[test]
    fn test_length() {
        let a = basic_vec();
        assert_eq!(a.length(), 3.4641016151377544)
    }

    #[test]
    fn test_length_squared() {
        let a = basic_vec();
        assert_eq!(a.length_squared(), 12.);
    }

    #[test]
    fn test_add() {
        let a = basic_vec();
        let b = basic_vec();

        assert_eq!(a + b, basic_result())
    }

    #[test]
    fn test_add_assign() {
        let mut a = basic_vec();
        a += basic_vec();
        assert_eq!(a, basic_result());
    }

    #[test]
    fn test_mul() {
        let a = basic_vec();
        assert_eq!(a * 2., basic_result());
        let b = basic_vec();
        assert_eq!(2. * b, basic_result());
    }

    #[test]
    fn test_mul_assign() {
        let mut a = basic_vec();
        a *= 2.;
        assert_eq!(a, basic_result());
    }

    #[test]
    fn test_mul_s1mmetric() {
        let a = basic_vec();
        assert_eq!(2. * a, a * 2.);
    }
    #[test]
    fn test_neg() {
        let a = basic_vec();
        assert_eq!(-a, Vec3(-2., -2., -2.))
    }

    #[test]
    fn test_div() {
        let a = basic_vec();
        assert_eq!(a / 2., basic_result_div());
    }

    #[test]
    fn test_div_assign() {
        let mut a = basic_vec();
        a /= 2.;
        assert_eq!(a, basic_result_div());
    }
}
