use std::fmt;

use crate::vec3::Vec3;

#[derive(Debug, Copy, Clone, PartialEq)]
pub struct Ray {
    pub origin: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(self, t: f64) -> Vec3 {
        self.origin + t * self.dir
    }
}

impl fmt::Display for Ray {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(
            f,
            "Ray Origin:\n x:{} y:{} z:{}\nRay Direction:\n x:{} y:{} z:{} ",
            self.origin.x, self.origin.y, self.origin.z, self.dir.x, self.dir.y, self.dir.z
        )
    }
}
