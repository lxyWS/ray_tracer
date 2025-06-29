// use std::{cmp::Ordering, sync::Arc};

// use rand::Rng;

// use crate::{
//     aabb::Aabb,
//     hittable::{HitRecord, Hittable},
//     interval::Interval,
//     ray::Ray,
//     rtweekend::random_int,
// };

// #[derive(Debug)]
// pub struct BvhNode {
//     left: Arc<dyn Hittable + Send + Sync>,
//     right: Arc<dyn Hittable + Send + Sync>,
//     bbox: Aabb,
// }

// impl BvhNode {
//     pub fn new(objects: &mut Vec<Arc<dyn Hittable + Send + Sync>>) -> Arc<Self> {
//         Self::from_objects(objects, 0, objects.len())
//     }

//     pub fn from_objects(
//         objects: &mut [Arc<dyn Hittable + Send + Sync>],
//         start: usize,
//         end: usize,
//     ) -> Arc<Self> {
//         let object_span = end - start;
//         let axis = random_int(0, 2);

//         // 轴选择
//         // let comparator = match axis {
//         //     0 => Self::box_compare,
//         //     1 => Self::box_y_compare,
//         //     _ => Self::box_z_compare,
//         // };

//         let comparator = |a: &Arc<dyn Hittable + Send + Sync>,
//                           b: &Arc<dyn Hittable + Send + Sync>| {
//             let a_bbox = a.bounding_box();
//             let b_bbox = b.bounding_box();
//             let a_min = a_bbox.axis_interval(axis).min;
//             let b_min = b_bbox.axis_interval(axis).min;
//             a_min.partial_cmp(&b_min).unwrap()
//         };

//         let (left, right) = match object_span {
//             1 => {
//                 let object = objects[start].clone();
//                 (object.clone(), object)
//             }
//             2 => {
//                 let first = objects[start].clone();
//                 let second = objects[start + 1].clone();

//                 // if comparator(&first, &second) == std::cmp::Ordering::Less {
//                 //     (first, second)
//                 // } else {
//                 //     (second, first)
//                 // }
//             }
//             _ => {
//                 let mid = start + object_span / 2;

//                 // objects[start..end].sort_by(|a, b| comparator(a, b));
//                 objects[start..end].sort_by(comparator);

//                 let left = Self::from_objects(objects, start, mid);
//                 let right = Self::from_objects(objects, mid, end);

//                 (left, right)
//             }
//         };

//         let bbox = Aabb::from_aabbs(left.bounding_box(), right.bounding_box());

//         Arc::new(Self { left, right, bbox })
//     }

//     fn box_compare(
//         a: &Arc<dyn Hittable + Send + Sync>,
//         b: &Arc<dyn Hittable + Send + Sync>,
//         axis_index: usize,
//     ) -> Ordering {
//         let a_axis_interval = a.bounding_box().axis_interval(axis_index);
//         let b_axis_interval = b.bounding_box().axis_interval(axis_index);
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
//     fn hit(&self, r: &Ray, mut ray_t: Interval, rec: &mut HitRecord) -> bool {
//         if !self.bbox.hit(r, ray_t) {
//             return false;
//         }

//         let hit_left = self.left.hit(r, ray_t, rec);
//         if hit_left {
//             ray_t.max = rec.t;
//         }

//         let hit_right = self.right.hit(r, ray_t, rec);

//         hit_left || hit_right
//     }

//     fn bounding_box(&self) -> Aabb {
//         self.bbox
//     }
// }

// unsafe impl Send for BvhNode {}
// unsafe impl Sync for BvhNode {}

use std::{cmp::Ordering, sync::Arc};

use crate::{
    aabb::Aabb, hittable::Hittable, hittable_list::HittableList, interval::Interval,
    rtweekend::random_int,
};

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
                let mid = start + object_span / 2;
                objects[start..end].sort_by(|a, b| comparator(a, b));

                let left_node = Self::from_objects(objects, start, mid);
                let right_node = Self::from_objects(objects, mid, end);

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
