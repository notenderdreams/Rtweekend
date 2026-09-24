mod image;
mod ray;
mod renderer;
mod utils;
mod vec3;

use image::Image;
use ray::Ray;
use vec3::Color;

use crate::vec3::{Point, Vec3};

fn main() {
    let aspect_ratio = 16.0 / 9.0;
    let img_w: usize = 400;
    let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);

    let mut img = Image::new(img_w, img_h);

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

        let color = ray_color(&r);
        to_u8(color)
    });

    utils::clear_screen();
    println!("{}", img);
}

fn hit_sphere(center: Point, radius: f32, r: &Ray) -> f32 {
    let oc = center - r.origin;
    let a = r.direction.len_squared();
    let h = r.direction.dot(oc);
    let c = oc.len_squared() - radius * radius;
    let discriminant = h * h - a * c;
    if discriminant < 0.0 {
        -1.0
    } else {
        (h - discriminant.sqrt()) / a
    }
}

fn ray_color(r: &Ray) -> Color {
    let center = Point::new(0.0, 0.0, -1.0);
    let t = hit_sphere(center, 0.5, r);
    if t > 0.0 {
        let n = (r.at(t) - center).normalize();
        return 0.5 * Color::new(n.x + 1.0, n.y + 1.0, n.z + 1.0);
    }

    let unit_dir = r.direction.normalize();
    let a = 0.5 * (unit_dir.y + 1.0);
    (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
}

fn to_u8(c: Color) -> (u8, u8, u8) {
    (
        (255.999 * c.x) as u8,
        (255.999 * c.y) as u8,
        (255.999 * c.z) as u8,
    )
}
