use crate::{
    image::Image,
    interval::Interval,
    object::{HRecord, Object},
    ray::Ray,
    renderer,
    utils::random,
    vec3::{Color, Point, Vec3, random_unit_vector},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub img_w: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    img_h: usize,
    center: Point,
    px_del_u: Vec3,
    px_del_v: Vec3,
    px00_loc: Point,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        img_w: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);

        let focal_len = 1.0;
        let vp_h = 2.0;
        let vp_w = vp_h * (img_w as f32 / img_h as f32);
        let center = Point::zero();

        let vp_u = Vec3::new(vp_w, 0.0, 0.0);
        let vp_v = Vec3::new(0.0, -vp_h, 0.0);

        let px_del_u = vp_u / img_w as f32;
        let px_del_v = vp_v / img_h as f32;

        let vp_upper_left = center - Vec3::new(0.0, 0.0, focal_len) - vp_u / 2.0 - vp_v / 2.0;
        let px00_loc = vp_upper_left + (px_del_u + px_del_v) * 0.5;

        Self {
            aspect_ratio,
            img_w,
            samples_per_pixel,
            max_depth,
            img_h,
            center,
            px_del_u,
            px_del_v,
            px00_loc,
        }
    }

    pub fn render(&mut self, world: &dyn Object) -> Image {
        let mut img = Image::new(self.img_w, self.img_h);

        renderer::render(&mut img, self.samples_per_pixel, |x, y, _s| {
            let r = self.get_ray(x, y);
            self.ray_color(&r, world, self.max_depth)
        });

        img
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let offset = sample_square();
        let px_sample = self.px00_loc
            + self.px_del_u * (x as f32 + offset.x)
            + self.px_del_v * (y as f32 + offset.y);
        let ray_dir = px_sample - self.center;
        Ray::new(self.center, ray_dir)
    }

    fn ray_color(&self, r: &Ray, world: &dyn Object, depth: usize) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        let mut rec = HRecord {
            p: Point::zero(),
            normal: Vec3::zero(),
            t: 0.0,
            front_face: false,
        };

        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let direction = rec.normal + random_unit_vector();
            let bounced = Ray::new(rec.p, direction);
            return 0.5 * self.ray_color(&bounced, world, depth - 1);
        }

        let unit_dir = r.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random() - 0.5, random() - 0.5, 0.0)
}
