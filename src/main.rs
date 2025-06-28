pub mod camera;
pub mod color;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod ray;
pub mod rtweekend;
pub mod sphere;
pub mod vec3;

use crate::camera::Camera;
use crate::color::Color;
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, Lambertian, Material, Metal};
use crate::rtweekend::{PI, random_double, random_double_range};
use crate::sphere::Sphere;
use crate::vec3::{Point3, Vec3};
use std::sync::{Arc, Mutex};

// 此代码可以跑出第一本书的最终场景
fn main() {
    let mut world = HittableList::new();

    // let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    // let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // // let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3)); // 金属材质
    // let material_left = Arc::new(Dielectric::new(1.50)); // 玻璃材质
    // let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50)); // 玻璃球内部气泡
    // // let material_left = Arc::new(Dielectric::new(1.00 / 1.33)); // 气泡（内全反射）
    // let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)); // 模糊金属

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, -100.5, -1.0),
    //     100.0,
    //     material_ground,
    // )));

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, 0.0, -1.2),
    //     0.5,
    //     material_center,
    // )));

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.5,
    //     material_left,
    // )));

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(-1.0, 0.0, -1.0),
    //     0.4,
    //     material_bubble,
    // )));

    // world.add(Arc::new(Sphere::new(
    //     Point3::new(1.0, 0.0, -1.0),
    //     0.5,
    //     material_right,
    // )));

    // let mut cam = Camera::new();
    // cam.aspect_ratio = 16.0 / 9.0;
    // cam.image_width = 400;
    // cam.samples_per_pixel = 100;
    // cam.max_depth = 50;

    // cam.vfov = 20.0;
    // cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    // cam.lookat = Point3::new(0.0, 0.0, -1.0);
    // cam.vup = Vec3::new(0.0, 1.0, 0.0);

    // cam.defocus_angle = 10.0;
    // cam.focus_dist = 3.4;

    let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        ground_material,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random_double();
            let center = Point3::new(
                a as f64 + 0.9 * random_double(),
                0.2,
                b as f64 + 0.9 * random_double(),
            );

            if (center - Point3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                let sphere_material: Arc<dyn Material>;

                if choose_mat < 0.8 {
                    let albedo = Color::random() * Color::random();
                    sphere_material = Arc::new(Lambertian::new(albedo));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                }

                world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
            }
        }
    }

    let material1 = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        material1,
    )));

    let material2 = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Arc::new(Sphere::new(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        material2,
    )));

    let material3 = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Arc::new(Sphere::new(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        material3,
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    cam.render(&world);
}
