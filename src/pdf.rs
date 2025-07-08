use crate::hittable::Hittable;
use crate::rtweekend::random_double;
use crate::vec3::{Point3, unit_vector};
use crate::{
    onb::Onb,
    rtweekend::PI,
    vec3::{Vec3, dot, random_cosine_direction, random_unit_vector},
};
use std::fmt;
use std::sync::Arc;

pub trait Pdf: Send + Sync {
    fn value(&self, direction: &Vec3) -> f64;
    fn generate(&self) -> Vec3;
}

impl fmt::Debug for dyn Pdf {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Pdf trait object")
    }
}

pub struct SpherePdf;

impl SpherePdf {
    pub fn new() -> Self {
        SpherePdf
    }
}

impl Pdf for SpherePdf {
    fn value(&self, _direction: &Vec3) -> f64 {
        1.0 / (4.0 * PI)
    }

    fn generate(&self) -> Vec3 {
        random_unit_vector()
    }
}

pub struct CosinePdf {
    uvw: Onb,
}

impl CosinePdf {
    pub fn new(w: Vec3) -> Self {
        Self { uvw: Onb::new(w) }
    }
}

impl Pdf for CosinePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        let cosine_theta = dot(&unit_vector(*direction), self.uvw.w());
        (cosine_theta / PI).max(0.0)
    }

    fn generate(&self) -> Vec3 {
        self.uvw.transform(random_cosine_direction())
    }
}

// pub struct HittablePdf<'a> {
//     objects: &'a dyn Hittable,
//     origin: Point3,
// }

// impl<'a> HittablePdf<'a> {
//     pub fn new(objects: &'a dyn Hittable, origin: Point3) -> Self {
//         Self { objects, origin }
//     }
// }

// impl<'a> Pdf for HittablePdf<'a> {
//     fn value(&self, direction: &Vec3) -> f64 {
//         self.objects.pdf_value(&self.origin, direction)
//     }

//     fn generate(&self) -> Vec3 {
//         self.objects.random(&self.origin)
//     }
// }

pub struct HittablePdf {
    objects: Arc<dyn Hittable + Send + Sync>,
    origin: Point3,
}

impl HittablePdf {
    pub fn new(objects: Arc<dyn Hittable + Send + Sync>, origin: Point3) -> Self {
        Self { objects, origin }
    }
}

impl Pdf for HittablePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        self.objects.pdf_value(&self.origin, direction)
    }

    fn generate(&self) -> Vec3 {
        self.objects.random(&self.origin)
    }
}

pub struct MixturePdf {
    p: [Arc<dyn Pdf + Send + Sync>; 2],
}

impl MixturePdf {
    pub fn new(p0: Arc<dyn Pdf + Send + Sync>, p1: Arc<dyn Pdf + Send + Sync>) -> Self {
        Self { p: [p0, p1] }
    }
}

impl Pdf for MixturePdf {
    fn value(&self, direction: &Vec3) -> f64 {
        0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
    }

    fn generate(&self) -> Vec3 {
        if random_double() < 0.5 {
            return self.p[0].generate();
        } else {
            return self.p[1].generate();
        }
    }
}

// pub struct MixturePdf<'a> {
//     p: [Box<dyn Pdf + 'a>; 2],
// }

// impl<'a> MixturePdf<'a> {
//     pub fn new(p0: Box<dyn Pdf + 'a>, p1: Box<dyn Pdf + 'a>) -> Self {
//         Self { p: [p0, p1] }
//     }
// }

// impl<'a> Pdf for MixturePdf<'a> {
//     fn value(&self, direction: &Vec3) -> f64 {
//         0.5 * self.p[0].value(direction) + 0.5 * self.p[1].value(direction)
//     }

//     fn generate(&self) -> Vec3 {
//         if random_double() < 0.5 {
//             self.p[0].generate()
//         } else {
//             self.p[1].generate()
//         }
//     }
// }
