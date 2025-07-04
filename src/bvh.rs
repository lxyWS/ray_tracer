// use std::{cmp::Ordering, sync::Arc};

// use crate::{aabb::Aabb, hittable::Hittable, hittable_list::HittableList, interval::Interval};

// #[derive(Debug)]
// pub struct BvhNode {
//     left: Arc<dyn Hittable + Send + Sync>,
//     right: Arc<dyn Hittable + Send + Sync>,
//     bbox: Aabb,
// }

// impl BvhNode {
//     pub fn new(list: &HittableList) -> Arc<Self> {
//         let mut objects = list.objects.clone();
//         let len = objects.len();
//         Self::from_objects(&mut objects, 0, len)
//     }

//     pub fn from_objects(
//         objects: &mut Vec<Arc<dyn Hittable + Send + Sync>>,
//         start: usize,
//         end: usize,
//     ) -> Arc<Self> {
//         let object_span = end - start;

//         let mut bbox = Aabb::EMPTY;
//         for i in start..end {
//             bbox = Aabb::from_aabbs(bbox, objects[i].bounding_box());
//         }

//         let axis = bbox.longest_axis();

//         let comparator = match axis {
//             0 => Self::box_x_compare,
//             1 => Self::box_y_compare,
//             _ => Self::box_z_compare,
//         };

//         let (left, right) = match object_span {
//             1 => {
//                 let object = objects[start].clone();
//                 (object.clone(), object)
//             }
//             2 => {
//                 let first = objects[start].clone();
//                 let second = objects[start + 1].clone();

//                 (first, second)
//             }

//             _ => {
//                 let mid = start + object_span / 2;
//                 objects[start..end].sort_by(|a, b| comparator(a, b));

//                 let left_node = Self::from_objects(objects, start, mid);
//                 let right_node = Self::from_objects(objects, mid, end);

//                 (
//                     left_node as Arc<dyn Hittable + Send + Sync>,
//                     right_node as Arc<dyn Hittable + Send + Sync>,
//                 )
//             }
//         };

//         Arc::new(Self { left, right, bbox })
//     }

//     fn box_compare(
//         a: &Arc<dyn Hittable + Send + Sync>,
//         b: &Arc<dyn Hittable + Send + Sync>,
//         axis_index: usize,
//     ) -> Ordering {
//         let a_bbox = a.bounding_box();
//         let b_bbox = b.bounding_box();

//         let a_axis_interval = a_bbox.axis_interval(axis_index);
//         let b_axis_interval = b_bbox.axis_interval(axis_index);

//         a_axis_interval
//             .min
//             .partial_cmp(&b_axis_interval.min)
//             .unwrap()
//     }

//     fn box_x_compare(
//         a: &Arc<dyn Hittable + Send + Sync>,
//         b: &Arc<dyn Hittable + Send + Sync>,
//     ) -> Ordering {
//         Self::box_compare(a, b, 0)
//     }

//     fn box_y_compare(
//         a: &Arc<dyn Hittable + Send + Sync>,
//         b: &Arc<dyn Hittable + Send + Sync>,
//     ) -> Ordering {
//         Self::box_compare(a, b, 1)
//     }

//     fn box_z_compare(
//         a: &Arc<dyn Hittable + Send + Sync>,
//         b: &Arc<dyn Hittable + Send + Sync>,
//     ) -> Ordering {
//         Self::box_compare(a, b, 2)
//     }
// }

// impl Hittable for BvhNode {
//     fn hit(
//         &self,
//         r: &crate::ray::Ray,
//         ray_t: crate::interval::Interval,
//         rec: &mut crate::hittable::HitRecord,
//     ) -> bool {
//         if !self.bbox.hit(r, ray_t) {
//             return false;
//         }

//         let hit_left = self.left.hit(r, ray_t, rec);

//         let new_ray_t = if hit_left {
//             Interval::new(ray_t.min, rec.t)
//         } else {
//             ray_t
//         };

//         let hit_right = self.right.hit(r, new_ray_t, rec);

//         hit_left || hit_right
//     }

//     fn bounding_box(&self) -> Aabb {
//         self.bbox
//     }
// }

// unsafe impl Send for BvhNode {}

// unsafe impl Sync for BvhNode {}

use rayon::join;
use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::Aabb, hittable::Hittable, hittable_list::HittableList, interval::Interval};

#[derive(Debug)]
pub struct BvhNode {
    left: Arc<dyn Hittable + Send + Sync>,
    right: Arc<dyn Hittable + Send + Sync>,
    bbox: Aabb,
}

impl BvhNode {
    pub fn new(list: &HittableList) -> Arc<Self> {
        let mut objects = list.objects.clone();
        let len = objects.len();
        Self::from_objects(&mut objects, 0, len)
    }

    pub fn from_objects(
        objects: &mut Vec<Arc<dyn Hittable + Send + Sync>>,
        start: usize,
        end: usize,
    ) -> Arc<Self> {
        let object_span = end - start;

        let mut bbox = Aabb::EMPTY;
        for i in start..end {
            bbox = Aabb::from_aabbs(bbox, objects[i].bounding_box());
        }

        let axis = bbox.longest_axis();

        let comparator = match axis {
            0 => Self::box_x_compare,
            1 => Self::box_y_compare,
            _ => Self::box_z_compare,
        };

        let (left, right) = match object_span {
            1 => {
                let object = objects[start].clone();
                (object.clone(), object)
            }
            2 => {
                let first = objects[start].clone();
                let second = objects[start + 1].clone();
                (first, second)
            }
            _ => {
                // 对当前区间的对象进行排序
                objects[start..end].sort_by(|a, b| comparator(a, b));
                let mid = start + object_span / 2;

                // 将当前区间的对象分成两个独立的部分
                let mut left_objects = objects[start..mid].to_vec();
                let mut right_objects = objects[mid..end].to_vec();

                // 提前计算长度
                let left_len = left_objects.len();
                let right_len = right_objects.len();

                // 使用 rayon::join 并行构建左右子树
                let (left_node, right_node) = join(
                    || Self::from_objects(&mut left_objects, 0, left_len),
                    || Self::from_objects(&mut right_objects, 0, right_len),
                );

                (
                    left_node as Arc<dyn Hittable + Send + Sync>,
                    right_node as Arc<dyn Hittable + Send + Sync>,
                )
            }
        };

        Arc::new(Self { left, right, bbox })
    }

    fn box_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
        axis_index: usize,
    ) -> Ordering {
        let a_bbox = a.bounding_box();
        let b_bbox = b.bounding_box();

        let a_axis_interval = a_bbox.axis_interval(axis_index);
        let b_axis_interval = b_bbox.axis_interval(axis_index);

        a_axis_interval
            .min
            .partial_cmp(&b_axis_interval.min)
            .unwrap()
    }

    fn box_x_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        Self::box_compare(a, b, 0)
    }

    fn box_y_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        Self::box_compare(a, b, 1)
    }

    fn box_z_compare(
        a: &Arc<dyn Hittable + Send + Sync>,
        b: &Arc<dyn Hittable + Send + Sync>,
    ) -> Ordering {
        Self::box_compare(a, b, 2)
    }
}

impl Hittable for BvhNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }

        let hit_left = self.left.hit(r, ray_t, rec);

        let new_ray_t = if hit_left {
            Interval::new(ray_t.min, rec.t)
        } else {
            ray_t
        };

        let hit_right = self.right.hit(r, new_ray_t, rec);

        hit_left || hit_right
    }

    fn bounding_box(&self) -> Aabb {
        self.bbox
    }
}

unsafe impl Send for BvhNode {}
unsafe impl Sync for BvhNode {}
