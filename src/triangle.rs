use std::sync::Arc;

use crate::{
    interval::Interval, material::Material, object::{HRecord, Object}, object_list::ObjectList, ray::Ray, vec3::{Point, Vec3}
};

#[allow(dead_code)]
pub struct Triangle {
    pub v0: Point,
    pub v1: Point,
    pub v2: Point,

    pub n0: Option<Vec3>,
    pub n1: Option<Vec3>,
    pub n2: Option<Vec3>,

    pub mat: Arc<dyn Material>,
}

#[allow(dead_code)]
impl Triangle {
    pub fn new(v0: Point, v1: Point, v2: Point, mat: Arc<dyn Material>) -> Self {
        Self {
            v0,
            v1,
            v2,
            n0: None,
            n1: None,
            n2: None,
            mat,
        }
    }
    pub fn new_with_normals(
        v0: Point,
        v1: Point,
        v2: Point,
        n0: Vec3,
        n1: Vec3,
        n2: Vec3,
        mat: Arc<dyn Material>,
    ) -> Self {
        Self {
            v0,
            v1,
            v2,
            n0: Some(n0),
            n1: Some(n1),
            n2: Some(n2),
            mat,
        }
    }

    pub fn from_quad(
        v0: Point,
        v1: Point,
        v2: Point,
        v3: Point,
        mat: Arc<dyn Material>,
    ) -> [Self; 2] {
        [
            Self::new(v0, v1, v2, Arc::clone(&mat)),
            Self::new(v0, v2, v3, mat),
        ]
    }

    pub fn make_box(a: Point, b: Point, mat: Arc<dyn Material>) -> ObjectList {
        make_box(a, b, mat)
    }

    pub fn face_normal(&self) -> Vec3 {
        (self.v1 - self.v0).cross(self.v2 - self.v0).normalize()
    }
}

impl Object for Triangle {
    fn hit(&self, r: &Ray, ray_t: Interval, rec: &mut HRecord) -> bool {
        let e1 = self.v1 - self.v0;
        let e2 = self.v2 - self.v0;

        let pvec = r.direction.cross(e2);
        let det = e1.dot(pvec);

        if det.abs() < 1e-8 {
            return false;
        }

        let inv_det = 1.0 / det;
        let tvec = r.origin - self.v0;

        let u = tvec.dot(pvec) * inv_det;
        if !(0.0..=1.0).contains(&u) {
            return false;
        }

        let qvec = tvec.cross(e1);

        let v = r.direction.dot(qvec) * inv_det;
        if v < 0.0 || (u + v) > 1.0 {
            return false;
        }

        let t = e2.dot(qvec) * inv_det;
        if !ray_t.contains(t) {
            return false;
        }

        rec.t = t;
        rec.p = r.at(t);
        rec.mat = Some(Arc::clone(&self.mat));

        let outward_normal = match (self.n0, self.n1, self.n2) {
            (Some(n0), Some(n1), Some(n2)) => {
                let w = 1.0 - u - v;
                (w * n0 + u * n1 + v * n2).normalize()
            }
            _ => self.face_normal(),
        };
        rec.set_face_normal(r, outward_normal);
        true
    }
}

#[allow(dead_code)]
pub fn make_box(a: Point, b: Point, mat: Arc<dyn Material>) -> ObjectList {
    let mut sides = ObjectList::new();

    let min = Point::new(a.x.min(b.x), a.y.min(b.y), a.z.min(b.z));
    let max = Point::new(a.x.max(b.x), a.y.max(b.y), a.z.max(b.z));

    let p000 = Point::new(min.x, min.y, min.z);
    let p001 = Point::new(min.x, min.y, max.z);
    let p010 = Point::new(min.x, max.y, min.z);
    let p011 = Point::new(min.x, max.y, max.z);
    let p100 = Point::new(max.x, min.y, min.z);
    let p101 = Point::new(max.x, min.y, max.z);
    let p110 = Point::new(max.x, max.y, min.z);
    let p111 = Point::new(max.x, max.y, max.z);

    for tri in Triangle::from_quad(p001, p101, p111, p011, Arc::clone(&mat)) {
        sides.add(Box::new(tri));
    }
    for tri in Triangle::from_quad(p100, p000, p010, p110, Arc::clone(&mat)) {
        sides.add(Box::new(tri));
    }
    for tri in Triangle::from_quad(p011, p111, p110, p010, Arc::clone(&mat)) {
        sides.add(Box::new(tri));
    }
    for tri in Triangle::from_quad(p000, p100, p101, p001, Arc::clone(&mat)) {
        sides.add(Box::new(tri));
    }
    for tri in Triangle::from_quad(p101, p100, p110, p111, Arc::clone(&mat)) {
        sides.add(Box::new(tri));
    }
    for tri in Triangle::from_quad(p000, p001, p011, p010, Arc::clone(&mat)) {
        sides.add(Box::new(tri));
    }

    sides
}
