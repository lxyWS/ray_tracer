use crate::hittable::{HitRecord, Hittable};
use crate::interval::Interval;
use crate::ray::Ray;
use std::sync::Arc;

/// 可被射线击中的物体列表
pub struct HittableList {
    objects: Vec<Arc<dyn Hittable>>,
}

impl HittableList {
    /// 创建空的物体列表
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
        }
    }

    /// 用单个物体创建列表
    pub fn with_object(object: Arc<dyn Hittable>) -> Self {
        let mut list = Self::new();
        list.add(object);
        list
    }

    /// 清空列表中的所有物体
    pub fn clear(&mut self) {
        self.objects.clear();
    }

    /// 向列表中添加一个物体
    pub fn add(&mut self, object: Arc<dyn Hittable>) {
        self.objects.push(object);
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
}
