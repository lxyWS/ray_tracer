use crate::vec3::{Point3, Vec3};

/// 表示三维空间中的射线
#[derive(Copy, Clone, Debug)]
pub struct Ray {
    orig: Point3, // 原点
    dir: Vec3,    // 方向向量
    tm: f64,      // 时间
}

impl Ray {
    // 默认构造
    pub fn new() -> Self {
        Self {
            orig: Point3::default(),
            dir: Vec3::default(),
            tm: 0.0,
        }
    }

    // 使用原点和方向向量构造
    pub fn with_origin_dir(origin: Point3, direction: Vec3) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: 0.0,
        }
    }

    pub fn with_origin_dir_time(origin: Point3, direction: Vec3, time: f64) -> Self {
        Self {
            orig: origin,
            dir: direction,
            tm: time,
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

    pub fn time(&self) -> f64 {
        self.tm
    }

    // 计算射线上参数为`t`的点
    // - `t`: 射线参数（t=0时为原点，t=1时为原点+方向向量）
    pub fn at(&self, t: f64) -> Point3 {
        self.orig + t * self.dir
    }
}
