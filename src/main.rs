mod camera;
mod image;
mod interval;
mod material;
mod object;
mod object_list;
mod ray;
mod renderer;
mod sphere;
mod utils;
mod vec3;

use std::sync::Arc;

use crate::{
    camera::Camera,
    material::{Dielectric, Lambertian, Material, Metal},
    object_list::ObjectList,
    sphere::Sphere,
    utils::{random, random_range},
    vec3::{Color, Point, Vec3},
};

fn main() {
    let mut world = ObjectList::new();

    scene_setup(&mut world);

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 1240;
    let samples_per_pixel: usize = 200;
    let max_depth: usize = 50;

    let mut cam = Camera::new(aspect_ratio, img_w, samples_per_pixel, max_depth);

    cam.vfov = 20.0;
    cam.lookfrom = Point::new(13.0, 2.0, 3.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);

    cam.defocus_angle = 0.0;
    cam.focus_dist = 10.4;

    let img = cam.render(&world);

    // utils::clear_screen();
    // println!("{}", img);
    std::fs::write("output.ppm", img.to_ppm()).unwrap();
}

fn scene_setup(world: &mut ObjectList) {
    // Ground sphere
    let m_ground = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        m_ground,
    )));

    // Scattered Ballz
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = random();
            let center = Point::new(a as f32 + 0.9 * random(), 0.2, b as f32 + 0.9 * random());

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let mat: Arc<dyn Material> = match choose_mat {
                    x if x < 0.8 => Arc::new(Lambertian::new(Color::rand() * Color::rand())),
                    x if x < 0.95 => {
                        Arc::new(Metal::new(Color::rand_in(0.5, 1.0), random_range(0.0, 0.5)))
                    }
                    _ => Arc::new(Dielectric::new(1.5)),
                };
                world.add(Box::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }

    // Metal
    let m_metal = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m_metal,
    )));

    // Dielectric
    let m_dielectric = Arc::new(Dielectric::new(1.50));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m_dielectric,
    )));

    //Lambertian
    let m_lambertian = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m_lambertian,
    )));
}
