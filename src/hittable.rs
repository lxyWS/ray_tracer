use std::fmt::Debug;
use std::ops::Neg;
use std::sync::Arc;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::rtweekend::INFINITY;
use crate::vec3::{Point3, Vec3, dot};

/// 存储射线与物体的交点信息
#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,                // 交点位置
    pub normal: Vec3,             // 交点法向量
    pub mat: Option<MaterialPtr>, // 碰撞点材质
    pub t: f64,                   // 射线参数t
    pub u: f64,                   // 纹理坐标u
    pub v: f64,
    pub front_face: bool,
}

impl HitRecord {
    pub fn set_face_normal(&mut self, r: &Ray, outward_normal: Vec3) {
        // 法向量向外为true，向内为false
        self.front_face = dot(r.direction(), &outward_normal) < 0.0;
        self.normal = if self.front_face {
            outward_normal
        } else {
            outward_normal.neg()
        };
    }
}

/// 可被射线击中的物体的trait
pub trait Hittable {
    /// 判断射线是否击中物体
    ///
    /// # 参数
    /// - `r`: 射线
    /// - `ray_tmin`: 射线参数t的最小值
    /// - `ray_tmax`: 射线参数t的最大值
    /// - `rec`: 存储交点信息的记录
    ///
    /// # 返回值
    /// - 若击中返回true，否则返回false
    // fn hit(&self, r: &Ray, ray_tmin: f64, ray_tmax: f64, rec: &mut HitRecord) -> bool;
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool;
    fn bounding_box(&self) -> Aabb;
}

// 平移变换的物体
pub struct Translate {
    object: Arc<dyn Hittable + Send + Sync>,
    offset: Vec3,
    bbox: Aabb,
}

impl Translate {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, offset: Vec3) -> Self {
        let bbox = object.bounding_box() + offset;
        Self {
            object,
            offset,
            bbox,
        }
    }
}

impl Hittable for Translate {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let offset_r =
            Ray::with_origin_dir_time(*r.origin() - self.offset, *r.direction(), r.time());
        if !self.object.hit(&offset_r, ray_t, rec) {
            return false;
        }

        rec.p += self.offset;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub struct RotateY {
    object: Arc<dyn Hittable + Send + Sync>,
    sin_theta: f64,
    cos_theta: f64,
    bbox: Aabb,
}

impl RotateY {
    pub fn new(object: Arc<dyn Hittable + Send + Sync>, angle: f64) -> Self {
        let radians = angle.to_radians();
        let sin_theta = radians.sin();
        let cos_theta = radians.cos();
        let bbox = object.bounding_box();

        let mut min = Point3::new(INFINITY, INFINITY, INFINITY);
        let mut max = Point3::new(-INFINITY, -INFINITY, -INFINITY);

        for i in 0..2 {
            for j in 0..2 {
                for k in 0..2 {
                    let x = if i == 0 { bbox.x.min } else { bbox.x.max };
                    let y = if j == 0 { bbox.y.min } else { bbox.y.max };
                    let z = if k == 0 { bbox.z.min } else { bbox.z.max };

                    let newx = cos_theta * x + sin_theta * z;
                    let newz = -sin_theta * x + cos_theta * z;

                    let tester = Vec3::new(newx, y, newz);

                    for c in 0..3 {
                        min[c] = min[c].min(tester[c]);
                        max[c] = max[c].max(tester[c]);
                    }
                }
            }
        }

        let bbox = Aabb::from_points(min, max);

        Self {
            object,
            sin_theta,
            cos_theta,
            bbox,
        }
    }
}

impl Hittable for RotateY {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let origin = Point3::new(
            self.cos_theta * r.origin().x() - self.sin_theta * r.origin().z(),
            r.origin().y(),
            self.sin_theta * r.origin().x() + self.cos_theta * r.origin().z(),
        );

        let direction = Vec3::new(
            self.cos_theta * r.direction().x() - self.sin_theta * r.direction().z(),
            r.direction().y(),
            self.sin_theta * r.direction().x() + self.cos_theta * r.direction().z(),
        );

        let rotated_r = Ray::with_origin_dir_time(origin, direction, r.time());

        if !self.object.hit(&rotated_r, ray_t, rec) {
            return false;
        }

        let p = Point3::new(
            self.cos_theta * rec.p.x() + self.sin_theta * rec.p.z(),
            rec.p.y(),
            -self.sin_theta * rec.p.x() + self.cos_theta * rec.p.z(),
        );

        let normal = Vec3::new(
            self.cos_theta * rec.normal.x() + self.sin_theta * rec.normal.z(),
            rec.normal.y(),
            -self.sin_theta * rec.normal.x() + self.cos_theta * rec.normal.z(),
        );

        rec.p = p;
        rec.normal = normal;
        // rec.set_face_normal(r, normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
