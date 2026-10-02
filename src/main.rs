mod aabb;
mod bvh;
mod camera;
mod constant_medium;
mod controller;
mod gui;
mod image;
mod instance;
mod interval;
mod material;
mod object;
mod object_list;
mod perlin;
mod quad;
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
    constant_medium::ConstantMedium,
    controller::CameraController,
    gui::Preview,
    instance::{RotateY, Translate},
    material::{Dielectric, DiffuseLight, Lambertian, Material, Metal},
    object::Object,
    object_list::ObjectList,
    quad::{Quad, box_primitive},
    sphere::Sphere,
    texture::{CheckerTexture, ImageTexture, NoiseTexture},
    utils::Rng,
    vec3::{Color, Point, Vec3},
};

fn main() {
    let scene_id: usize = std::env::args()
        .nth(1)
        .and_then(|s| s.parse().ok())
        .unwrap_or(9);

    match scene_id {
        1 => bouncing_spheres(),
        2 => checkered_spheres(),
        3 => earth(),
        4 => perlin_spheres(),
        5 => quads(),
        6 => simple_light(),
        7 => cornell_box(),
        8 => cornell_smoke(),
        9 => final_scene(800, 1000, 40),
        10 => final_scene(400, 250, 4),
        11 => planar_shapes(),
        _ => final_scene(400, 250, 4),
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

    cam.bg = Color::new(0.70, 0.80, 1.00);
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

    cam.bg = Color::new(0.70, 0.80, 1.00);
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
        println!("Render complete! Saved image to output.ppm");
    } else {
        std::fs::write("output.ppm", preview.to_ppm()).unwrap();
        println!("Saved current preview buffer to output.ppm");
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

    cam.bg = Color::new(0.70, 0.80, 1.00);
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

    cam.bg = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 20.0;
    cam.lookfrom = Point::new(13.0, 2.0, 3.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

fn quads() {
    let mut world = ObjectList::new();

    // Materials
    let left_red: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let back_green: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let right_blue: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.2, 0.2, 1.0)));
    let upper_orange: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(Color::new(1.0, 0.5, 0.0)));
    let lower_teal: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.2, 0.8, 0.8)));

    // Quads
    world.add(Box::new(Quad::new(
        Point::new(-3.0, -2.0, 5.0),
        Vec3::new(0.0, 0.0, -4.0),
        Vec3::new(0.0, 4.0, 0.0),
        left_red,
    )));
    world.add(Box::new(Quad::new(
        Point::new(-2.0, -2.0, 0.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 4.0, 0.0),
        back_green,
    )));
    world.add(Box::new(Quad::new(
        Point::new(3.0, -2.0, 1.0),
        Vec3::new(0.0, 0.0, 4.0),
        Vec3::new(0.0, 4.0, 0.0),
        right_blue,
    )));
    world.add(Box::new(Quad::new(
        Point::new(-2.0, 3.0, 1.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 4.0),
        upper_orange,
    )));
    world.add(Box::new(Quad::new(
        Point::new(-2.0, -3.0, 5.0),
        Vec3::new(4.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -4.0),
        lower_teal,
    )));

    // Camera
    let aspect_ratio = 1.0;
    let img_w: usize = 400;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.bg = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 80.0;
    cam.lookfrom = Point::new(0.0, 0.0, 9.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

fn planar_shapes() {
    let mut world = ObjectList::new();

    let mat_tri: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(1.0, 0.2, 0.2)));
    let mat_disk: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.2, 1.0, 0.2)));
    let mat_ring: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.2, 0.4, 1.0)));
    let mat_floor: Arc<dyn Material> = Arc::new(Lambertian::from_color(Color::new(0.8, 0.8, 0.8)));

    // Floor Quad (Parallelogram)
    world.add(Box::new(Quad::new(
        Point::new(-5.0, -1.5, -5.0),
        Vec3::new(10.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 10.0),
        mat_floor,
    )));

    // 1. Triangle on the left
    world.add(Box::new(Quad::triangle(
        Point::new(-3.5, -1.0, 0.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(1.0, 2.0, 0.0),
        mat_tri,
    )));

    // 2. Circular Disk in the center
    world.add(Box::new(Quad::ellipse(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        mat_disk,
    )));

    // 3. Annulus (Ring) on the right (inner radius = 0.5)
    world.add(Box::new(Quad::annulus(
        Point::new(3.0, 0.0, 0.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        0.5,
        mat_ring,
    )));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 600;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.bg = Color::new(0.70, 0.80, 1.00);
    cam.vfov = 40.0;
    cam.lookfrom = Point::new(0.0, 2.0, 8.0);
    cam.lookat = Point::new(0.0, 0.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

fn simple_light() {
    let mut world = ObjectList::new();

    // 1. Two marbled spheres
    let pertext = Arc::new(NoiseTexture::new(4.0));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -1000.0, 0.0),
        1000.0,
        Arc::new(Lambertian::new(pertext.clone())),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 2.0, 0.0),
        2.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    // 2. Light materials (intensity = 4.0)
    let difflight: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(Color::new(4.0, 4.0, 4.0)));

    // 3. Rectangular light ceiling panel
    world.add(Box::new(Quad::new(
        Point::new(3.0, 1.0, -2.0),
        Vec3::new(2.0, 0.0, 0.0),
        Vec3::new(0.0, 2.0, 0.0),
        difflight.clone(),
    )));

    // 4. Glowing light sphere suspended in the air
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 7.0, 0.0),
        2.0,
        difflight,
    )));

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 900;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 500;
    let max_depth: usize = 100;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);

    cam.bg = Color::new(0.0, 0.0, 0.0);

    cam.vfov = 20.0;
    cam.lookfrom = Point::new(26.0, 3.0, 6.0);
    cam.lookat = Point::new(0.0, 2.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;
    cam.focus_dist = (cam.lookfrom - cam.lookat).len();

    render(cam, world);
}

