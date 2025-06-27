use std::f64;
// use std::f64::consts::PI;
use rand::Rng;
use std::sync::Arc;

// 常量定义
pub const INFINITY: f64 = f64::INFINITY;
pub const PI: f64 = 3.1415926535897932385;

// 工具函数
/// 将角度转换为弧度
pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * PI / 180.0
}

pub fn random_double() -> f64 {
    rand::thread_rng().gen_range(0.0..1.0)
}

pub fn random_double_range(min: f64, max: f64) -> f64 {
    rand::thread_rng().gen_range(min..max)
}

// 类型别名
pub type SharedPtr<T> = Arc<T>;
pub use self::SharedPtr as make_shared;
