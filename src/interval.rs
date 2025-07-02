use std::{f64, ops::Add};

/// 表示一个实数区间 [min, max]
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Interval {
    pub min: f64,
    pub max: f64,
}

impl Interval {
    // 静态常量定义
    pub const EMPTY: Interval = Interval {
        min: f64::INFINITY,
        max: f64::NEG_INFINITY,
    };

    pub const UNIVERSE: Interval = Interval {
        min: f64::NEG_INFINITY,
        max: f64::INFINITY,
    };

    /// 创建一个空区间 [+∞, -∞]
    pub fn new_empty() -> Self {
        Self {
            min: f64::INFINITY,
            max: f64::NEG_INFINITY,
        }
    }

    /// 创建一个包含整个实数域的区间 [-∞, +∞]
    pub fn new_universe() -> Self {
        Self {
            min: f64::NEG_INFINITY,
            max: f64::INFINITY,
        }
    }

    /// 创建一个指定上下界的区间
    pub fn new(min: f64, max: f64) -> Self {
        Self { min, max }
    }

    pub fn from_intervals(a: &Interval, b: &Interval) -> Self {
        Self {
            min: a.min.min(b.min),
            max: a.max.max(b.max),
        }
    }

    /// 计算区间的大小（长度）
    pub fn size(&self) -> f64 {
        self.max - self.min
    }

    /// 判断值是否在区间内（包括边界）
    pub fn contains(&self, x: f64) -> bool {
        self.min <= x && x <= self.max
    }

    /// 判断值是否在区间内（不包括边界）
    pub fn surrounds(&self, x: f64) -> bool {
        self.min < x && x < self.max
    }

    pub fn clamp(&self, x: f64) -> f64 {
        if x < self.min {
            return self.min;
        }
        if x > self.max {
            return self.max;
        }
        return x;
    }

    pub fn expand(&self, delta: f64) -> Self {
        let padding = delta / 2.0;
        Self {
            min: self.min - padding,
            max: self.max + padding,
        }
    }

    pub fn add_dispplacement(&self, displacement: f64) -> Self {
        Self {
            min: self.min + displacement,
            max: self.max + displacement,
        }
    }
}

impl Add<f64> for Interval {
    type Output = Self;

    fn add(self, displacement: f64) -> Self::Output {
        self.add_dispplacement(displacement)
    }
}

impl Add<Interval> for f64 {
    type Output = Interval;

    fn add(self, interval: Interval) -> Self::Output {
        interval.add_dispplacement(self)
    }
}
