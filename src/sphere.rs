use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::ray::Ray;
use crate::vec3::{Point3, dot};

/// 表示三维空间中的球体
pub struct Sphere {
    center: Point3, // 球心坐标
    radius: f64,    // 半径（确保非负）
    mat: MaterialPtr,
}

impl Sphere {
    /// 创建一个新的球体
    ///
    /// # 参数
    /// - `center`: 球心坐标
    /// - `radius`: 半径（负值会被截断为0）
    pub fn new(center: Point3, radius: f64, mat: MaterialPtr) -> Self {
        Self {
            center,
            radius: radius.max(0.0), // 确保半径非负
            mat: mat,
        }
    }
}

/// 实现Hittable trait，使球体可被射线击中
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let oc = self.center - *r.origin();
        let a = r.direction().length_squared();
        let h = dot(r.direction(), &oc);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = h * h - a * c;

        // 判别式小于0表示无交点
        if discriminant < 0.0 {
            return false;
        }

        let sqrtd = discriminant.sqrt();

        // Find the nearest root that lies in the acceptable range.
        let mut root = (h - sqrtd) / a;

        // 检查第一个根是否在有效范围内
        if !ray_t.surrounds(root) {
            // 第一个根无效，尝试第二个根
            root = (h + sqrtd) / a;
            if !ray_t.surrounds(root) {
                // 两个根都无效
                return false;
            }
        }

        // 记录交点信息
        rec.t = root;
        rec.p = r.at(rec.t);
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        rec.mat = Some(self.mat.clone());

        // 计算法向量（从球心指向交点，已归一化）
        let outward_normal = (rec.p - self.center) / self.radius;
        rec.normal = outward_normal;

        true
    }
}
