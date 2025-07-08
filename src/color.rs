use crate::interval::Interval;
use crate::vec3::Vec3;

// 颜色类型别名（等同于Vec3）
pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        return linear_component.sqrt();
    }
    0.0
}

/// 向输出流写入颜色像素值
///
/// # 参数
/// - `out`: 输出流引用
/// - `pixel_color`: 颜色向量（[0,1]范围内的RGB值）
pub fn write_color<W: std::io::Write>(out: &mut W, pixel_color: &Color) {
    let mut r = pixel_color.x();
    let mut g = pixel_color.y();
    let mut b = pixel_color.z();

    if r.is_nan() {
        r = 0.0;
    }
    if g.is_nan() {
        g = 0.0;
    }
    if b.is_nan() {
        b = 0.0;
    }

    // let r = linear_to_gamma(r);
    // let g = linear_to_gamma(g);
    // let b = linear_to_gamma(b);

    r = linear_to_gamma(r);
    g = linear_to_gamma(g);
    b = linear_to_gamma(b);

    // 将[0,1]范围转换为[0,255]字节范围
    let intensity = Interval::new(0.000, 0.999);
    let r_clamped = intensity.clamp(r);
    let g_clamped = intensity.clamp(g);
    let b_clamped = intensity.clamp(b);

    // 将[0,1)范围映射到[0,255]的整数
    let rbyte = (256.0 * r_clamped) as i32;
    let gbyte = (256.0 * g_clamped) as i32;
    let bbyte = (256.0 * b_clamped) as i32;

    // 写入像素颜色分量
    writeln!(out, "{} {} {}", rbyte, gbyte, bbyte).unwrap();
}

/// 返回颜色像素值的字符串
pub fn write_color_to_string(pixel_color: &Color) -> String {
    let r = pixel_color.x();
    let g = pixel_color.y();
    let b = pixel_color.z();

    let r = linear_to_gamma(r);
    let g = linear_to_gamma(g);
    let b = linear_to_gamma(b);

    // 将[0,1]范围转换为[0,255]字节范围
    let intensity = Interval::new(0.000, 0.999);
    let r_clamped = intensity.clamp(r);
    let g_clamped = intensity.clamp(g);
    let b_clamped = intensity.clamp(b);

    // 将[0,1)范围映射到[0,255]的整数
    let rbyte = (256.0 * r_clamped) as i32;
    let gbyte = (256.0 * g_clamped) as i32;
    let bbyte = (256.0 * b_clamped) as i32;

    // 返回格式化的字符串
    format!("{} {} {}\n", rbyte, gbyte, bbyte)
}