pub fn cornell_box() {
    let mut world = ObjectList::new();

    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(15.0, 15.0, 15.0)));

    // 5 Walls + Ceiling Light
    world.add(Box::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Box::new(Quad::new(
        Point::new(343.0, 554.0, 332.0),
        Vec3::new(-130.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -105.0),
        light,
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point::new(555.0, 555.0, 555.0),
        Vec3::new(-555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, -555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Box 1 (Tall Block): 165 x 330 x 165, rotated 15 deg, translated to (265, 0, 295)
    let box1 = box_primitive(
        Point::zero(),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Box::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(box1);

    // Box 2 (Short Block): 165 x 165 x 165, rotated -18 deg, translated to (130, 0, 65)
    let box2 = box_primitive(Point::zero(), Point::new(165.0, 165.0, 165.0), white);
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Box::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(box2);

    // Camera Configuration
    let aspect_ratio = 1.0;
    let img_w: usize = 600;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 200;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);
    cam.bg = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point::new(278.0, 278.0, -800.0);
    cam.lookat = Point::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    render(cam, world);
}

pub fn cornell_smoke() {
    let mut world = ObjectList::new();

    let red = Arc::new(Lambertian::from_color(Color::new(0.65, 0.05, 0.05)));
    let white = Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let green = Arc::new(Lambertian::from_color(Color::new(0.12, 0.45, 0.15)));
    let light = Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));

    // Cornell Walls
    world.add(Box::new(Quad::new(
        Point::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        green,
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        red,
    )));
    world.add(Box::new(Quad::new(
        Point::new(113.0, 554.0, 127.0),
        Vec3::new(330.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 305.0),
        light,
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 555.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 0.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 555.0),
        white.clone(),
    )));
    world.add(Box::new(Quad::new(
        Point::new(0.0, 0.0, 555.0),
        Vec3::new(555.0, 0.0, 0.0),
        Vec3::new(0.0, 555.0, 0.0),
        white.clone(),
    )));

    // Box 1: Tall Block converted to Black Smoke
    let box1 = box_primitive(
        Point::zero(),
        Point::new(165.0, 330.0, 165.0),
        white.clone(),
    );
    let box1 = Arc::new(RotateY::new(box1, 15.0));
    let box1 = Arc::new(Translate::new(box1, Vec3::new(265.0, 0.0, 295.0)));
    world.add(Box::new(ConstantMedium::from_color(
        box1,
        0.01,
        Color::new(0.0, 0.0, 0.0),
    )));

    // Box 2: Short Block converted to White Fog
    let box2 = box_primitive(Point::zero(), Point::new(165.0, 165.0, 165.0), white);
    let box2 = Arc::new(RotateY::new(box2, -18.0));
    let box2 = Arc::new(Translate::new(box2, Vec3::new(130.0, 0.0, 65.0)));
    world.add(Box::new(ConstantMedium::from_color(
        box2,
        0.01,
        Color::new(1.0, 1.0, 1.0),
    )));

    // Camera Configuration
    let aspect_ratio = 1.0;
    let img_w: usize = 600;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);
    let samples_per_pixel: usize = 200;
    let max_depth: usize = 50;

    let mut cam = Camera::new(img_w, img_h, samples_per_pixel, max_depth);
    cam.bg = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point::new(278.0, 278.0, -800.0);
    cam.lookat = Point::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    render(cam, world);
}

