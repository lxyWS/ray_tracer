use crate::vec3::{Vec3, cross, unit_vector};

pub struct Onb {
    axis: [Vec3; 3],
}

impl Onb {
    pub fn new(n: Vec3) -> Self {
        let axis_2 = unit_vector(n);
        let a = if axis_2.x().abs() > 0.9 {
            Vec3::new(0.0, 1.0, 0.0)
        } else {
            Vec3::new(1.0, 0.0, 0.0)
        };

        let axis_1 = unit_vector(cross(&axis_2, &a));
        let axis_0 = cross(&axis_2, &axis_1);

        Self {
            axis: [axis_0, axis_1, axis_2],
        }
    }

    pub fn u(&self) -> &Vec3 {
        &self.axis[0]
    }

    pub fn v(&self) -> &Vec3 {
        &self.axis[1]
    }

    pub fn w(&self) -> &Vec3 {
        &self.axis[2]
    }

    pub fn transform(&self, v: Vec3) -> Vec3 {
        self.axis[0] * v.x() + self.axis[1] * v.y() + self.axis[2] * v.z()
    }
}
