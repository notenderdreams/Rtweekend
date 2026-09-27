use std::sync::atomic::Ordering;

use crate::{
    gui::Preview,
    image::Image,
    interval::Interval,
    object::{HRecord, Object},
    ray::Ray,
    renderer,
    utils::Rng,
    vec3::{Color, Point, Vec3, random_in_unit_disk},
};

pub struct Camera {
    pub img_w: usize,
    pub img_h: usize,
    pub samples_per_pixel: usize,
    pub max_depth: usize,

    pub vfov: f32,
    pub lookfrom: Point,
    pub lookat: Point,
    pub vup: Vec3,

    pub defocus_angle: f32,
    pub focus_dist: f32,

    center: Point,
    px_del_u: Vec3,
    px_del_v: Vec3,
    px00_loc: Point,

    u: Vec3,
    v: Vec3,
    w: Vec3,

    defocus_disk_u: Vec3,
    defocus_disk_v: Vec3,
}

impl Camera {
    pub fn new(img_w: usize, img_h: usize, samples_per_pixel: usize, max_depth: usize) -> Self {
        let vfov = 90.0;
        let lookfrom = Point::zero();
        let lookat = Point::new(0.0, 0.0, -1.0);
        let vup = Vec3::new(0.0, 1.0, 0.0);
        let defocus_angle = 0.0;
        let focus_dist = 10.0;

        let (center, px_del_u, px_del_v, px00_loc, u, v, w, defocus_disk_u, defocus_disk_v) =
            Self::compute(
                img_w,
                img_h,
                vfov,
                lookfrom,
                lookat,
                vup,
                defocus_angle,
                focus_dist,
            );

        Self {
            img_w,
            img_h,
            samples_per_pixel,
            max_depth,
            vfov,
            lookfrom,
            lookat,
            vup,
            defocus_angle,
            focus_dist,
            center,
            px_del_u,
            px_del_v,
            px00_loc,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        }
    }

    #[allow(dead_code)]
    pub fn aspect_ratio(&self) -> f32 {
        self.img_w as f32 / self.img_h as f32
    }

    pub fn init(&mut self) {
        let (center, px_del_u, px_del_v, px00_loc, u, v, w, defocus_disk_u, defocus_disk_v) =
            Self::compute(
                self.img_w,
                self.img_h,
                self.vfov,
                self.lookfrom,
                self.lookat,
                self.vup,
                self.defocus_angle,
                self.focus_dist,
            );
        self.center = center;
        self.px_del_u = px_del_u;
        self.px_del_v = px_del_v;
        self.px00_loc = px00_loc;
        self.u = u;
        self.v = v;
        self.w = w;
        self.defocus_disk_u = defocus_disk_u;
        self.defocus_disk_v = defocus_disk_v;
    }

    #[allow(clippy::too_many_arguments)]
    fn compute(
        img_w: usize,
        img_h: usize,
        vfov: f32,
        lookfrom: Point,
        lookat: Point,
        vup: Vec3,
        defocus_angle: f32,
        focus_dist: f32,
    ) -> (Point, Vec3, Vec3, Point, Vec3, Vec3, Vec3, Vec3, Vec3) {
        let center = lookfrom;
        let theta = vfov.to_radians();
        let h = (theta / 2.0).tan();
        let vp_h = 2.0 * h * focus_dist;
        let vp_w = vp_h * (img_w as f32 / img_h as f32);

        let w = (lookfrom - lookat).normalize();
        let u = vup.cross(w).normalize();
        let v = w.cross(u);

        let vp_u = u * vp_w;
        let vp_v = -v * vp_h;

        let px_del_u = vp_u / img_w as f32;
        let px_del_v = vp_v / img_h as f32;

        let vp_upper_left = center - (w * focus_dist) - vp_u / 2.0 - vp_v / 2.0;
        let px00_loc = vp_upper_left + (px_del_u + px_del_v) * 0.5;

        let defocus_radius = focus_dist * (defocus_angle / 2.0).to_radians().tan();
        let defocus_disk_u = u * defocus_radius;
        let defocus_disk_v = v * defocus_radius;

        (
            center,
            px_del_u,
            px_del_v,
            px00_loc,
            u,
            v,
            w,
            defocus_disk_u,
            defocus_disk_v,
        )
    }

