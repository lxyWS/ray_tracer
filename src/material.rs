use crate::pdf::Pdf;
use crate::{
    color::Color,
    hittable::HitRecord,
    onb::Onb,
    pdf::{CosinePdf, SpherePdf},
    ray::Ray,
    rtweekend::{PI, random_double},
    texture::{SolidColor, Texture},
    vec3::{
        Point3, dot, random_cosine_direction, random_unit_vector, reflect, refract, unit_vector,
    },
};
use std::fmt;
use std::sync::Arc;

#[derive(Default)]
pub struct ScatterRecord {
    pub attenuation: Color,
    pub pdf_ptr: Option<Arc<dyn Pdf + Send + Sync>>,
    pub skip_pdf: bool,
    pub skip_pdf_ray: Option<Ray>,
}

impl fmt::Debug for ScatterRecord {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("ScatterRecord")
            .field("attenuation", &self.attenuation)
            .field("pdf_ptr", &self.pdf_ptr.as_ref().map(|_| "Pdf"))
            .field("skip_pdf", &self.skip_pdf)
            .field("skip_pdf_ray", &self.skip_pdf_ray)
            .finish()
    }
}

pub trait Material: Send + Sync + std::fmt::Debug {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        // _attenuation: &mut Color, // 材质吸收后的剩余光线能量
        // _scattered: &mut Ray,     // 散射后的光线
        // _pdf: &mut f64,
        _srec: &mut ScatterRecord,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, _rec: &HitRecord, _u: f64, _v: f64, _p: &Point3) -> Color {
        Color::new(0.0, 0.0, 0.0)
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        0.0
    }
}

#[derive(Debug)]
pub struct Lambertian {
    // 朗伯表面
    // albedo: Color,
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }

    pub fn from_texture(texture: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex: texture }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        // attenuation: &mut Color, // 材质吸收后的剩余光线能量
        // scattered: &mut Ray,     // 散射后的光线
        // pdf: &mut f64,
        srec: &mut ScatterRecord,
    ) -> bool {
        // let scatter_direction = rec.normal + random_unit_vector(); // 散射方向
        // let scatter_direction = random_on_hemisphere(&rec.normal);

        // let scatter_direction = if scatter_direction.near_zero() {
        //     rec.normal
        // } else {
        //     scatter_direction
        // };
        // *scattered = Ray::with_origin_dir_time(rec.p, scatter_direction, r_in.time());

        // let uvw = Onb::new(rec.normal);
        // let scatter_direction = uvw.transform(random_cosine_direction());

        // *scattered = Ray::with_origin_dir_time(rec.p, unit_vector(scatter_direction), r_in.time());
        // *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        // *pdf = dot(uvw.w(), scattered.direction()) / PI;

        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(CosinePdf::new(rec.normal)));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, rec: &HitRecord, scattered: &Ray) -> f64 {
        // let cos_theta = dot(&rec.normal, &unit_vector(*scattered.direction()));
        // if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
        // 1.0 / (2.0 * PI)

        let cos_theta = dot(&rec.normal, &unit_vector(*scattered.direction()));
        if cos_theta < 0.0 { 0.0 } else { cos_theta / PI }
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
        // attenuation: &mut Color, // 材质吸收后的剩余光线能量
        // scattered: &mut Ray,     // 散射后的光线
        // _pdf: &mut f64,
        srec: &mut ScatterRecord,
    ) -> bool {
        let mut reflected = reflect(r_in.direction(), &rec.normal);
        reflected = unit_vector(reflected) + self.fuzz * random_unit_vector();

        // *scattered = Ray::with_origin_dir_time(rec.p, reflected, r_in.time());
        // *attenuation = self.albedo;

        srec.attenuation = self.albedo;
        srec.pdf_ptr = None;
        srec.skip_pdf = true;
        srec.skip_pdf_ray = Some(Ray::with_origin_dir_time(rec.p, reflected, r_in.time()));

        // dot(scattered.direction(), &rec.normal) > 0.0
        true
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
        // attenuation: &mut Color, // 材质吸收后的剩余光线能量
        // scattered: &mut Ray,     // 散射后的光线
        // _pdf: &mut f64,
        srec: &mut ScatterRecord,
    ) -> bool {
        // *attenuation = Color::new(1.0, 1.0, 1.0);
        srec.attenuation = Color::new(1.0, 1.0, 1.0);
        srec.pdf_ptr = None;
        srec.skip_pdf = true;

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
        // *scattered = Ray::with_origin_dir_time(rec.p, direction, r_in.time());
        srec.skip_pdf_ray = Some(Ray::with_origin_dir_time(rec.p, direction, r_in.time()));
        true
    }
}

#[derive(Debug)]
pub struct DiffuseLight {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl DiffuseLight {
    pub fn new(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }

    pub fn from_color(emit: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(emit)),
        }
    }
}

impl Material for DiffuseLight {
    fn scatter(
        &self,
        _r_in: &Ray,
        _rec: &HitRecord,
        // _attenuation: &mut Color, // 材质吸收后的剩余光线能量
        // _scattered: &mut Ray,     // 散射后的光线
        // _pdf: &mut f64,
        _srec: &mut ScatterRecord,
    ) -> bool {
        false
    }

    fn emitted(&self, _r_in: &Ray, rec: &HitRecord, u: f64, v: f64, p: &Point3) -> Color {
        // self.tex.value(u, v, p)
        if !rec.front_face {
            return Color::new(0.0, 0.0, 0.0);
        }
        self.tex.value(u, v, p)
    }
}

pub type MaterialPtr = Arc<dyn Material + Send + Sync>; // Material trait 的智能指针类型

#[derive(Debug)]
pub struct Isotropic {
    tex: Arc<dyn Texture + Send + Sync>,
}

impl Isotropic {
    pub fn new(tex: Arc<dyn Texture + Send + Sync>) -> Self {
        Self { tex }
    }

    pub fn from_color(albedo: Color) -> Self {
        Self {
            tex: Arc::new(SolidColor::new(albedo)),
        }
    }
}

impl Material for Isotropic {
    fn scatter(
        &self,
        _r_in: &Ray,
        rec: &HitRecord,
        // attenuation: &mut Color, // 材质吸收后的剩余光线能量
        // scattered: &mut Ray,     // 散射后的光线
        // pdf: &mut f64,
        srec: &mut ScatterRecord,
    ) -> bool {
        // *scattered = Ray::with_origin_dir_time(rec.p, random_unit_vector(), r_in.time());
        // *attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        // *pdf = 1.0 / (4.0 * PI);

        srec.attenuation = self.tex.value(rec.u, rec.v, &rec.p);
        srec.pdf_ptr = Some(Arc::new(SpherePdf::new()));
        srec.skip_pdf = false;
        true
    }

    fn scattering_pdf(&self, _r_in: &Ray, _rec: &HitRecord, _scattered: &Ray) -> f64 {
        1.0 / (4.0 * PI)
    }
}
