mod image;
mod interval;
mod object;
mod object_list;
mod ray;
mod renderer;
mod sphere;
mod utils;
mod vec3;

use crate::{
    image::Image,
    interval::Interval,
    object::{HRecord, Object},
    object_list::ObjectList,
    ray::Ray,
    sphere::Sphere,
    vec3::{Color, Point, Vec3},
};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 400;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);

    let mut img = Image::new(img_w, img_h);

    let mut world = ObjectList::new();
    world.add(Box::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Box::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    let focal_len = 1.0;
    let vp_h = 2.0;
    let vp_w = vp_h * (img_w as f32 / img_h as f32);
    let cam_center = Point::zero();

    let vp_u = Vec3::new(vp_w, 0.0, 0.0);
    let vp_v = Vec3::new(0.0, -vp_h, 0.0);

    let px_del_u = vp_u / img_w as f32;
    let px_del_v = vp_v / img_h as f32;

    let vp_upper_left = cam_center - Vec3::new(0.0, 0.0, focal_len) - vp_u / 2.0 - vp_v / 2.0;
    let pixel00_loc = vp_upper_left + (px_del_u + px_del_v) * 0.5;

    renderer::render(&mut img, |x, y, _w, _h| {
        let pixel_center = pixel00_loc + px_del_u * x as f32 + px_del_v * y as f32;
        let ray_direction = pixel_center - cam_center;
        let r = Ray::new(cam_center, ray_direction);

        let color = ray_color(&r, &world);
        utils::to_u8(color)
    });

    utils::clear_screen();
    println!("{}", img);
}

fn ray_color(r: &Ray, world: &dyn Object) -> Color {
    let mut rec = HRecord {
        p: Point::zero(),
        normal: Vec3::zero(),
        t: 0.0,
        front_face: false,
    };

    if world.hit(r, Interval::new(0.0, f32::INFINITY), &mut rec) {
        return 0.5 * (rec.normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_dir = r.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}
