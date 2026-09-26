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
    material::{Dielectric, Lambertian, Metal},
    object_list::ObjectList,
    sphere::Sphere,
    vec3::{Color, Point},
};

fn main() {
    let mut world = ObjectList::new();

    scene_setup(&mut world);

    // Camera
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 400;
    let samples_per_pixel: usize = 100;
    let max_depth: usize = 50;

    let mut cam = Camera::new(aspect_ratio, img_w, samples_per_pixel, max_depth);
    let img = cam.render(&world);

    // utils::clear_screen();
    // println!("{}", img);
    std::fs::write("output.ppm", img.to_ppm()).unwrap();
}

fn scene_setup(world: &mut ObjectList) {
    // Materials
    let material_ground = Arc::new(Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Arc::new(Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Arc::new(Dielectric::new(1.0 / 1.33));
    let material_right = Arc::new(Metal::new(Color::new(0.8, 0.6, 0.2), 1.0));

    // Ground sphere
    world.add(Box::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground,
    )));

    // Center diffuse sphere
    world.add(Box::new(Sphere::new(
        Point::new(0.0, 0.0, -1.2),
        0.5,
        material_center,
    )));

    // Left dielectric sphere
    world.add(Box::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left,
    )));

    // Right metal sphere
    world.add(Box::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right,
    )));
}
