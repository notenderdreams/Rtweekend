mod camera;
mod controller;
mod gui;
mod image;
mod interval;
mod material;
mod object;
mod object_list;
mod ray;
mod renderer;
mod sphere;
mod triangle;
mod utils;
mod vec3;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    camera::Camera,
    controller::CameraController,
    gui::Preview,
    material::{Dielectric, Lambertian, Material, Metal},
    object_list::ObjectList,
    sphere::Sphere,
    triangle::{Triangle, make_box},
    utils::Rng,
    vec3::{Color, Point, Vec3},
};

fn main() {
    let mut world = ObjectList::new();
    let mut rng = Rng::new(0);

    scene_setup(&mut world, &mut rng);
    let world = Arc::new(world);

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 960;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 12;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.vfov = 32.0;
    cam.lookfrom = Point::new(3.8, 3.2, 7.8);
    cam.lookat = Point::new(0.0, 1.2, 0.5);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.4;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    let controller = Arc::new(Mutex::new(CameraController::new(
        cam.lookfrom,
        cam.lookat,
        cam.vup,
        cam.vfov,
    )));

    let preview = Arc::new(Preview::new(cam.img_w, cam.img_h, Arc::clone(&controller)));
    let render_preview = Arc::clone(&preview);
    let render_world = Arc::clone(&world);

    let handle =
        thread::spawn(move || cam.render_loop(render_world.as_ref(), Some(&render_preview)));
    preview.run("Rtweekend");

    if let Some(img) = handle.join().expect("render thread panicked") {
        std::fs::write("output.ppm", img.to_ppm()).unwrap();
    }
}

fn scene_setup(world: &mut ObjectList, _rng: &mut Rng) {
    let m_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.72, 0.70, 0.67)));
    let m_white: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.88, 0.88, 0.88)));
    let m_coral: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.85, 0.32, 0.22)));
    let m_teal: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.12, 0.45, 0.45)));

    let m_gold: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.95, 0.78, 0.25), 0.05));
    let m_mirror: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.92, 0.94, 0.96), 0.0));
    let m_copper: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.88, 0.55, 0.38), 0.2));

    let m_glass: Arc<dyn Material> = Arc::new(Dielectric::new(1.5));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        m_ground,
    )));

    world.add(Box::new(make_box(
        Point::new(1.6, 0.0, -0.8),
        Point::new(3.2, 3.2, 0.8),
        Arc::clone(&m_mirror),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(2.4, 3.95, 0.0),
        0.75,
        Arc::clone(&m_gold),
    )));

    world.add(Box::new(make_box(
        Point::new(-3.4, 0.0, -1.2),
        Point::new(-1.2, 1.8, 1.0),
        Arc::clone(&m_coral),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(-2.3, 2.45, -0.1),
        0.65,
        Arc::clone(&m_glass),
    )));

    world.add(Box::new(make_box(
        Point::new(-0.85, 0.0, 0.6),
        Point::new(0.85, 1.7, 2.3),
        Arc::clone(&m_glass),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.85, 1.45),
        0.45,
        Arc::clone(&m_gold),
    )));

    world.add(Box::new(make_box(
        Point::new(-1.2, 0.0, -3.2),
        Point::new(1.2, 1.0, -1.6),
        Arc::clone(&m_white),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 2.0, -2.4),
        1.0,
        Arc::clone(&m_teal),
    )));

    world.add(Box::new(Triangle::new(
        Point::new(-2.2, 3.6, -1.5),
        Point::new(0.0, 5.2, -1.0),
        Point::new(1.4, 4.0, -2.2),
        Arc::clone(&m_copper),
    )));

    world.add(Box::new(Sphere::new(
        Point::new(-0.5, 0.25, 2.9),
        0.25,
        Arc::clone(&m_glass),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(1.3, 0.35, 2.4),
        0.35,
        Arc::clone(&m_mirror),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(2.7, 0.22, 1.8),
        0.22,
        Arc::clone(&m_coral),
    )));
}

#[allow(dead_code)]
fn scene_random_spheres(world: &mut ObjectList, rng: &mut Rng) {
    let m_ground: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.5, 0.5, 0.5)));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        m_ground,
    )));

    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random();
            let center = Point::new(
                a as f32 + 0.9 * rng.random(),
                0.2,
                b as f32 + 0.9 * rng.random(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let mat: Arc<dyn Material> = match choose_mat {
                    x if x < 0.8 => Arc::new(Lambertian::new(Color::rand(rng) * Color::rand(rng))),
                    x if x < 0.95 => Arc::new(Metal::new(
                        Color::rand_in(0.5, 1.0, rng),
                        rng.random_range(0.0, 0.5),
                    )),
                    _ => Arc::new(Dielectric::new(1.5)),
                };
                world.add(Box::new(Sphere::new(center, 0.2, mat)));
            }
        }
    }

    let m_metal: Arc<dyn Material> = Arc::new(Metal::new(Color::new(0.7, 0.6, 0.5), 0.0));
    world.add(Box::new(Sphere::new(
        Point::new(4.0, 1.0, 0.0),
        1.0,
        m_metal,
    )));

    let m_dielectric: Arc<dyn Material> = Arc::new(Dielectric::new(1.50));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 1.0, 0.0),
        1.0,
        m_dielectric,
    )));

    let m_lambertian: Arc<dyn Material> = Arc::new(Lambertian::new(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m_lambertian,
    )));
}
