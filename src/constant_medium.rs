use std::sync::Arc;

use crate::{
    color::Color,
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::{Isotropic, Material},
    rtweekend::{INFINITY, random_double},
    texture::Texture,
    vec3::Vec3,
};

pub struct ConstantMedium {
    boundary: Arc<dyn Hittable + Send + Sync>,
    neg_inv_density: f64,
    phase_function: Arc<dyn Material + Send + Sync>,
}

impl ConstantMedium {
    pub fn new_with_texture(
        boundary: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        tex: Arc<dyn Texture + Send + Sync>,
    ) -> Self {
        let phase_function = Arc::new(Isotropic::new(tex));
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }

    pub fn new_with_color(
        boundary: Arc<dyn Hittable + Send + Sync>,
        density: f64,
        albedo: Color,
    ) -> Self {
        let phase_function = Arc::new(Isotropic::from_color(albedo));
        Self {
            boundary,
            neg_inv_density: -1.0 / density,
            phase_function,
        }
    }
}

impl Hittable for ConstantMedium {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        let mut rec1 = HitRecord::default();
        let mut rec2 = HitRecord::default();

        if !self.boundary.hit(r, Interval::UNIVERSE, &mut rec1) {
            return false;
        }

        if !self
            .boundary
            .hit(r, Interval::new(rec1.t + 0.0001, INFINITY), &mut rec2)
        {
            return false;
        }

        let mut rec1_t = if rec1.t < ray_t.min {
            ray_t.min
        } else {
            rec1.t
        };
        let mut rec2_t = if rec2.t > ray_t.max {
            ray_t.max
        } else {
            rec2.t
        };

        if rec1_t >= rec2_t {
            return false;
        }

        if rec1_t < 0.0 {
            rec1_t = 0.0;
        }

        let ray_length = r.direction().length();
        let distance_inside_boundary = (rec2_t - rec1_t) * ray_length;
        let hit_distance = self.neg_inv_density * random_double().ln();

        if hit_distance > distance_inside_boundary {
            return false;
        }

        rec.t = rec1_t + hit_distance / ray_length;
        rec.p = r.at(rec.t);

        rec.normal = Vec3::new(1.0, 0.0, 0.0);
        rec.front_face = true;
        rec.mat = Some(self.phase_function.clone());

        true
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.boundary.bounding_box()
    }
}
