use std::{cmp::Ordering, sync::Arc};

use crate::{aabb::AABB, interval::Interval, object::Object, object_list::ObjectList, utils::Rng};

pub struct BVHNode {
    left: Arc<dyn Object>,
    right: Arc<dyn Object>,
    bbox: AABB,
}

impl BVHNode {
    pub fn from_list(list: ObjectList, rng: &mut Rng) -> Self {
        let mut objects: Vec<Arc<dyn Object>> =
            list.objects.into_iter().map(|b| Arc::from(b)).collect();
        Self::build(&mut objects, rng)
    }
    pub fn build(objects: &mut [Arc<dyn Object>], rng: &mut Rng) -> Self {
        let comparator = match rng.random_range(0.0, 3.0) as usize {
            0 => box_x_compare,
            1 => box_y_compare,
            _ => box_z_compare,
        };

        let span = objects.len();
        let (left, right): (Arc<dyn Object>, Arc<dyn Object>) = match span {
            0 => panic!("Cannot build BVH with 0 objects"),
            1 => (Arc::clone(&objects[0]), Arc::clone(&objects[0])),
            2 => (Arc::clone(&objects[0]), Arc::clone(&objects[1])),
            _ => {
                objects.sort_by(comparator);
                let mid = span / 2;
                let (left_slice, right_slice) = objects.split_at_mut(mid);

                let left_node: Arc<dyn Object> = Arc::new(Self::build(left_slice, rng));
                let right_node: Arc<dyn Object> = Arc::new(Self::build(right_slice, rng));
                (left_node, right_node)
            }
        };

        let bbox = AABB::enclose(&left.bounding_box(), &right.bounding_box());
        Self { left, right, bbox }
    }
}

impl Object for BVHNode {
    fn hit(
        &self,
        r: &crate::ray::Ray,
        ray_t: crate::interval::Interval,
        rec: &mut crate::object::HRecord,
    ) -> bool {
        if !self.bbox.hit(r, ray_t) {
            return false;
        }
        let hit_left = self.left.hit(r, ray_t, rec);
        let ray_t_right = Interval::new(ray_t.min, if hit_left { rec.t } else { ray_t.max });
        let hit_right = self.right.hit(r, ray_t_right, rec);

        hit_left || hit_right
    }

    #[inline]
    fn bounding_box(&self) -> AABB {
        self.bbox
    }
}

fn box_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>, axis: usize) -> Ordering {
    let a_min = a.bounding_box().axis_interval(axis).min;
    let b_min = b.bounding_box().axis_interval(axis).min;
    a_min.partial_cmp(&b_min).unwrap_or(Ordering::Equal)
}

fn box_x_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    box_compare(a, b, 0)
}

fn box_y_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    box_compare(a, b, 1)
}

fn box_z_compare(a: &Arc<dyn Object>, b: &Arc<dyn Object>) -> Ordering {
    box_compare(a, b, 2)
}
