use rand::Rng;
use std::f64;
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

pub fn random_int(min: i32, max: i32) -> i32 {
    random_double_range(min as f64, (max + 1) as f64) as i32
}

// 类型别名
pub type SharedPtr<T> = Arc<T>;
pub use self::SharedPtr as make_shared;
