use crate::{
    hittable::{HitRecord, Hittable},
    interval::Interval,
    material::Material,
    ray::Ray,
    triangle::Triangle,
    vec3::Point3,
};
use std::sync::Arc;
use std::{error::Error, path::Path};
use tobj::{LoadOptions, load_obj};

#[derive(Debug)]
pub struct ObjModel {
    pub triangles: Vec<Arc<dyn Hittable + Send + Sync>>,
    pub bbox_min: Point3,
    pub bbox_max: Point3,
}

impl ObjModel {
    pub fn load<P: AsRef<Path>>(
        path: P,
        material: Arc<dyn Material + Send + Sync>,
        scale: f64,
        offset: Point3,
    ) -> Result<Self, Box<dyn Error>> {
        let obj = load_obj(
            path.as_ref(),
            &LoadOptions {
                triangulate: true,
                single_index: true,
                ..Default::default()
            },
        )?;

        let (models, _) = obj;
        let mut triangles: Vec<Arc<dyn Hittable + Send + Sync>> = Vec::new();
        let mut bbox_min = Point3::new(f64::INFINITY, f64::INFINITY, f64::INFINITY);
        let mut bbox_max = Point3::new(f64::NEG_INFINITY, f64::NEG_INFINITY, f64::NEG_INFINITY);

        for model in models {
            let mesh = model.mesh;
            let indices = mesh.indices;
            let positions = mesh.positions;

            for chunk in indices.chunks(3) {
                let i0 = chunk[0] as usize * 3;
                let i1 = chunk[1] as usize * 3;
                let i2 = chunk[2] as usize * 3;

                let v0 = Point3::new(
                    positions[i0] as f64 * scale + offset.x(),
                    positions[i0 + 1] as f64 * scale + offset.y(),
                    positions[i0 + 2] as f64 * scale + offset.z(),
                );

                let v1 = Point3::new(
                    positions[i1] as f64 * scale + offset.x(),
                    positions[i1 + 1] as f64 * scale + offset.y(),
                    positions[i1 + 2] as f64 * scale + offset.z(),
                );

                let v2 = Point3::new(
                    positions[i2] as f64 * scale + offset.x(),
                    positions[i2 + 1] as f64 * scale + offset.y(),
                    positions[i2 + 2] as f64 * scale + offset.z(),
                );

                // 更新包围盒
                bbox_min = bbox_min.min(v0).min(v1).min(v2);
                bbox_max = bbox_max.max(v0).max(v1).max(v2);

                let triangle = Triangle::new(v0, v1, v2, material.clone());
                triangles.push(Arc::new(triangle) as Arc<dyn Hittable + Send + Sync>);
            }
        }

        Ok(Self {
            triangles,
            bbox_min,
            bbox_max,
        })
    }
}

impl Hittable for ObjModel {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut hit_anything = false;
        let mut closest_so_far = ray_t.max;

        for triangle in &self.triangles {
            let mut temp_rec = HitRecord::default();
            if triangle.hit(r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec;
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        crate::aabb::Aabb::from_points(self.bbox_min, self.bbox_max)
    }
}
