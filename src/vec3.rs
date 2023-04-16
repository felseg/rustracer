use std::ops::{self};

#[derive(Copy, Clone, PartialEq, Debug)]
struct Vec3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vec3 {
    pub fn length(self) -> f64 {
        f64::sqrt(self.length_squared())
    }
    pub fn length_squared(self) -> f64 {
        self.x * self.x + self.y * self.y + self.z * self.z
    }
}

impl ops::Add<Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, rhs: Vec3) -> Self::Output {
        Vec3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl ops::AddAssign<Vec3> for Vec3 {
    fn add_assign(&mut self, rhs: Vec3) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl ops::Neg for Vec3 {
    type Output = Vec3;

    fn neg(self) -> Self::Output {
        Vec3 {
            x: self.x * -1.,
            y: self.y * -1.,
            z: self.z * -1.,
        }
    }
}

impl ops::Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, f: f64) -> Self::Output {
        Vec3 {
            x: self.x * f,
            y: self.y * f,
            z: self.z * f,
        }
    }
}

impl ops::MulAssign<f64> for Vec3 {
    fn mul_assign(&mut self, f: f64) {
        self.x *= f;
        self.y *= f;
        self.z *= f;
    }
}

impl ops::Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, f: f64) -> Self::Output {
        Vec3 {
            x: self.x * 1. / f,
            y: self.y * 1. / f,
            z: self.z * 1. / f,
        }
    }
}

impl ops::DivAssign<f64> for Vec3 {
    fn div_assign(&mut self, f: f64) {
        self.x /= f;
        self.y /= f;
        self.z /= f;
    }
}

#[cfg(test)]
mod vec3_tests {
    use super::*;

    fn basic_vec() -> Vec3 {
        Vec3 {
            x: 2.,
            y: 2.,
            z: 2.,
        }
    }

    fn basic_result() -> Vec3 {
        Vec3 {
            x: 4.,
            y: 4.,
            z: 4.,
        }
    }

    fn basic_result_div() -> Vec3 {
        Vec3 {
            x: 1.,
            y: 1.,
            z: 1.,
        }
    }

    #[test]
    fn test_length() {
        let a = basic_result();
        assert_eq!(a.length(), 3.46410161514)
    }

    #[test]
    fn test_length_squared() {
        let a = basic_result();
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
    }

    #[test]
    fn test_mul_assign() {
        let mut a = basic_vec();
        a *= 2.;
        assert_eq!(a, basic_result());
    }

    #[test]
    fn test_neg() {
        let a = basic_vec();
        assert_eq!(
            -a,
            Vec3 {
                x: -2.,
                y: -2.,
                z: -2.
            }
        )
    }

    #[test]
    fn test_div() {
        let a = basic_vec();
        let b = basic_vec();
        assert_eq!(a / 2., basic_result_div())
    }

    #[test]
    fn test_div_assign() {
        let mut a = basic_vec();
        a /= 2.;
        assert_eq!(a, basic_result_div());
    }
}
