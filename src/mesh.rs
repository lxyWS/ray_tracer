// use std::{error::Error, path::Path, sync::Arc};
// use tobj::{LoadOptions, load_obj};

// use crate::{
//     bvh::BvhNode, hittable::Hittable, hittable_list::HittableList, material::MaterialPtr,
//     triangle::Triangle, vec3::Point3,
// };

// #[derive(Debug)]
// pub struct Mesh {
//     bvh: Arc<BvhNode>,
// }

// impl Mesh {
//     pub fn from_obj(
//         path: impl AsRef<Path> + std::fmt::Debug,
//         material: MaterialPtr,
//     ) -> Result<Self, Box<dyn Error>> {
//         let (models, _) = load_obj(
//             path,
//             &LoadOptions {
//                 triangulate: true,
//                 ..Default::default()
//             },
//         )?;

//         let mut triangles = HittableList::new();

//         for model in models {
//             let mesh = model.mesh;

//             if mesh.positions.is_empty() {
//                 continue;
//             }

//             for face in mesh.indices.chunks(3) {
//                 // 获取三个顶点索引
//                 let i0 = face[0] as usize;
//                 let i1 = face[1] as usize;
//                 let i2 = face[2] as usize;

//                 // 获取顶点坐标
//                 let v0 = Point3::new(
//                     mesh.positions[3 * i0] as f64,
//                     mesh.positions[3 * i0 + 1] as f64,
//                     mesh.positions[3 * i0 + 2] as f64,
//                 );

//                 let v1 = Point3::new(
//                     mesh.positions[3 * i1] as f64,
//                     mesh.positions[3 * i1 + 1] as f64,
//                     mesh.positions[3 * i1 + 2] as f64,
//                 );

//                 let v2 = Point3::new(
//                     mesh.positions[3 * i2] as f64,
//                     mesh.positions[3 * i2 + 1] as f64,
//                     mesh.positions[3 * i2 + 2] as f64,
//                 );

//                 // 创建三角形并添加到列表
//                 triangles.add(Arc::new(Triangle::new(v0, v1, v2, material.clone())));
//             }
//         }

//         let bvh = BvhNode::new(&triangles);

//         Ok(Self { bvh })
//     }
// }

// impl Hittable for Mesh {
//     fn hit(
//         &self,
//         r: &crate::ray::Ray,
//         ray_t: crate::interval::Interval,
//         rec: &mut crate::hittable::HitRecord,
//     ) -> bool {
//         self.bvh.hit(r, ray_t, rec)
//     }

//     fn bounding_box(&self) -> crate::aabb::Aabb {
//         self.bvh.bounding_box()
//     }
// }

use std::{error::Error, path::Path, sync::Arc};
use tobj::{LoadOptions, load_obj};

use crate::{
    bvh::BvhNode, hittable::Hittable, hittable_list::HittableList, material::MaterialPtr,
    triangle::Triangle, vec3::Point3,
};

#[derive(Debug)]
pub struct Mesh {
    bvh: Arc<BvhNode>,
}

impl Mesh {
    pub fn from_obj(
        path: impl AsRef<Path> + std::fmt::Debug,
        material: MaterialPtr,
    ) -> Result<Self, Box<dyn Error>> {
        let options = LoadOptions {
            triangulate: true, // 确保所有面都转换为三角形
            single_index: true,
            ..Default::default()
        };

        let (models, _) = load_obj(path, &options)?;
        let mut triangles = HittableList::new();

        println!("Loading OBJ file...");

        for model in models {
            let mesh = &model.mesh;
            println!(
                "  - Model: {}, vertices: {}, faces: {}",
                model.name,
                mesh.positions.len() / 3,
                mesh.indices.len() / 3
            );

            // 遍历所有三角形面
            for face in mesh.indices.chunks(3) {
                if face.len() < 3 {
                    continue;
                }

                // 获取顶点索引
                let i0 = face[0] as usize;
                let i1 = face[1] as usize;
                let i2 = face[2] as usize;

                // 获取顶点坐标
                let v0 = Point3::new(
                    mesh.positions[3 * i0] as f64,
                    mesh.positions[3 * i0 + 1] as f64,
                    mesh.positions[3 * i0 + 2] as f64,
                );

                let v1 = Point3::new(
                    mesh.positions[3 * i1] as f64,
                    mesh.positions[3 * i1 + 1] as f64,
                    mesh.positions[3 * i1 + 2] as f64,
                );

                let v2 = Point3::new(
                    mesh.positions[3 * i2] as f64,
                    mesh.positions[3 * i2 + 1] as f64,
                    mesh.positions[3 * i2 + 2] as f64,
                );

                // 创建三角形
                triangles.add(Arc::new(Triangle::new(v0, v1, v2, material.clone())));
            }
        }

        println!("Loaded {} triangles", triangles.objects.len());

        // 构建 BVH 加速结构
        let bvh = BvhNode::new(&triangles);
        println!("BVH bounding box: {:?}", bvh.bounding_box());

        Ok(Self { bvh })
    }
}

impl Hittable for Mesh {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::hittable::HitRecord,
    ) -> bool {
        self.bvh.hit(r, ray_t, rec)
    }

    fn bounding_box(&self) -> crate::aabb::Aabb {
        self.bvh.bounding_box()
    }
}
