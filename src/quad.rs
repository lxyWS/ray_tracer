use std::sync::Arc;

use crate::{
    aabb::Aabb,
    hittable::{HitRecord, Hittable},
    hittable_list::HittableList,
    interval::Interval,
    material::Material,
    vec3::{Point3, Vec3, cross, dot, unit_vector},
};

pub struct Quad {
    q: Point3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: Aabb,
    normal: Vec3,
    d: f64,
}

impl Quad {
    pub fn new(q: Point3, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        let n = cross(&u, &v);
        let normal = unit_vector(n);
        let d = dot(&normal, &q);
        let w = n / dot(&n, &n);

        let mut quad = Self {
            q,
            u,
            v,
            w,
            mat,
            bbox: Aabb::new_empty(),
            normal,
            d,
        };

        quad.set_bounding_box();
        quad
    }

    fn set_bounding_box(&mut self) {
        let p1 = self.q;
        let p2 = self.q + self.u;
        let p3 = self.q + self.v;
        let p4 = self.q + self.u + self.v;

        let bbox1 = Aabb::from_points(p1, p4);
        let bbox2 = Aabb::from_points(p2, p3);

        self.bbox = Aabb::from_aabbs(bbox1, bbox2);
    }

    fn is_interior(&self, a: f64, b: f64, rec: &mut HitRecord) -> bool {
        let unit_interval = Interval::new(0.0, 1.0);
        if (!unit_interval.contains(a)) || (!unit_interval.contains(b)) {
            return false;
        }

        rec.u = a;
        rec.v = b;
        true
    }
}

impl Hittable for Quad {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let denom = dot(&self.normal, &r.direction());

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - dot(&self.normal, &*r.origin())) / denom;

        if !ray_t.contains(t) {
            return false;
        }

        // Determine if the hit point lies within the planar shape using its plane coordinates.
        let intersection = r.at(t);

        let planar_hitpt_vector = intersection - self.q;
        let alpha = dot(&self.w, &cross(&planar_hitpt_vector, &self.v));
        let beta = dot(&self.w, &cross(&self.u, &planar_hitpt_vector));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(self.mat.clone());
        rec.set_face_normal(r, self.normal);

        true
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

pub fn box_new(a: Point3, b: Point3, mat: Arc<dyn Material>) -> Arc<HittableList> {
    let mut sides = HittableList::new();

    let min = Point3::new(a.x().min(b.x()), a.y().min(b.y()), a.z().min(b.z()));
    let max = Point3::new(a.x().max(b.x()), a.y().max(b.y()), a.z().max(b.z()));

    let dx = Vec3::new(max.x() - min.x(), 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y() - min.y(), 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z() - min.z());

    // 前表面（正z方向）
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), max.z()),
        dx,
        dy,
        mat.clone(),
    )));
    // 右侧面（正x方向）
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), max.z()),
        -dz,
        dy,
        mat.clone(),
    )));
    // 后表面（负z方向）
    sides.add(Arc::new(Quad::new(
        Point3::new(max.x(), min.y(), min.z()),
        -dx,
        dy,
        mat.clone(),
    )));
    // 左侧面（负x方向）
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dz,
        dy,
        mat.clone(),
    )));
    // 顶面（正y方向）
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), max.y(), max.z()),
        dx,
        -dz,
        mat.clone(),
    )));
    // 底面（负y方向）
    sides.add(Arc::new(Quad::new(
        Point3::new(min.x(), min.y(), min.z()),
        dx,
        dz,
        mat,
    )));

    Arc::new(sides)
}
