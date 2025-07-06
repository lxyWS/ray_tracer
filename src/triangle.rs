// use crate::{
//     aabb::Aabb,
//     hittable::Hittable,
//     material::MaterialPtr,
//     vec3::{Point3, Vec3, cross, dot, unit_vector},
// };

// #[derive(Debug)]
// pub struct Triangle {
//     v0: Point3,
//     v1: Point3,
//     v2: Point3,
//     normal: Vec3,
//     material: MaterialPtr,
//     bbox: Aabb,
// }

// impl Triangle {
//     pub fn new(v0: Point3, v1: Point3, v2: Point3, material: MaterialPtr) -> Self {
//         let edge1 = v1 - v0;
//         let edge2 = v2 - v0;
//         let normal = unit_vector(cross(&edge1, &edge2));

//         let min_point = Point3::new(
//             v0.x().min(v1.x()).min(v2.x()),
//             v0.y().min(v1.y()).min(v2.y()),
//             v0.z().min(v1.z()).min(v2.z()),
//         );

//         let max_point = Point3::new(
//             v0.x().max(v1.x()).max(v2.x()),
//             v0.y().max(v1.y()).max(v2.y()),
//             v0.z().max(v1.z()).max(v2.z()),
//         );

//         let bbox = Aabb::from_points(min_point, max_point);

//         Self {
//             v0,
//             v1,
//             v2,
//             normal,
//             material,
//             bbox,
//         }
//     }
// }

// impl Hittable for Triangle {
//     fn hit(
//         &self,
//         r: &crate::ray::Ray,
//         ray_t: crate::interval::Interval,
//         rec: &mut crate::hittable::HitRecord,
//     ) -> bool {
//         let edge1 = self.v1 - self.v0;
//         let edge2 = self.v2 - self.v0;
//         let h = cross(r.direction(), &edge2);
//         let a = dot(&edge1, &h);

//         if a.abs() < 1e-8 {
//             return false;
//         }

//         let f = 1.0 / a;
//         let s = *r.origin() - self.v0;
//         let u = f * dot(&s, &h);

//         if u < 0.0 || u > 1.0 {
//             return false;
//         }

//         let q = cross(&s, &edge1);
//         let v = f * dot(r.direction(), &q);

//         if v < 0.0 || u + v > 1.0 {
//             return false;
//         }

//         let t = f * dot(&edge2, &q);

//         if !ray_t.contains(t) {
//             return false;
//         }

//         rec.t = t;
//         rec.p = r.at(t);
//         rec.set_face_normal(r, self.normal);
//         rec.mat = Some(self.material.clone());
//         rec.u = u;
//         rec.v = v;

//         true
//     }

//     fn bounding_box(&self) -> Aabb {
//         self.bbox
//     }
// }

use crate::{
    aabb::Aabb,
    hittable::HitRecord,
    hittable::Hittable,
    interval::Interval,
    material::MaterialPtr,
    ray::Ray,
    vec3::{Point3, Vec3, cross, dot, unit_vector},
};

#[derive(Debug)]
pub struct Triangle {
    v0: Point3,
    v1: Point3,
    v2: Point3,
    normal: Vec3,
    material: MaterialPtr,
    bbox: Aabb,
}

impl Triangle {
    pub fn new(v0: Point3, v1: Point3, v2: Point3, material: MaterialPtr) -> Self {
        let edge1 = v1 - v0;
        let edge2 = v2 - v0;
        let normal = unit_vector(cross(&edge1, &edge2));

        let min_point = Point3::new(
            v0.x().min(v1.x()).min(v2.x()),
            v0.y().min(v1.y()).min(v2.y()),
            v0.z().min(v1.z()).min(v2.z()),
        );

        let max_point = Point3::new(
            v0.x().max(v1.x()).max(v2.x()),
            v0.y().max(v1.y()).max(v2.y()),
            v0.z().max(v1.z()).max(v2.z()),
        );

        let bbox = Aabb::from_points(min_point, max_point);

        Self {
            v0,
            v1,
            v2,
            normal,
            material,
            bbox,
        }
    }
}

impl Hittable for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        // Möller–Trumbore 算法
        let edge1 = self.v1 - self.v0;
        let edge2 = self.v2 - self.v0;
        let h = cross(r.direction(), &edge2);
        let a = dot(&edge1, &h);

        // 如果射线与三角形平面平行
        if a.abs() < 1e-8 {
            return false;
        }

        let f = 1.0 / a;
        let s = *r.origin() - self.v0;
        let u = f * dot(&s, &h);

        // 检查 u 是否在三角形范围内
        if u < 0.0 || u > 1.0 {
            return false;
        }

        let q = cross(&s, &edge1);
        let v = f * dot(r.direction(), &q);

        // 检查 v 是否在三角形范围内
        if v < 0.0 || u + v > 1.0 {
            return false;
        }

        // 计算射线参数 t
        let t = f * dot(&edge2, &q);

        // 检查 t 是否在有效范围内
        if !ray_t.contains(t) {
            return false;
        }

        // 填充命中记录
        rec.t = t;
        rec.p = r.at(t);
        rec.set_face_normal(r, self.normal);
        rec.mat = Some(self.material.clone());
        rec.u = u;
        rec.v = v;

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}