    pub fn render_loop(&mut self, world: &dyn Object, preview: Option<&Preview>) -> Option<Image> {
        let mut last_img: Option<Image> = None;

        loop {
            if preview.is_some_and(|p| p.abort.load(Ordering::Relaxed)) {
                return last_img;
            }

            if let Some(p) = preview {
                p.restart.store(false, Ordering::Relaxed);
                let state = p.controller.lock().unwrap().current;
                self.lookfrom = state.lookfrom;
                self.lookat = state.lookat;
                self.vup = state.vup;
                self.vfov = state.vfov;
            }

            self.init();
            let mut img = Image::new(self.img_w, self.img_h);

            let is_clay = preview.is_some_and(|p| p.clay.load(Ordering::Relaxed));
            let samples = if is_clay { 1 } else { self.samples_per_pixel };

            renderer::render(&mut img, samples, preview, |x, y, _s, rng| {
                let r = self.get_ray(x, y, rng);
                if is_clay {
                    self.clay_color(&r, world)
                } else {
                    self.ray_color(&r, world, self.max_depth, rng)
                }
            });

            last_img = Some(img);

            let Some(p) = preview else {
                return last_img;
            };

            while !p.restart.load(Ordering::Relaxed) && !p.abort.load(Ordering::Relaxed) {
                std::thread::sleep(std::time::Duration::from_millis(16));
            }
        }
    }

    fn get_ray(&self, x: usize, y: usize, rng: &mut Rng) -> Ray {
        let offset = sample_square(rng);
        let px_sample = self.px00_loc
            + self.px_del_u * (x as f32 + offset.x)
            + self.px_del_v * (y as f32 + offset.y);

        let ray_origin = if self.defocus_angle <= 0.0 {
            self.center
        } else {
            self.defocus_disk_sample(rng)
        };

        let ray_dir = px_sample - ray_origin;
        Ray::new(ray_origin, ray_dir)
    }

    fn defocus_disk_sample(&self, rng: &mut Rng) -> Point {
        let p = random_in_unit_disk(rng);
        self.center + (self.defocus_disk_u * p.x) + (self.defocus_disk_v * p.y)
    }

    fn ray_color(&self, r: &Ray, world: &dyn Object, depth: usize, rng: &mut Rng) -> Color {
        if depth == 0 {
            return Color::zero();
        }

        let mut rec = HRecord::new();

        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            if let Some(mat) = &rec.mat
                && let Some((attenuation, scattered)) = mat.scatter(r, &rec, rng)
            {
                return attenuation * self.ray_color(&scattered, world, depth - 1, rng);
            }
            return Color::zero();
        }

        let unit_dir = r.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
    fn clay_color(&self, r: &Ray, world: &dyn Object) -> Color {
        let mut rec = HRecord::new();

        if world.hit(r, Interval::new(0.001, f32::INFINITY), &mut rec) {
            let light_dir = Vec3::new(0.5, 0.8, 0.3).normalize();
            let ndotl = rec.normal.dot(light_dir).max(0.0);

            let base = Color::new(0.7, 0.7, 0.72);
            let ambient = 0.15;
            return base * (ambient + (1.0 - ambient) * ndotl);
        }

        let unit_dir = r.direction.normalize();
        let a = 0.5 * (unit_dir.y + 1.0);
        (1.0 - a) * Color::new(1.0, 1.0, 1.0) + a * Color::new(0.5, 0.7, 1.0)
    }
}

fn sample_square(rng: &mut Rng) -> Vec3 {
    Vec3::new(rng.random() - 0.5, rng.random() - 0.5, 0.0)
}
