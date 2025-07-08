pub mod aabb;
pub mod bvh;
pub mod camera;
pub mod color;
pub mod constant_medium;
pub mod hittable;
pub mod hittable_list;
pub mod interval;
pub mod material;
pub mod mesh;
pub mod obj_loader;
pub mod onb;
pub mod pdf;
pub mod perlin;
pub mod quad;
pub mod ray;
pub mod rtw_stb_image;
pub mod rtweekend;
pub mod sphere;
pub mod texture;
pub mod triangle;
pub mod vec3;

use crate::bvh::BvhNode;
use crate::camera::Camera;
use crate::color::Color;
use crate::hittable::{Hittable, RotateY, Translate};
use crate::hittable_list::HittableList;
use crate::material::{Dielectric, DiffuseLight, Lambertian, Material, Metal};
use crate::mesh::Mesh;
use crate::quad::{Quad, box_new};
use crate::rtweekend::{random_double, random_double_range};
use crate::sphere::Sphere;
use crate::texture::{CheckerTexture, NoiseTexture, SolidColor};
use crate::triangle::Triangle;
use crate::vec3::{Point3, Vec3};
use obj_loader::ObjModel;
use std::sync::Arc;
use std::time::Instant;

fn for_output13() {
    let mut world = HittableList::new();

    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    // let material_left = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.8), 0.3)); // 金属材质
    let material_left = Arc::new(Dielectric::new(1.50)); // 玻璃材质
    let material_bubble = Arc::new(Dielectric::new(1.00 / 1.50)); // 玻璃球内部气泡
    // let material_left = Arc::new(Dielectric::new(1.00 / 1.33)); // 气泡（内全反射）
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0)); // 模糊金属

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(-1.0, 0.0, -1.0),
        0.4,
        material_bubble,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(-2.0, 2.0, 1.0);
    cam.lookat = Point3::new(0.0, 0.0, -1.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 10.0;
    cam.focus_dist = 3.4;

    // cam.render(&world);
}

fn last_picture_the_first_book() {
    let mut world = HittableList::new();

    // let ground_material = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    // world.add(Arc::new(Sphere::new(
    //     Point3::new(0.0, -1000.0, 0.0),
    //     1000.0,
    //     ground_material,
    // )));

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let ground_material = Arc::new(Lambertian::from_texture(checker));
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
                    let center2 = center + Vec3::new(0.0, random_double_range(0.0, 0.5), 0.0);
                    world.add(Arc::new(Sphere::new_moving(
                        center,
                        center2,
                        0.2,
                        sphere_material,
                    )));
                    // world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else if choose_mat < 0.95 {
                    let albedo = Color::random_range(0.5, 1.0);
                    let fuzz = random_double_range(0.0, 0.5);
                    sphere_material = Arc::new(Metal::new(albedo, fuzz));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                } else {
                    sphere_material = Arc::new(Dielectric::new(1.5));
                    world.add(Arc::new(Sphere::new(center, 0.2, sphere_material)));
                }
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

    let bvh_node = BvhNode::new(&world);
    let mut world_bvh = HittableList::new();
    world_bvh.add(bvh_node);

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    // cam.image_width = 1200; // 加上运动效果之前
    // cam.samples_per_pixel = 500; // 加上运动效果之前
    cam.image_width = 400; // 加上运动效果之后
    cam.samples_per_pixel = 100; // 加上运动效果之后
    // cam.max_depth = 50;
    cam.max_depth = 20;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.6;
    cam.focus_dist = 10.0;

    // cam.render(&world);
    // cam.render(&world_bvh);
}

