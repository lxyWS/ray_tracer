use crate::aabb::Aabb;
use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::random_int;
use std::sync::Arc;

/// 可被射线击中的物体列表
#[derive(Debug)]
pub struct HittableList {
    pub objects: Vec<Arc<dyn Hittable + Send + Sync>>,
    bbox: Aabb,
}

impl HittableList {
    /// 创建空的物体列表
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            bbox: Aabb::new_empty(),
        }
    }

    /// 用单个物体创建列表
    pub fn with_object(object: Arc<dyn Hittable + Send + Sync>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    /// 清空列表中的所有物体
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// 向列表中添加一个物体
    pub fn add(&mut self, object: Arc<dyn Hittable + Send + Sync>) {
        self.objects.push(object.clone());
        self.bbox = Aabb::from_aabbs(self.bbox, object.bounding_box());
    }
}

/// 实现Hittable trait，使列表可被射线击中
impl Hittable for HittableList {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HitRecord) -> bool {
        let mut temp_rec = HitRecord::default();
        let mut hit_anything = false;
        // let mut closest_so_far = ray_tmax;
        let mut closest_so_far = ray_t.max;

        // 遍历所有物体，寻找最近的交点
        for object in &self.objects {
            if object.hit(&r, Interval::new(ray_t.min, closest_so_far), &mut temp_rec) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                *rec = temp_rec.clone(); // 更新交点信息
            }
        }

        hit_anything
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }

    fn pdf_value(&self, origin: &crate::vec3::Point3, direction: &crate::vec3::Vec3) -> f64 {
        let weight = 1.0 / self.objects.len() as f64;
        let mut sum = 0.0;

        for object in &self.objects {
            sum += weight * object.pdf_value(origin, direction);
        }

        sum
    }

    fn random(&self, origin: &crate::vec3::Point3) -> crate::vec3::Vec3 {
        let index = random_int(0, self.objects.len() as i32 - 1) as usize;
        self.objects[index].random(origin)
    }
}

// impl Debug for HittableList {
//     fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
//         f.debug_struct("HittableList")
//             .field("object_count", &self.objects.len())
//             .field("bbox", &self.bbox)
//             .finish()
//     }
// }

unsafe impl Sync for HittableList {}
