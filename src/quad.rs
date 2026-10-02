use std::sync::Arc;

use crate::{
    aabb::AABB,
    interval::Interval,
    material::Material,
    object::{HRecord, Object},
    object_list::ObjectList,
    ray::Ray,
    vec3::{Point, Vec3},
};

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum PlanarShape {
    Parallelogram,
    Triangle,
    Ellipse,
    Annulus { inner_radius: f32 },
}

pub struct Quad {
    q: Point,
    u: Vec3,
    v: Vec3,
    w: Vec3,
    mat: Arc<dyn Material>,
    bbox: AABB,
    normal: Vec3,
    d: f32,
    shape: PlanarShape,
}

impl Quad {
    /// Parallelogram with starting corner Q and side vectors u and v.
    pub fn new(q: Point, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        Self::with_shape(q, u, v, mat, PlanarShape::Parallelogram)
    }

    /// Triangle with starting corner Q and side vectors u and v.
    /// Vertices are Q, Q + u, and Q + v.
    pub fn triangle(q: Point, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Self {
        Self::with_shape(q, u, v, mat, PlanarShape::Triangle)
    }

    /// Ellipse or circular disk centered at `center`, with semi-axes `side_a` and `side_b`.
    /// When side_a and side_b are orthogonal and equal length, this forms a circular disk.
    pub fn ellipse(center: Point, side_a: Vec3, side_b: Vec3, mat: Arc<dyn Material>) -> Self {
        let q = center - side_a - side_b;
        let u = 2.0 * side_a;
        let v = 2.0 * side_b;
        Self::with_shape(q, u, v, mat, PlanarShape::Ellipse)
    }

    /// Annulus (Ring / Washer) centered at `center`, with semi-axes `side_a` and `side_b`,
    /// and normalized inner_radius in [0, 1] relative to the outer edge.
    pub fn annulus(
        center: Point,
        side_a: Vec3,
        side_b: Vec3,
        inner_radius: f32,
        mat: Arc<dyn Material>,
    ) -> Self {
        let q = center - side_a - side_b;
        let u = 2.0 * side_a;
        let v = 2.0 * side_b;
        Self::with_shape(q, u, v, mat, PlanarShape::Annulus { inner_radius })
    }

    pub fn with_shape(
        q: Point,
        u: Vec3,
        v: Vec3,
        mat: Arc<dyn Material>,
        shape: PlanarShape,
    ) -> Self {
        let n = u.cross(v);
        let normal = n.normalize();
        let d = normal.dot(q);
        let w = n / n.len_squared();

        let bbox_diagonal1 = AABB::from_points(q, q + u + v);
        let bbox_diagonal2 = AABB::from_points(q + u, q + v);
        let bbox = AABB::enclose(&bbox_diagonal1, &bbox_diagonal2).pad();

        Self {
            q,
            u,
            v,
            w,
            mat,
            bbox,
            normal,
            d,
            shape,
        }
    }

    pub fn is_interior(&self, a: f32, b: f32, rec: &mut HRecord) -> bool {
        match self.shape {
            PlanarShape::Parallelogram => {
                let unit_interval = Interval::new(0.0, 1.0);
                if !unit_interval.contains(a) || !unit_interval.contains(b) {
                    return false;
                }
                rec.u = a;
                rec.v = b;
                true
            }
            PlanarShape::Triangle => {
                if a < 0.0 || b < 0.0 || (a + b) > 1.0 {
                    return false;
                }
                rec.u = a;
                rec.v = b;
                true
            }
            PlanarShape::Ellipse => {
                let a_center = a - 0.5;
                let b_center = b - 0.5;
                if 4.0 * (a_center * a_center + b_center * b_center) > 1.0 {
                    return false;
                }
                rec.u = a;
                rec.v = b;
                true
            }
            PlanarShape::Annulus { inner_radius } => {
                let a_center = a - 0.5;
                let b_center = b - 0.5;
                let r2 = 4.0 * (a_center * a_center + b_center * b_center);
                if r2 < (inner_radius * inner_radius) || r2 > 1.0 {
                    return false;
                }
                rec.u = a;
                rec.v = b;
                true
            }
        }
    }
}

impl Object for Quad {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let denom = self.normal.dot(r.direction);

        if denom.abs() < 1e-8 {
            return false;
        }

        let t = (self.d - self.normal.dot(r.origin)) / denom;
        if !ray_t.contains(t) {
            return false;
        }

        let intersection = r.at(t);
        let planar_hitpt = intersection - self.q;
        let alpha = self.w.dot(planar_hitpt.cross(self.v));
        let beta = self.w.dot(self.u.cross(planar_hitpt));

        if !self.is_interior(alpha, beta, rec) {
            return false;
        }

        rec.t = t;
        rec.p = intersection;
        rec.mat = Some(Arc::clone(&self.mat));
        rec.set_face_normal(r, self.normal);
        true
    }

    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

pub struct Triangle;
impl Triangle {
    pub fn new(q: Point, u: Vec3, v: Vec3, mat: Arc<dyn Material>) -> Quad {
        Quad::triangle(q, u, v, mat)
    }
}

pub struct Ellipse;
impl Ellipse {
    pub fn new(center: Point, side_a: Vec3, side_b: Vec3, mat: Arc<dyn Material>) -> Quad {
        Quad::ellipse(center, side_a, side_b, mat)
    }
}

pub struct Annulus;
impl Annulus {
    pub fn new(
        center: Point,
        side_a: Vec3,
        side_b: Vec3,
        inner_radius: f32,
        mat: Arc<dyn Material>,
    ) -> Quad {
        Quad::annulus(center, side_a, side_b, inner_radius, mat)
    }
}

pub fn box_primitive(a: Point, b: Point, mat: Arc<dyn Material>) -> Arc<dyn Object> {
    let mut sides = ObjectList::new();

    // Construct extrema points
    let min = Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let dx = Vec3::new(max.x - min.x, 0.0, 0.0);
    let dy = Vec3::new(0.0, max.y - min.y, 0.0);
    let dz = Vec3::new(0.0, 0.0, max.z - min.z);

    // Front (+Z)
    sides.add(Box::new(Quad::new(
        Point::new(min.x, min.y, max.z),
        dx,
        dy,
        mat.clone(),
    )));
    // Right (+X)
    sides.add(Box::new(Quad::new(
        Point::new(max.x, min.y, max.z),
        -dz,
        dy,
        mat.clone(),
    )));
    // Back (-Z)
    sides.add(Box::new(Quad::new(
        Point::new(max.x, min.y, min.z),
        -dx,
        dy,
        mat.clone(),
    )));
    // Left (-X)
    sides.add(Box::new(Quad::new(
        Point::new(min.x, min.y, min.z),
        dz,
        dy,
        mat.clone(),
    )));
    // Top (+Y)
    sides.add(Box::new(Quad::new(
        Point::new(min.x, max.y, max.z),
        dx,
        -dz,
        mat.clone(),
    )));
    // Bottom (-Y)
    sides.add(Box::new(Quad::new(
        Point::new(min.x, min.y, min.z),
        dx,
        dz,
        mat,
    )));

    Arc::new(sides)
}