fn checkered_spheres() {
    let mut world = HittableList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let checker_material = Arc::new(Lambertian::from_texture(checker.clone()));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -10.0, 0.0),
        10.0,
        checker_material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 10.0, 0.0),
        10.0,
        checker_material,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn earth() {
    let earth_texture = Arc::new(texture::ImageTexture::new("earthmap.jpg"));
    let earth_surface = Arc::new(Lambertian::from_texture(earth_texture));
    let globe = Arc::new(Sphere::new(Point3::new(0.0, 0.0, 0.0), 2.0, earth_surface));

    let mut world = HittableList::new();
    world.add(globe);

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 12.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn perlin_spheres() {
    let mut world = HittableList::new();

    let perlin_texture = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture.clone()));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(13.0, 2.0, 3.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn quads() {
    let mut world = HittableList::new();

    let left_red = Arc::new(Lambertian::new(Color::new(1.0, 0.2, 0.2)));
    let back_green = Arc::new(Lambertian::new(Color::new(0.2, 1.0, 0.2)));
    let right_blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    let upper_orange = Arc::new(Lambertian::new(Color::new(1.0, 0.5, 0.0)));
    let lower_teal = Arc::new(Lambertian::new(Color::new(0.2, 0.8, 0.8)));

    world.add(Arc::new(Quad::new(
        Point3::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    let bvh_node = BvhNode::new(&world);
    let mut world_bvh = HittableList::new();
    world_bvh.add(bvh_node);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.70, 0.80, 1.00);

    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    //cam.render(&world);
    // cam.render(&world_bvh);
}

fn simple_light() {
    let mut world = HittableList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));
    let perlin_material = Arc::new(Lambertian::from_texture(pertext.clone()));

    // 地面球体
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, -1000.0, 0.0),
        1000.0,
        perlin_material.clone(),
    )));

    // 中央球体
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 2.0, 0.0),
        2.0,
        perlin_material,
    )));

    let difflight = Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 7.0, 0.0),
        2.0,
        difflight.clone(),
    )));

    world.add(Arc::new(Quad::new(
        Point3::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight,
    )));

    let mut cam = Camera::new();

    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0); // 黑色背景

    cam.vfov = 20.0;
    cam.lookfrom = Point3::new(26.0, 3.0, 6.0);
    cam.lookat = Point3::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn cornell_box() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    // 左侧墙（红色）
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    // 地板（白色）
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    // 天花板（白色）
    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    // 后墙（白色）
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));
    // 光源（天花板上的灯）
    world.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light.clone(),
    )));

    // let aluminum = Arc::new(Metal::new(Color::new(0.80, 0.85, 0.88), 0.0));
    // let mut box1: Arc<dyn Hittable + Send + Sync> = box_new(
    //     Point3::new(0.0, 0.0, 0.0),
    //     Point3::new(165.0, 330.0, 165.0),
    //     aluminum,
    // );

    // // 修改前
    let mut box1: Arc<dyn Hittable + Send + Sync> = box_new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    let glass = Arc::new(Dielectric::new(1.5));
    world.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        glass,
    )));

    // let mut box2: Arc<dyn Hittable + Send + Sync> = box_new(
    //     Point3::new(0.0, 0.0, 0.0),
    //     Point3::new(165.0, 165.0, 165.0),
    //     white.clone(),
    // );
    // box2 = Arc::new(RotateY::new(box2, -18.0));
    // box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    // world.add(box2);

    // let empty_material = Arc::new(Material);
    let empty_material = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    // let empty_material = Arc::new(Lambertian::new(Color::new(0.0, 0.0, 0.0))); // 黑色材质，不发
    let mut lights = HittableList::new();
    lights.add(Arc::new(Quad::new(
        Point3::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        empty_material.clone(),
    )));
    lights.add(Arc::new(Sphere::new(
        Point3::new(190.0, 90.0, 190.0),
        90.0,
        empty_material,
    )));

    // let lights = Quad::new(
    //     Point3::new(343.0, 554.0, 332.0),
    //     Vec3::new(-130.0, 0.0, 0.0),
    //     Vec3::new(0.0, 0.0, -105.0),
    //     empty_material,
    // );

    // let bvh_node = BvhNode::new(&world);
    // let mut world_bvh = HittableList::new();
    // world_bvh.add(bvh_node);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    // cam.samples_per_pixel = 200; // 2nd book
    cam.samples_per_pixel = 1000; // 3rd book
    // cam.samples_per_pixel = 10;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0); // 黑色背景

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world);
    // cam.render(&world_bvh);
    cam.render(Arc::new(world), Arc::new(lights));
}

