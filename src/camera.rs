use crate::color::{Color, write_color_to_string};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::ray::Ray;
use crate::rtweekend::{INFINITY, degrees_to_radians, random_double};
use crate::vec3::{Point3, Vec3, cross, random_in_unit_disk, unit_vector};
use rayon::prelude::*;
use std::io::{self, Write};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
/// 相机类，负责生成射线并渲染场景
pub struct Camera {
    // 相机参数（公开）
    pub aspect_ratio: f64,
    pub image_width: i32,
    pub samples_per_pixel: i32, // count of random samples for each pixel
    pub max_depth: i32,         // Maximum number of ray bounces into scene
    pub vfov: f64,              // Vertical view angle (field of view)
    pub lookfrom: Point3,       // Point camera is looking from
    pub lookat: Point3,         // Point camera is looking at
    pub vup: Vec3,              // Camera-relative "up" direction
    pub defocus_angle: f64,     // Variation angle of rays through each pixel
    pub focus_dist: f64,        // Distance from camera lookfrom point to plane of perfect focus

    // 私有成员
    image_height: i32,
    pixel_samples_scale: f64, // 像素采样的缩放因子
    center: Point3,
    pixel00_loc: Point3,
    pixel_delta_u: Vec3,
    pixel_delta_v: Vec3,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    defocus_disk_u: Vec3, // Defocus disk horizontal radius
    defocus_disk_v: Vec3, // Defocus disk vertical radius
}

impl Camera {
    /// 创建新相机
    pub fn new() -> Self {
        Self {
            aspect_ratio: 1.0,
            image_width: 100,
            samples_per_pixel: 10,
            max_depth: 10,
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Point3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_samples_scale: 0.0,
            center: Point3::default(),
            pixel00_loc: Point3::default(),
            pixel_delta_u: Vec3::default(),
            pixel_delta_v: Vec3::default(),
            u: Vec3::default(),
            v: Vec3::default(),
            w: Vec3::default(),
            defocus_disk_u: Vec3::default(),
            defocus_disk_v: Vec3::default(),
        }
    }

    /// 渲染给定场景
    pub fn render(&self, world: &(impl Hittable + Sync)) {
        let mut camera = self.clone();
        camera.initialize();

        println!("P3\n{} {}\n255", camera.image_width, camera.image_height);

        // // 逐像素渲染
        // for j in 0..camera.image_height {
        //     // 显示进度
        //     eprint!("\rScanlines remaining: {}", camera.image_height - j);
        //     io::stderr().flush().unwrap();

        //     for i in 0..camera.image_width {
        //         let mut pixel_color = Color::new(0.0, 0.0, 0.0);

        //         // 对每个像素多次采样
        //         for _ in 0..camera.samples_per_pixel {
        //             let r = camera.get_ray(i, j);
        //             // pixel_color += self.ray_color(&r, world);
        //             pixel_color += self.ray_color(&r, self.max_depth, world);
        //         }

        //         write_color(
        //             &mut io::stdout(),
        //             &(camera.pixel_samples_scale * pixel_color),
        //         );
        //     }
        // }

        let camera_arc = Arc::new(camera);

        // 收集输出的互斥锁
        let outputs: Mutex<Vec<String>> =
            Mutex::new(vec![String::new(); camera_arc.image_height as usize]);
        let progress_counter = Arc::new(Mutex::new(0));

        let world_arc = Arc::new(world);

        // 每个线程处理图像的一行
        (0..camera_arc.image_height).into_par_iter().for_each(|j| {
            let mut line_output = String::new();

            for i in 0..camera_arc.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                for _ in 0..camera_arc.samples_per_pixel {
                    let r = camera_arc.get_ray(i, j);
                    pixel_color += camera_arc.ray_color(&r, camera_arc.max_depth, *world_arc);
                }

                let color_str =
                    write_color_to_string(&(camera_arc.pixel_samples_scale * pixel_color));
                line_output.push_str(&color_str);
            }

            // 更新进度
            let mut progress = progress_counter.lock().unwrap();
            *progress += 1;
            eprint!(
                "\r渲染进度: {:.1}%",
                (*progress) as f64 / camera_arc.image_height as f64 * 100.0
            );
            io::stderr().flush().unwrap();

            let mut outputs = outputs.lock().unwrap();
            outputs[j as usize] = line_output;
        });

        let outputs = outputs.into_inner().unwrap();
        for output in outputs {
            print!("{}", output);
        }

        eprint!("\rDone.                 \n");
        io::stderr().flush().unwrap();
    }

    /// 初始化相机内部参数
    fn initialize(&mut self) {
        // 计算图像高度
        self.image_height = (self.image_width as f64 / self.aspect_ratio) as i32;
        self.image_height = if self.image_height < 1 {
            1
        } else {
            self.image_height
        };

        self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        // 设置相机中心
        self.center = self.lookfrom;

        // 计算视口尺寸
        // let focal_length = (self.lookfrom - self.lookat).length();
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
        // let viewport_height = 2.0 * h * focal_length;
        let viewport_height = 2.0 * h * self.focus_dist;
        let viewport_width = viewport_height * (self.image_width as f64 / self.image_height as f64);

        self.w = unit_vector(self.lookfrom - self.lookat);
        self.u = unit_vector(cross(&self.vup, &self.w));
        self.v = cross(&self.w, &self.u);

        // 计算视口边缘向量
        let viewport_u = viewport_width * self.u;
        let viewport_v = -viewport_height * self.v;

        // 计算像素间的增量向量
        self.pixel_delta_u = viewport_u / (self.image_width as f64);
        self.pixel_delta_v = viewport_v / (self.image_height as f64);

        // 计算视口左上角位置
        // let viewport_upper_left =
        //     self.center - (focal_length * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0).tan());
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32) -> Ray {
        // 在像素区域内随机采样
        let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // 构建射线
        // let ray_origin = self.center;
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::with_origin_dir_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square(&self) -> Vec3 {
        // Returns the vector to a random point in the [-.5,-.5]-[+.5,+.5] unit square.
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    /// 计算射线与场景交互后的颜色
    fn ray_color(&self, r: &Ray, depth: i32, world: &impl Hittable) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = crate::hittable::HitRecord::default();

        if world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            // let direction = random_on_hemisphere(&rec.normal);
            // let direction = rec.normal + random_unit_vector();
            // 0.5 * self.ray_color(&Ray::with_origin_dir(rec.p, direction), depth - 1, world)
            let mut scattered = Ray::new();
            let mut attenuation = Color::default();
            if rec
                .mat
                .as_ref()
                .and_then(|mat| Some(mat.scatter(r, &rec, &mut attenuation, &mut scattered)))
                .unwrap_or(false)
            {
                return attenuation * self.ray_color(&scattered, depth - 1, world);
            }

            Color::default()
        } else {
            let unit_direction = unit_vector(*r.direction());
            let a = 0.5 * (unit_direction.y() + 1.0);
            (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
        }
    }
}
