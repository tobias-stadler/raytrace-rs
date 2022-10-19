use std::rc::Rc;

use crate::linalg::*;
use crate::tracer::*;

pub struct Sphere {
    pub origin: Vec3,
    pub radius: fVec,
    pub material: Rc<dyn Material>,
}

impl Hit for Sphere {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        let z = ray.origin - self.origin;
        let a = ray.direction * ray.direction;
        let half_b = z * ray.direction;
        let c = z * z - self.radius * self.radius;

        let disc = half_b * half_b - a * c;

        if disc < 0.0 {
            None
        } else {
            let disc_sqrt = disc.sqrt();
            let t;
            let t_near = (-half_b - disc_sqrt) / a;
            if ray.min <= t_near && t_near <= ray.max {
                t = t_near;
            } else {
                let t_far = (-half_b + disc_sqrt) / a;
                if ray.min <= t_far && t_far <= ray.max {
                    t = t_far
                } else {
                    return None;
                }
            }
            let intersect = ray.at(t);
            Some(HitResult {
                normal: (intersect - self.origin) / self.radius,
                intersect: intersect,
                at: t,
            })
        }
    }

    fn material(&self) -> &dyn Material {
        self.material.as_ref()
    }
}
