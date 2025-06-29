use std::fmt::Debug;
use std::ops::Neg;

use crate::aabb::Aabb;
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec3::{Point3, Vec3, dot};

/// 存储射线与物体的交点信息
#[derive(Debug, Clone, Default)]
pub struct HitRecord {
    pub p: Point3,                // 交点位置
    pub normal: Vec3,             // 交点法向量
    pub mat: Option<MaterialPtr>, // 碰撞点材质
    pub t: f64,                   // 射线参数t
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
