use image::{DynamicImage, io::Reader};
use std::{
    env,
    ffi::OsStr,
    path::{Path, PathBuf},
};

/// 模拟 C++ 的 rtw_image 类
#[derive(Debug)]
pub struct RtwImage {
    float_data: Option<Vec<f32>>,
    byte_data: Option<Vec<u8>>,
    width: u32,
    height: u32,
    bytes_per_scanline: usize,
}

impl RtwImage {
    const BYTES_PER_PIXEL: usize = 3;
    const MAGENTA: [u8; 3] = [255, 0, 255];

    /// 创建一个新的 RtwImage 实例
    pub fn new(image_filename: &str) -> Self {
        let mut image = Self {
            float_data: None,
            byte_data: None,
            width: 0,
            height: 0,
            bytes_per_scanline: 0,
        };

        // 尝试在多个位置查找图像文件
        if image.try_load_in_locations(image_filename) {
            return image;
        }

        eprintln!("ERROR: Could not load image file '{}'", image_filename);
        image
    }

    /// 尝试在多个可能的位置加载图像
    fn try_load_in_locations(&mut self, filename: &str) -> bool {
        let mut paths = Vec::new();

        // 添加环境变量指定的路径
        if let Some(imagedir) = env::var_os("RTW_IMAGES") {
            paths.push(PathBuf::from(imagedir).join(filename));
        }

        // 添加其他可能的位置
        paths.push(PathBuf::from(filename));
        paths.push(PathBuf::from("images").join(filename));
        paths.push(PathBuf::from("../images").join(filename));
        paths.push(PathBuf::from("../../images").join(filename));
        paths.push(PathBuf::from("../../../images").join(filename));
        paths.push(PathBuf::from("../../../../images").join(filename));
        paths.push(PathBuf::from("../../../../../images").join(filename));
        paths.push(PathBuf::from("../../../../../../images").join(filename));

        for path in paths {
            if self.load(&path) {
                return true;
            }
        }

        false
    }

    /// 加载图像文件
    fn load(&mut self, filename: &Path) -> bool {
        // 检查文件扩展名以确定是否可能是 HDR 图像
        let is_hdr = filename
            .extension()
            .and_then(OsStr::to_str)
            .map(|ext| ext.eq_ignore_ascii_case("hdr"))
            .unwrap_or(false);

        if is_hdr {
            // 处理 HDR 图像
            // if let Ok(img) = Reader::open(filename)
            //     .and_then(|reader| reader.with_guessed_format())
            //     .and_then(|reader| reader.decode())
            // {
            //     self.process_image(img);
            //     return true;
            // }
            if let Ok(reader) = Reader::open(filename).and_then(|r| r.with_guessed_format()) {
                if let Ok(img) = reader.decode() {
                    self.process_image(img);
                    return true;
                }
            }
        } else {
            // 处理 LDR 图像
            if let Ok(img) = image::open(filename) {
                self.process_image(img);
                return true;
            }
        }

        false
    }

    /// 处理加载的图像
    fn process_image(&mut self, img: DynamicImage) {
        // 转换为 RGB 格式
        let rgb_img = img.to_rgb32f();
        self.width = rgb_img.width();
        self.height = rgb_img.height();
        self.bytes_per_scanline = (self.width as usize) * Self::BYTES_PER_PIXEL;

        // 提取浮点数据
        self.float_data = Some(rgb_img.pixels().flat_map(|p| [p[0], p[1], p[2]]).collect());

        // 生成字节数据
        self.convert_to_bytes();
    }

    /// 将浮点数据转换为字节数据
    fn convert_to_bytes(&mut self) {
        if let Some(float_data) = &self.float_data {
            let total_bytes = (self.width * self.height) as usize * Self::BYTES_PER_PIXEL;
            let mut byte_data = Vec::with_capacity(total_bytes);

            for &value in float_data {
                byte_data.push(Self::float_to_byte(value));
            }

            self.byte_data = Some(byte_data);
        }
    }

    /// 将浮点值转换为字节
    fn float_to_byte(value: f32) -> u8 {
        if value <= 0.0 {
            0
        } else if value >= 1.0 {
            255
        } else {
            (value * 255.0) as u8
        }
    }

    /// 获取图像宽度
    pub fn width(&self) -> u32 {
        self.width
    }

    /// 获取图像高度
    pub fn height(&self) -> u32 {
        self.height
    }

    /// 获取像素数据
    pub fn pixel_data(&self, x: u32, y: u32) -> &[u8] {
        if self.byte_data.is_none() {
            return &Self::MAGENTA;
        }

        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        let index =
            (y as usize * self.bytes_per_scanline + x as usize * Self::BYTES_PER_PIXEL) as usize;

        if let Some(byte_data) = &self.byte_data {
            &byte_data[index..index + 3]
        } else {
            &Self::MAGENTA
        }
    }

    /// 获取浮点像素数据（可选功能）
    pub fn float_pixel_data(&self, x: u32, y: u32) -> Option<[f32; 3]> {
        if self.float_data.is_none() {
            return None;
        }

        let x = x.min(self.width - 1);
        let y = y.min(self.height - 1);

        let index = (y as usize * self.width as usize + x as usize) * 3;

        if let Some(float_data) = &self.float_data {
            Some([
                float_data[index],
                float_data[index + 1],
                float_data[index + 2],
            ])
        } else {
            None
        }
    }
}

// Gamma 校正函数
fn srgb_to_linear(c: f32) -> f32 {
    if c <= 0.04045 {
        c / 12.92
    } else {
        ((c + 0.055) / 1.055).powf(2.4)
    }
}