pub fn final_scene(image_width: usize, samples_per_pixel: usize, max_depth: usize) {
    let mut rng = Rng::new(0);

    // 1. 20x20 ground grid of boxes with random heights, grouped in a BVH
    let mut boxes1 = ObjectList::new();
    let ground: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(Color::new(0.48, 0.83, 0.53)));

    let boxes_per_side = 20;
    for i in 0..boxes_per_side {
        for j in 0..boxes_per_side {
            let w = 100.0;
            let x0 = -1000.0 + i as f32 * w;
            let z0 = -1000.0 + j as f32 * w;
            let y0 = 0.0;
            let x1 = x0 + w;
            let y1 = rng.random_range(1.0, 101.0);
            let z1 = z0 + w;

            boxes1.add(Box::new(box_primitive(
                Point::new(x0, y0, z0),
                Point::new(x1, y1, z1),
                ground.clone(),
            )));
        }
    }

    let mut world = ObjectList::new();
    world.add(Box::new(BVHNode::from_list(boxes1, &mut rng)));

    // 2. Ceiling light
    let light: Arc<dyn Material> =
        Arc::new(DiffuseLight::from_color(Color::new(7.0, 7.0, 7.0)));
    world.add(Box::new(Quad::new(
        Point::new(123.0, 554.0, 147.0),
        Vec3::new(300.0, 0.0, 0.0),
        Vec3::new(0.0, 0.0, 265.0),
        light,
    )));

    // 3. Moving sphere (motion blur)
    let center1 = Point::new(400.0, 400.0, 200.0);
    let center2 = center1 + Vec3::new(30.0, 0.0, 0.0);
    let sphere_material: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(Color::new(0.7, 0.3, 0.1)));
    world.add(Box::new(Sphere::new_moving(
        center1,
        center2,
        50.0,
        sphere_material,
    )));

    // 4. Glass sphere and metal sphere
    world.add(Box::new(Sphere::new(
        Point::new(260.0, 150.0, 45.0),
        50.0,
        Arc::new(Dielectric::new(1.5)),
    )));
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 150.0, 145.0),
        50.0,
        Arc::new(Metal::new(Color::new(0.8, 0.8, 0.9), 1.0)),
    )));

    // 5. Blue subsurface sphere (glass boundary with dense constant medium inside)
    let boundary: Arc<dyn Object> = Arc::new(Sphere::new(
        Point::new(360.0, 150.0, 145.0),
        70.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Box::new(boundary.clone()));
    world.add(Box::new(ConstantMedium::from_color(
        boundary,
        0.2,
        Color::new(0.2, 0.4, 0.9),
    )));

    // 6. Global thin mist covering the entire scene
    let mist_boundary: Arc<dyn Object> = Arc::new(Sphere::new(
        Point::new(0.0, 0.0, 0.0),
        5000.0,
        Arc::new(Dielectric::new(1.5)),
    ));
    world.add(Box::new(ConstantMedium::from_color(
        mist_boundary,
        0.0001,
        Color::new(1.0, 1.0, 1.0),
    )));

    // 7. Earth globe
    let emat: Arc<dyn Material> = Arc::new(Lambertian::new(Arc::new(ImageTexture::new(
        "assets/earthmap.jpg",
    ))));
    world.add(Box::new(Sphere::new(
        Point::new(400.0, 200.0, 400.0),
        100.0,
        emat,
    )));

    // 8. Marble (Perlin noise) sphere
    let pertext = Arc::new(NoiseTexture::new(0.2));
    world.add(Box::new(Sphere::new(
        Point::new(220.0, 280.0, 300.0),
        80.0,
        Arc::new(Lambertian::new(pertext)),
    )));

    // 9. Cluster of 1000 spheres packed into a rotated and translated BVH
    let mut boxes2 = ObjectList::new();
    let white: Arc<dyn Material> =
        Arc::new(Lambertian::from_color(Color::new(0.73, 0.73, 0.73)));
    let ns = 1000;
    for _ in 0..ns {
        boxes2.add(Box::new(Sphere::new(
            Point::rand_in(0.0, 165.0, &mut rng),
            10.0,
            white.clone(),
        )));
    }

    let bvh2 = Arc::new(BVHNode::from_list(boxes2, &mut rng));
    let rot = Arc::new(RotateY::new(bvh2, 15.0));
    let trans = Box::new(Translate::new(rot, Vec3::new(-100.0, 270.0, 395.0)));
    world.add(trans);

    // Camera Configuration
    let aspect_ratio = 1.0;
    let img_h = ((image_width as f32 / aspect_ratio) as usize).max(1);

    let mut cam = Camera::new(image_width, img_h, samples_per_pixel, max_depth);
    cam.bg = Color::new(0.0, 0.0, 0.0);
    cam.vfov = 40.0;
    cam.lookfrom = Point::new(478.0, 278.0, -600.0);
    cam.lookat = Point::new(278.0, 278.0, 0.0);
    cam.vup = Vec3::new(0.0, 1.0, 0.0);
    cam.defocus_angle = 0.0;

    render(cam, world);
}