fn cornell_smoke() {
    let mut world = HittableList::new();

    let red = Arc::new(Lambertian::new(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    world.add(Arc::new(Quad::new(
        Point3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(113.0, 554.0, 127.0),
        Vec3::new(333.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    let mut box1: Arc<dyn Hittable + Send + Sync> = box_new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    box1 = Arc::new(RotateY::new(box1, 15.0));
    box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));

    let mut box2: Arc<dyn Hittable + Send + Sync> = box_new(
        Point3::new(0.0, 0.0, 0.0),
        Point3::new(165.0, 165.0, 165.0),
        white.clone(),
    );
    box2 = Arc::new(RotateY::new(box2, -18.0));
    box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));

    let smoke1 = Arc::new(constant_medium::ConstantMedium::new_with_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0), // 黑色烟雾
    ));

    let smoke2 = Arc::new(constant_medium::ConstantMedium::new_with_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0), // 白色烟雾
    ));

    world.add(smoke1);
    world.add(smoke2);

    let bvh_node = BvhNode::new(&world);
    let mut world_bvh = HittableList::new();
    world_bvh.add(bvh_node);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = 600;
    cam.samples_per_pixel = 200;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0); // 黑色背景

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 278.0, -800.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world_bvh);
}

fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut boxes1 = HittableList::new();
    let ground = Arc::new(Lambertian::new(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f64 * w;
            let z0 = -1000.0 + j as f64 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = random_double_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(box_new(
                Point3::new(x0, y0, z0),
                Point3::new(x1, y1, z1),
                ground.clone(),
            ));
        }
    }

    let mut world = HittableList::new();
    let boxes1_bvh = BvhNode::new(&boxes1);
    world.add(boxes1_bvh);

    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Arc::new(Quad::new(
        Point3::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    let center1 = Point3::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material = Arc::new(Lambertian::new(Color::new(0.7, 0.3, 0.1)));
    world.add(Arc::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));

    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    let boundary = Arc::new(Sphere::new(
        Point3::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(boundary.clone());

    let medium1 = Arc::new(constant_medium::ConstantMedium::new_with_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    ));
    world.add(medium1);

    let boundary2 = Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    let medium2 = Arc::new(constant_medium::ConstantMedium::new_with_color(
        boundary2,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    ));
    world.add(medium2);

    let earth_texture = Arc::new(texture::ImageTexture::new("grumble.jpg"));
    let earth_material = Arc::new(Lambertian::from_texture(earth_texture));
    world.add(Arc::new(Sphere::new(
        Point3::new(400.0, 200.0, 400.0),
        100.0,
        earth_material,
    )));

    let perlin_texture = Arc::new(NoiseTexture::new(0.2));
    let perlin_material = Arc::new(Lambertian::from_texture(perlin_texture));
    world.add(Arc::new(Sphere::new(
        Point3::new(220.0, 280.0, 300.0),
        80.0,
        perlin_material,
    )));

    let mut boxes2 = HittableList::new();
    // let white = Arc::new(Lambertian::new(Color::new(0.73, 0.73, 0.73)));
    let grumble_texture = Arc::new(texture::ImageTexture::new("grumble.jpg"));
    let grumble_material = Arc::new(Lambertian::from_texture(grumble_texture));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Arc::new(Sphere::new(
            Point3::random_range(0.0, 165.0),
            10.0,
            grumble_material.clone(),
        )));
    }

    let boxes2_bvh = BvhNode::new(&boxes2);
    let mut boxes2_transformed = boxes2_bvh as Arc<dyn Hittable + Send + Sync>;
    boxes2_transformed = Arc::new(RotateY::new(boxes2_transformed, 15.0));
    boxes2_transformed = Arc::new(Translate::new(
        boxes2_transformed,
        Vec3::new(-100.0, 270.0, 395.0),
    ));
    world.add(boxes2_transformed);

    let mut cam = Camera::new();

    cam.aspect_ratio = 1.0;
    cam.image_width = image_width as i32;
    cam.samples_per_pixel = samples_per_pixel as i32;
    cam.max_depth = max_depth as i32;
    cam.background = Color::new(0.0, 0.0, 0.0); // 黑色背景

    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(478.0, 278.0, -600.0);
    cam.lookat = Point3::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn cornell_box_with_obj() {
    let mut world = HittableList::new();

    // 加载 OBJ 模型
    let material = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 0.2));
    let bunny = ObjModel::load(
        "models/cottage_obj.obj", // 替换为你的OBJ文件路径
        material,
        1000.0,                           // 缩放
        Point3::new(278.0, 100.0, 280.0), // 位置
    )
    .expect("Failed to load OBJ model");

    world.add(Arc::new(bunny));

    let mut cam = Camera::new();
    cam.aspect_ratio = 16.0 / 9.0;
    cam.image_width = 1200;
    cam.samples_per_pixel = 500;
    cam.max_depth = 50;
    cam.background = Color::new(0.0, 0.0, 0.0); // 纯黑背景

    // 调整相机参数聚焦模型
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(278.0, 200.0, -500.0); // 更近的观察距离
    cam.lookat = Point3::new(278.0, 100.0, 280.0); // 对准模型中心
    cam.vup = Point3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn test_mesh_rendering() {
    println!("Starting mesh rendering test...");
    let mut world = HittableList::new();

    // 添加光源
    let light_color = Arc::new(SolidColor::new(Color::new(15.0, 15.0, 15.0)));
    let light = Arc::new(DiffuseLight::new(light_color));
    world.add(Arc::new(Quad::new(
        Point3::new(0.0, 2.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 2.0),
        light,
    )));

    // 使用更明显的材质
    let material = Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 0.2));

    // 加载测试模型
    println!("Loading OBJ model...");
    let mesh =
        Mesh::from_obj("models/test_triangle.obj", material).expect("Failed to load OBJ file");
    world.add(Arc::new(mesh));

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 4;
    cam.max_depth = 2;
    cam.background = Color::new(0.0, 0.0, 0.0);

    // 调整相机位置确保能看到三角形
    cam.vfov = 40.0;
    cam.lookfrom = Point3::new(0.0, 0.5, 2.0); // 更靠近三角形
    cam.lookat = Point3::new(0.0, 0.0, 0.0); // 看向三角形中心
    cam.vup = Point3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    println!("Rendering...");
    // cam.render(&world);
    println!("Rendering completed!");
}

