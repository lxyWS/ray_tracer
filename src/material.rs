use crate::{
    color::Color,
    hittable::HitRecord,
    ray::Ray,
    rtweekend::random_double,
    vec3::{dot, random_unit_vector, reflect, refract, unit_vector},
};
use std::sync::Arc;

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, // 材质吸收后的剩余光线能量
        scattered: &mut Ray,     // 散射后的光线
    ) -> bool;
}

#[derive(Debug)]
pub struct Lambertian {
    // 朗伯表面
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, // 材质吸收后的剩余光线能量
        scattered: &mut Ray,     // 散射后的光线
    ) -> bool {
        let scatter_direction = rec.normal + random_unit_vector(); // 散射方向

        let scatter_direction = if scatter_direction.near_zero() {
            rec.normal
        } else {
            scatter_direction
        };
        *scattered = Ray::with_origin_dir_time(rec.p, scatter_direction, r_in.time());
        *attenuation = self.albedo;
        true
    }
}

#[derive(Debug)]
pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Self {
            albedo,
            fuzz: fuzz.min(1.0),
        }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, // 材质吸收后的剩余光线能量
        scattered: &mut Ray,     // 散射后的光线
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(reflected) + self.fuzz * random_unit_vector();

        *scattered = Ray::with_origin_dir_time(rec.p, reflected, r_in.time());
        *attenuation = self.albedo;

        dot(scattered.direction(), &rec.normal) > 0.0
    }
}

#[derive(Debug)]
pub struct Dielectric {
    // 电介质
    refraction_index: f64, // 折射率
}

impl Dielectric {
    pub fn new(refraction_index: f64) -> Self {
        Self {
            refraction_index: refraction_index,
        }
    }

    // 施莱克近似
    fn reflectance(cosine: f64, ref_idx: f64) -> f64 {
        let r0 = ((1.0 - ref_idx) / (1.0 + ref_idx)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        r_in: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color, // 材质吸收后的剩余光线能量
        scattered: &mut Ray,     // 散射后的光线
    ) -> bool {
        *attenuation = Color::new(1.0, 1.0, 1.0);

        let ri = if rec.front_face {
            1.0 / self.refraction_index
        } else {
            self.refraction_index
        };

        let unit_direction = unit_vector(*r_in.direction());

        let cos_theta = (-dot(&unit_direction, &rec.normal)).min(1.0);
        let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

        let cannot_refract = ri * sin_theta > 1.0;
        let direction = if cannot_refract || Self::reflectance(cos_theta, ri) > random_double() {
            reflect(&unit_direction, &rec.normal)
        } else {
            refract(&unit_direction, &rec.normal, ri)
        };
        // let refracted = refract(&unit_direction, &rec.normal, ri);
        *scattered = Ray::with_origin_dir_time(rec.p, direction, r_in.time());
        true
    }
}

pub type MaterialPtr = Arc<dyn Material + Send + Sync>; // Material trait 的智能指针类型
