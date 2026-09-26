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

    pub vfov: f32,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vec3,

    img_h: usize,
    center: Point,
    px_del_u: Vec3,
    px_del_v: Vec3,
    px00_loc: Point,

    u: Vec3,
    v: Vec3,
    w: Vec3,
}

impl Camera {
    pub fn new(
        aspect_ratio: f32,
        img_w: usize,
        samples_per_pixel: usize,
        max_depth: usize,
    ) -> Self {
        let img_h = ((img_w as f32 / aspect_ratio) as usize).max(1);

        let vfov = 90.0;
        let lookfrom = Point::zero();
        let lookat = Point::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);

        let (center, px_del_u, px_del_v, px00_loc, u, v, w) =
            Self::compute(img_w, img_h, vfov, lookfrom, lookat, vup);

        Self {
            aspect_ratio,
            img_w,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            img_h,
            center,
            px_del_u,
            px_del_v,
            px00_loc,
            u,
            v,
            w,
        }
    }

    pub fn init(&mut self) {
        let (center, px_del_u, px_del_v, px00_loc, u, v, w) = Self::compute(
            self.img_w,
            self.img_h,
            self.vfov,
            self.lookfrom,
            self.lookat,
            self.vup,
        );
        self.center = center;
        self.px_del_u = px_del_u;
        self.px_del_v = px_del_v;
        self.px00_loc = px00_loc;
        self.u = u;
        self.v = v;
        self.w = w;
    }

    fn compute(
        img_w: usize,
        img_h: usize,
        vfov: f32,
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
    ) -> (Point, Vec3, Vec3, Point, Vec3, Vec3, Vec3) {
        let center = lookfrom;
        let focal_len = (lookfrom - lookat).len();
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let vp_h = 2.0 * h * focal_len;
        let vp_w = vp_h * (img_w as f32 / img_h as f32);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let vp_u = u * vp_w;
        let vp_v = -v * vp_h;

        let px_del_u = vp_u / img_w as f32;
        let px_del_v = vp_v / img_h as f32;

        let vp_upper_left = center - (w * focal_len) - vp_u / 2.0 - vp_v / 2.0;
        let px00_loc = vp_upper_left + (px_del_u + px_del_v) * 0.5;

        (center, px_del_u, px_del_v, px00_loc, u, v, w)
    }

    pub fn render(&mut self, world: &dyn Object) -> Image {
        self.init();
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

        let mut rec = HRecord::new();

        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            if let Some(mat) = &rec.mat {
                if let Some((attenuation, scattered)) = mat.scatter(r, &rec) {
                    return attenuation * self.ray_color(&scattered, world, depth - 1);
                }
            }
            return Color::zero();
        }

        let unit_dir = r.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

fn sample_square() -> Vec3 {
    Vec3::new(random() - 0.5, random() - 0.5, 0.0)
}
