use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::material::MaterialPtr;
use crate::onb::Onb;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, PI, random_double};
use crate::vec3::{Point3, Vec3, dot};

/// 表示三维空间中的球体
#[derive(Debug)]
pub struct Sphere {
    center: Ray, // 球心坐标
    radius: f64, // 半径（确保非负）
    mat: MaterialPtr,
    bbox: Aabb, // bounding box
}

impl Sphere {
    /// 创建一个新的球体
    ///
    /// # 参数
    /// - `center`: 球心坐标
    /// - `radius`: 半径（负值会被截断为0）
    pub fn new(static_center: Point3, radius: f64, mat: MaterialPtr) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        Self {
            center: Ray::with_origin_dir(static_center, Vec3::new(0.0, 0.0, 0.0)),
            radius: radius.max(0.0), // 确保半径非负
            mat: mat,
            bbox: Aabb::from_points(static_center - rvec, static_center + rvec),
        }
    }

    pub fn new_moving(center1: Point3, center2: Point3, radius: f64, mat: MaterialPtr) -> Self {
        let rvec = Vec3::new(radius, radius, radius);
        let box1 = Aabb::from_points(center1 - rvec, center1 + rvec);
        let box2 = Aabb::from_points(center2 - rvec, center2 + rvec);
        Self {
            center: Ray::with_origin_dir(center1, center2 - center1),
            radius: radius.max(0.0),
            mat: mat,
            bbox: Aabb::from_aabbs(box1, box2),
        }
    }

    pub fn get_sphere_uv(p: &Point3) -> (f64, f64) {
        let theta = (-p.y()).acos();
        let phi = (-p.z()).atan2(p.x()) + PI;

        let u = phi / (2.0 * PI);
        let v = theta / PI;

        (u, v)
    }

    pub fn random_to_sphere(radius: f64, distance_squared: f64) -> Vec3 {
        let r1 = random_double();
        let r2 = random_double();
        let z = 1.0 + r2 * ((1.0 - radius.powi(2) / distance_squared).sqrt() - 1.0);

        let phi = 2.0 * PI * r1;
        let x = phi.cos() * (1.0 - z.powi(2)).sqrt();
        let y = phi.sin() * (1.0 - z.powi(2)).sqrt();

        Vec3::new(x, y, z)
    }
}

/// 实现Hittable trait，使球体可被射线击中
impl Hittable for Sphere {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let current_center = self.center.at(r.time());
        let oc = current_center - *r.origin();
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
        let outward_normal = (rec.p - current_center) / self.radius;
        rec.set_face_normal(r, outward_normal);
        let (u, v) = Sphere::get_sphere_uv(&outward_normal);
        rec.u = u;
        rec.v = v;
        rec.mat = Some(self.mat.clone());

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &Point3, direction: &Vec3) -> f64 {
        let mut rec = HitRecord::default();
        let ray = Ray::with_origin_dir(*origin, *direction);

        if !self.hit(&ray, Interval::new(0.001, INFINITY), &mut rec) {
            return 0.0;
        }

        let center = self.center.at(0.0);
        let dist_squared = (center - *origin).length_squared();
        let cos_theta_max = (1.0 - self.radius.powi(2) / dist_squared).sqrt();
        let solid_angle = 2.0 * PI * (1.0 - cos_theta_max);

        1.0 / solid_angle
    }

    fn random(&self, origin: &Point3) -> Vec3 {
        let center = self.center.at(0.0);
        let direction = center - *origin;
        let distance_squared = direction.length_squared();

        let uvw = Onb::new(direction);

        uvw.transform(Sphere::random_to_sphere(self.radius, distance_squared))
    }
}

unsafe impl Sync for Sphere {}
