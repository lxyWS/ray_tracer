use crate::{
    interval::Interval,
    ray::Ray,
    vec3::{Point3, Vec3},
};
use std::fmt::Debug;
use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Aabb {
    pub x: Interval,
    pub y: Interval,
    pub z: Interval,
}

impl Aabb {
    pub const EMPTY: Aabb = Aabb {
        x: Interval::EMPTY,
        y: Interval::EMPTY,
        z: Interval::EMPTY,
    };

    pub const UNIVERSE: Aabb = Aabb {
        x: Interval::UNIVERSE,
        y: Interval::UNIVERSE,
        z: Interval::UNIVERSE,
    };

    pub fn new_empty() -> Self {
        Self {
            x: Interval::new_empty(),
            y: Interval::new_empty(),
            z: Interval::new_empty(),
        }
    }

    pub fn from_intervals(x: Interval, y: Interval, z: Interval) -> Self {
        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn from_points(a: Point3, b: Point3) -> Self {
        let x = if a.x() <= b.x() {
            Interval::new(a.x(), b.x())
        } else {
            Interval::new(b.x(), a.x())
        };
        let y = if a.y() <= b.y() {
            Interval::new(a.y(), b.y())
        } else {
            Interval::new(b.y(), a.y())
        };
        let z = if a.z() <= b.z() {
            Interval::new(a.z(), b.z())
        } else {
            Interval::new(b.z(), a.z())
        };

        let mut aabb = Self { x, y, z };
        aabb.pad_to_minimums();
        aabb
    }

    pub fn from_aabbs(box0: Aabb, box1: Aabb) -> Self {
        Self {
            x: Interval::from_intervals(&box0.x, &box1.x),
            y: Interval::from_intervals(&box0.y, &box1.y),
            z: Interval::from_intervals(&box0.z, &box1.z),
        }
    }

    pub fn axis_interval(&self, n: usize) -> &Interval {
        match n {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &self.x,
        }
    }

    pub fn hit(&self, r: &Ray, mut ray_t: Interval) -> bool {
        let ray_orig = r.origin();
        let ray_dir = r.direction();

        for axis in 0..3 {
            let ax = self.axis_interval(axis);
            let adinv = 1.0 / ray_dir[axis];

            let t0 = (ax.min - ray_orig[axis]) * adinv;
            let t1 = (ax.max - ray_orig[axis]) * adinv;

            let (t_min, t_max) = if t0 < t1 { (t0, t1) } else { (t1, t0) };

            if t_min > ray_t.min {
                ray_t.min = t_min;
            }
            if t_max < ray_t.max {
                ray_t.max = t_max;
            }

            if ray_t.max <= ray_t.min {
                return false;
            }
        }
        true
    }

    pub fn longest_axis(&self) -> usize {
        if self.x.size() > self.y.size() {
            if self.x.size() > self.z.size() {
                return 0;
            } else {
                return 2;
            }
        } else {
            if self.y.size() > self.z.size() {
                return 1;
            } else {
                return 2;
            }
        }
    }

    fn pad_to_minimums(&mut self) {
        let delta = 0.0001;
        if self.x.size() < delta {
            self.x = self.x.expand(delta)
        };
        if self.y.size() < delta {
            self.y = self.y.expand(delta)
        };
        if self.z.size() < delta {
            self.z = self.z.expand(delta)
        };
    }

    pub fn add_offset(&self, offset: Vec3) -> Self {
        Self {
            x: self.x + offset.x(),
            y: self.y + offset.y(),
            z: self.z + offset.z(),
        }
    }
}

impl Add<Vec3> for Aabb {
    type Output = Self;

    fn add(self, offset: Vec3) -> Self::Output {
        self.add_offset(offset)
    }
}

impl Add<Aabb> for Vec3 {
    type Output = Aabb;

    fn add(self, bbox: Aabb) -> Self::Output {
        bbox.add_offset(self)
    }
}

unsafe impl Send for Aabb {}
unsafe impl Sync for Aabb {}
