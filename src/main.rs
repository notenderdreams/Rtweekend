mod camera;
mod image;
mod interval;
mod object;
mod object_list;
mod ray;
mod renderer;
mod sphere;
mod utils;
mod vec3;

use crate::{camera::Camera, object_list::ObjectList, sphere::Sphere, vec3::Point};

fn main() {
    let mut world = ObjectList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

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
