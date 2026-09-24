use crate::{
    image::Image,
    interval::Interval,
    object::{HRecord, Object},
    ray::Ray,
    renderer, utils,
    vec3::{Color, Point, Vec3},
};

pub struct Camera {
    pub aspect_ratio: f32,
    pub img_w: usize,

    img_h: usize,
    center: Point,
    px_del_u: Vec3,
    px_del_v: Vec3,
    px00_loc: Point,
}

impl Camera {
    pub fn new(aspect_ratio: f32, img_w: usize) -> Self {
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
            img_h,
            center,
            px_del_u,
            px_del_v,
            px00_loc,
        }
    }
    pub fn render(&mut self, world: &dyn Object) -> Image {
        let mut img = Image::new(self.img_w, self.img_h);

        renderer::render(&mut img, |x, y| {
            let r = self.get_ray(x, y);
            let c = self.ray_color(&r, world);
            utils::to_u8(c)
        });

        img
    }

    fn get_ray(&self, x: usize, y: usize) -> Ray {
        let px_center = self.px00_loc + self.px_del_u * x as f32 + self.px_del_v * y as f32;
        let ray_dir = px_center - self.center;
        Ray::new(self.center, ray_dir)
    }

    fn ray_color(&self, r: &Ray, world: &dyn Object) -> Color {
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
}
