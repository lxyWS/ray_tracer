use crate::color::{Color, write_color_to_string};
use crate::hittable::Hittable;
use crate::interval::Interval;
use crate::material::ScatterRecord;
use crate::pdf::{CosinePdf, HittablePdf, MixturePdf, Pdf};
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
    pub background: Color,
    pub vfov: f64,          // Vertical view angle (field of view)
    pub lookfrom: Point3,   // Point camera is looking from
    pub lookat: Point3,     // Point camera is looking at
    pub vup: Vec3,          // Camera-relative "up" direction
    pub defocus_angle: f64, // Variation angle of rays through each pixel
    pub focus_dist: f64,    // Distance from camera lookfrom point to plane of perfect focus

    // 私有成员
    image_height: i32,
    pixel_samples_scale: f64, // 像素采样的缩放因子
    sqrt_spp: i32,            // 样本数的平方根（分层采样时使用）
    recip_sqrt_spp: f64,      //  1/sqrt_spp（分层采样时使用）
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
            background: Color::new(0.0, 0.0, 0.0), // 默认黑色背景
            vfov: 90.0,
            lookfrom: Point3::new(0.0, 0.0, 0.0),
            lookat: Point3::new(0.0, 0.0, -1.0),
            vup: Point3::new(0.0, 1.0, 0.0),
            defocus_angle: 0.0,
            focus_dist: 10.0,
            image_height: 0,
            pixel_samples_scale: 0.0,
            sqrt_spp: 0,
            recip_sqrt_spp: 0.0,
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
    // pub fn render(&self, world: &impl Hittable, lights: &impl Hittable) {
    pub fn render(
        &self,
        world: Arc<dyn Hittable + Send + Sync>,
        lights: Arc<dyn Hittable + Send + Sync>,
    ) {
        let mut camera = self.clone();
        camera.initialize();

        println!("P3\n{} {}\n255", camera.image_width, camera.image_height);

        let camera_arc = Arc::new(camera);

        // 收集输出的互斥锁
        let outputs: Mutex<Vec<String>> =
            Mutex::new(vec![String::new(); camera_arc.image_height as usize]);
        let progress_counter = Arc::new(Mutex::new(0));

        // 修改前
        // let world_arc = Arc::new(world);
        // let lights_arc = Arc::new(lights);
        // 每个线程处理图像的一行
        (0..camera_arc.image_height).into_par_iter().for_each(|j| {
            let mut line_output = String::new();

            for i in 0..camera_arc.image_width {
                let mut pixel_color = Color::new(0.0, 0.0, 0.0);

                // for _ in 0..camera_arc.samples_per_pixel {
                //     let r = camera_arc.get_ray(i, j);
                //     pixel_color += camera_arc.ray_color(&r, camera_arc.max_depth, *world_arc);
                // }

                for s_j in 0..camera_arc.sqrt_spp {
                    for s_i in 0..camera_arc.sqrt_spp {
                        let r = camera_arc.get_ray(i, j, s_i, s_j);
                        // pixel_color += camera_arc.ray_color(&r, camera_arc.max_depth, &**world_arc);

                        // 修改ray_color之前的代码
                        // pixel_color += camera_arc.ray_color(
                        //     &r,
                        //     camera_arc.max_depth,
                        //     &**world_arc,
                        //     &**lights_arc,
                        // )

                        // 修改ray_color之后
                        pixel_color += camera_arc.ray_color(
                            &r,
                            camera_arc.max_depth,
                            Arc::clone(&world),
                            Arc::clone(&lights),
                        )
                    }
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

        self.sqrt_spp = (self.samples_per_pixel as f64).sqrt() as i32;
        self.pixel_samples_scale = 1.0 / ((self.sqrt_spp * self.sqrt_spp) as f64);
        self.recip_sqrt_spp = 1.0 / (self.sqrt_spp as f64);

        // self.pixel_samples_scale = 1.0 / (self.samples_per_pixel as f64);

        // 设置相机中心
        self.center = self.lookfrom;

        // 计算视口尺寸
        let theta = degrees_to_radians(self.vfov);
        let h = (theta / 2.0).tan();
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
        let viewport_upper_left =
            self.center - (self.focus_dist * self.w) - viewport_u / 2.0 - viewport_v / 2.0;
        self.pixel00_loc = viewport_upper_left + 0.5 * (self.pixel_delta_u + self.pixel_delta_v);

        let defocus_radius = self.focus_dist * (degrees_to_radians(self.defocus_angle / 2.0).tan());
        self.defocus_disk_u = self.u * defocus_radius;
        self.defocus_disk_v = self.v * defocus_radius;
    }

    fn get_ray(&self, i: i32, j: i32, s_i: i32, s_j: i32) -> Ray {
        // 在像素区域内随机采样
        let offset = self.sample_square_stratified(s_i, s_j);
        // let offset = self.sample_square();
        let pixel_sample = self.pixel00_loc
            + ((i as f64 + offset.x()) * self.pixel_delta_u)
            + ((j as f64 + offset.y()) * self.pixel_delta_v);

        // 构建射线
        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample()
        };
        let ray_direction = pixel_sample - ray_origin;
        let ray_time = random_double();

        Ray::with_origin_dir_time(ray_origin, ray_direction, ray_time)
    }

    fn sample_square_stratified(&self, s_i: i32, s_j: i32) -> Vec3 {
        let px = ((s_i as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        let py = ((s_j as f64 + random_double()) * self.recip_sqrt_spp) - 0.5;
        Vec3::new(px, py, 0.0)
    }

    fn sample_square(&self) -> Vec3 {
        Vec3::new(random_double() - 0.5, random_double() - 0.5, 0.0)
    }

    fn defocus_disk_sample(&self) -> Point3 {
        let p = random_in_unit_disk();
        self.center + (p.x() * self.defocus_disk_u) + (p.y() * self.defocus_disk_v)
    }

    /// 计算射线与场景交互后的颜色
    fn ray_color(
        &self,
        r: &Ray,
        depth: i32,
        // world: &impl Hittable,
        // lights: &impl Hittable,
        world: Arc<dyn Hittable + Send + Sync>,
        lights: Arc<dyn Hittable + Send + Sync>,
    ) -> Color {
        if depth <= 0 {
            return Color::new(0.0, 0.0, 0.0);
        }

        let mut rec = crate::hittable::HitRecord::default();

        if !world.hit(r, Interval::new(0.001, INFINITY), &mut rec) {
            return self.background;
        }

        let mut srec = ScatterRecord::default();

        let color_from_emission = rec.mat.as_ref().map_or(Color::default(), |mat| {
            mat.emitted(r, &rec, rec.u, rec.v, &rec.p)
        });

        if rec
            .mat
            .as_ref()
            .map_or(false, |mat| !mat.scatter(r, &rec, &mut srec))
        {
            return color_from_emission;
        }

        if srec.skip_pdf {
            let scattered = srec.skip_pdf_ray.clone().unwrap();
            let scattering_pdf = rec
                .mat
                .as_ref()
                .map_or(0.0, |mat| mat.scattering_pdf(r, &rec, &scattered));

            let color_from_scatter = {
                srec.attenuation
                    * self.ray_color(
                        &scattered,
                        depth - 1,
                        Arc::clone(&world),
                        Arc::clone(&lights),
                    )
            };

            // return color_from_emission + color_from_scatter;
            return color_from_scatter;
        }

        let light_pdf = HittablePdf::new(Arc::clone(&lights), rec.p);
        let light_pdf_arc = Arc::new(light_pdf);
        let mat_pdf_arc = srec.pdf_ptr.clone().unwrap();
        // let mat_pdf = srec.pdf_ptr.clone().unwrap();

        let mixed_pdf = MixturePdf::new(light_pdf_arc, mat_pdf_arc);

        let scattered_dir = mixed_pdf.generate();
        let scattered = Ray::with_origin_dir_time(rec.p, scattered_dir, r.time());
        let pdf_value = mixed_pdf.value(scattered.direction());

        let scattering_pdf = rec
            .mat
            .as_ref()
            .map_or(0.0, |mat| mat.scattering_pdf(r, &rec, &scattered));

        // let color_from_scatter = if scattering_pdf > 1e-8 {
        //     (srec.attenuation
        //         * scattering_pdf
        //         * self.ray_color(
        //             &scattered,
        //             depth - 1,
        //             Arc::clone(&world),
        //             Arc::clone(&lights),
        //         ))
        //         / pdf_value
        // } else {
        //     Color::new(0.0, 0.0, 0.0)
        // };
        let color_from_scatter = {
            (srec.attenuation
                * scattering_pdf
                * self.ray_color(
                    &scattered,
                    depth - 1,
                    Arc::clone(&world),
                    Arc::clone(&lights),
                ))
                / pdf_value
        };

        color_from_emission + color_from_scatter

        // // let mut scattered = Ray::new();
        // // let mut attenuation = Color::default();
        // // let mut pdf_value = 0.0;

        // if rec.mat.as_ref().map_or(true, |mat| {
        //     !mat.scatter(r, &rec, &mut attenuation, &mut scattered, &mut pdf_value)
        // }) {
        //     return color_from_emission;
        // }

        // // let p0 = Arc::new(HittablePdf::new(lights, rec.p));
        // // let p1 = Arc::new(CosinePdf::new(rec.normal));
        // // let mixed_pdf = MixturePdf::new(p0, p1);
        // let p0 = Box::new(HittablePdf::new(lights, rec.p));
        // let p1 = Box::new(CosinePdf::new(rec.normal));
        // let mixed_pdf = MixturePdf::new(p0, p1);

        // scattered = Ray::with_origin_dir_time(rec.p, mixed_pdf.generate(), r.time());
        // pdf_value = mixed_pdf.value(scattered.direction());

        // // let light_pdf = HittablePdf::new(lights, rec.p);
        // // scattered = Ray::with_origin_dir_time(rec.p, light_pdf.generate(), r.time());
        // // pdf_value = light_pdf.value(scattered.direction());

        // // // 光源上随机选点
        // // let on_light = Point3::new(
        // //     random_double_range(213.0, 343.0),
        // //     554.0,
        // //     random_double_range(227.0, 332.0),
        // // );
        // // let to_light = on_light - rec.p;
        // // let distance_squared = to_light.length_squared();
        // // let to_light_dir = unit_vector(to_light);

        // // if dot(&to_light_dir, &rec.normal) < 0.0 {
        // //     return color_from_emission;
        // // }

        // // let light_area = (343.0 - 213.0) * (332.0 - 227.0);
        // // let light_cosine = to_light_dir.y().abs();

        // // if light_cosine < 0.000001 {
        // //     return color_from_emission;
        // // }

        // // pdf_value = distance_squared / (light_cosine * light_area);

        // // scattered = Ray::with_origin_dir_time(rec.p, to_light_dir, r.time());

        // // // cosine PDF
        // // let surface_pdf = CosinePdf::new(rec.normal);
        // // scattered = Ray::with_origin_dir_time(rec.p, surface_pdf.generate(), r.time());
        // // pdf_value = surface_pdf.value(scattered.direction());

        // let scattering_pdf = rec
        //     .mat
        //     .as_ref()
        //     .map_or(0.0, |mat| mat.scattering_pdf(r, &rec, &scattered));

        // // let scattering_pdf = rec
        // //     .mat
        // //     .as_ref()
        // //     .map_or(0.0, |mat| mat.scattering_pdf(r, &rec, &scattered));

        // // pdf_value = scattering_pdf;
        // // let pdf_value = 1.0 / (2.0 * PI);

        // // // RR终止策略
        // // let p = attenuation
        // //     .x()
        // //     .max(attenuation.y())
        // //     .max(attenuation.z())
        // //     .clamp(0.1, 1.0);

        // // if random_double() >= p {
        // //     return color_from_emission;
        // // }

        // let color_from_scatter = if scattering_pdf > 1e-8 {
        //     (attenuation * scattering_pdf * self.ray_color(&scattered, depth - 1, world, lights))
        //         / pdf_value
        // } else {
        //     Color::new(0.0, 0.0, 0.0)
        // };

        // // let color_from_scatter = attenuation * self.ray_color(&scattered, depth - 1, world);

        // color_from_emission + color_from_scatter
    }
}
