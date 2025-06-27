use crate::vec3::{Point3, Vec3};

/// 表示三维空间中的射线
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Point3, // 原点
    dir: Vec3,    // 方向向量
}

impl Ray {
    // 默认构造
    pub fn new() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
        }
    }

    // 使用原点和方向向量构造
    pub fn with_origin_dir(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
        }
    }

    // 获取原点
    pub fn origin(&self) -> &Point3 {
        &self.orig
    }

    // 获取方向向量
    pub fn direction(&self) -> &Vec3 {
        &self.dir
    }

    // 计算射线上参数为`t`的点
    // - `t`: 射线参数（t=0时为原点，t=1时为原点+方向向量）
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