fn test_triangle() {
    let mut world = HittableList::new();
    let red = Arc::new(Lambertian::new(Color::new(0.8, 0.2, 0.2)));
    let triangle = Arc::new(Triangle::new(
        Point3::new(-10.0, 0.0, -5.0),
        Point3::new(10.0, 0.0, -5.0),
        Point3::new(0.0, 10.0, -5.0),
        red,
    ));
    world.add(triangle);

    let blue = Arc::new(Lambertian::new(Color::new(0.2, 0.2, 1.0)));
    world.add(Arc::new(Sphere::new(
        Point3::new(0.0, 0.0, -20.0),
        10.0,
        blue,
    )));

    let mut cam = Camera::new();
    cam.aspect_ratio = 1.0;
    cam.image_width = 400;
    cam.samples_per_pixel = 100;
    cam.max_depth = 50;
    cam.background = Color::new(0.7, 0.8, 1.0);
    cam.vfov = 80.0;
    cam.lookfrom = Point3::new(0.0, 0.0, 9.0);
    cam.lookat = Point3::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    // cam.render(&world);
}

fn main() {
    let start = Instant::now(); // 开始计时

    // last_picture_the_first_book(); // bouncing spheres
    // for_output13();
    // checkered_spheres();
    // earth();
    // perlin_spheres();
    // quads();
    // simple_light();
    cornell_box();
    // cornell_smoke();
    // final_scene(400, 250, 4);
    // final_scene(800, 10000, 40);
    // cornell_box_with_obj();
    // test_mesh_rendering();
    // test_triangle();

    let elapsed = start.elapsed();
    println!("\n渲染完成,用时: {:.2}秒", elapsed.as_secs_f64());
}
