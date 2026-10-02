mod aabb;
mod bvh;
mod camera;
mod controller;
mod gui;
mod image;
mod interval;
mod material;
mod object;
mod object_list;
mod perlin;
mod ray;
mod renderer;
mod sphere;
mod texture;
// mod triangle;
mod utils;
mod vec3;

use std::{
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    bvh::BVHNode,
    camera::Camera,
    controller::CameraController,
    gui::Preview,
    material::{Dielectric, Lambertian, Material, Metal},
    object_list::ObjectList,
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    utils::Rng,
    vec3::{Color, Point, Vec3},
};

fn main() {
    let scene_id: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(2);

    match scene_id {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        _ => checkered_spheres(),
    }
}

fn bouncing_spheres() {
    let mut spheres = ObjectList::new();
    let mut rng = Rng::new(0);

    scene_setup(&mut spheres, &mut rng);
    let bvh_tree = BVHNode::from_list(spheres, &mut rng);

    let mut world = ObjectList::new();
    let checker = Arc::new(CheckerTexture::from_colors(
        0.9,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));
    let m_ground: Arc<dyn Material> = Arc::new(Lambertian::new(checker));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        m_ground,
    )));
    world.add(Box::new(bvh_tree));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 1240;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.vfov = 32.0;
    cam.lookfrom = Point::new(3.8, 3.2, 7.8);
    cam.lookat = Point::new(0.0, 1.2, 0.5);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.4;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

fn checkered_spheres() {
    let mut world = ObjectList::new();

    let checker = Arc::new(CheckerTexture::from_colors(
        0.32,
        Color::new(0.2, 0.3, 0.1),
        Color::new(0.9, 0.9, 0.9),
    ));

    let mat: Arc<dyn Material> = Arc::new(Lambertian::new(checker));

    world.add(Box::new(Sphere::new(
        Point::new(0.0, -10.0, 0.0),
        10.0,
        mat.clone(),
    )));
    world.add(Box::new(Sphere::new(Point::new(0.0, 10.0, 0.0), 10.0, mat)));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 400;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.vfov = 20.0;
    cam.lookfrom = Point::new(13.0, 2.0, 3.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

fn render(mut cam: Camera, world: ObjectList) {
    let render_world = Arc::new(world);

    let controller = Arc::new(Mutex::new(CameraController::new(
        cam.lookfrom,
        cam.lookat,
        cam.vup,
        cam.vfov,
    )));

    let preview = Arc::new(Preview::new(cam.img_w, cam.img_h, Arc::clone(&controller)));
    let render_preview = Arc::clone(&preview);

    let handle =
        thread::spawn(move || cam.render_loop(render_world.as_ref(), Some(&render_preview)));
    preview.run("Rtweekend");

    if let Some(img) = handle.join().expect("render thread panicked") {
        std::fs::write("output.ppm", img.to_ppm()).unwrap();
    }
}
fn earth() {
    let mut world = ObjectList::new();

    // Directly specify the path to the JPG
    let earth_texture = Arc::new(ImageTexture::new("assets/earthmap.jpg"));
    let earth_surface: Arc<dyn Material> = Arc::new(Lambertian::new(earth_texture));
    let globe = Box::new(Sphere::new(Point::new(0.0, 0.0, 0.0), 2.0, earth_surface));
    world.add(globe);

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 400;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.vfov = 20.0;
    cam.lookfrom = Point::new(0.0, 0.0, 12.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

fn scene_setup(world: &mut ObjectList, rng: &mut Rng) {
    for a in -11..11 {
        for b in -11..11 {
            let choose_mat = rng.random();
            let center = Point::new(
                a as f32 + 0.9 * rng.random(),
                0.2,
                b as f32 + 0.9 * rng.random(),
            );

            if (center - Point::new(4.0, 0.2, 0.0)).len() > 0.9 {
                let sphere = match choose_mat {
                    x if x < 0.8 => {
                        let mat =
                            Arc::new(Lambertian::from_color(Color::rand(rng) * Color::rand(rng)));
                        let center2 = center + Vec3::new(0.0, rng.random_range(0.0, 0.5), 0.0);
                        Sphere::new_moving(center, center2, 0.2, mat)
                    }
                    x if x < 0.95 => {
                        let mat = Arc::new(Metal::new(
                            Color::rand_in(0.5, 1.0, rng),
                            rng.random_range(0.0, 0.5),
                        ));
                        Sphere::new(center, 0.2, mat)
                    }
                    _ => Sphere::new(center, 0.2, Arc::new(Dielectric::new(1.5))),
                };
                world.add(Box::new(sphere));
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

    let m_lambertian: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(Color::new(0.4, 0.2, 0.1)));
    world.add(Box::new(Sphere::new(
        Point::new(-4.0, 1.0, 0.0),
        1.0,
        m_lambertian,
    )));
}

fn perlin_spheres() {
    let mut world = ObjectList::new();

    let pertext = Arc::new(NoiseTexture::new(4.0));

    // Large ground sphere
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));

    // Smaller top sphere
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    // Camera setup
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 400;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.vfov = 20.0;
    cam.lookfrom = Point::new(13.0, 2.0, 3.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}
