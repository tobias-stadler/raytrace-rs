use std::cell::RefCell;
use std::ops::DerefMut;

use crate::image::*;
use crate::linalg::*;
use crate::tracer::*;
use rand::prelude::*;

pub struct Background {
    pub color: Color,
}

impl Hit for Background {
    fn hit(&self, ray: &Ray) -> Option<HitResult> {
        if ray.max.is_infinite() {
            Some(HitResult {
                intersect: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                normal: Vec3 {
                    x: 0.,
                    y: 0.,
                    z: 0.,
                },
                at: ray.max,
            })
        } else {
            None
        }
    }

    fn material(&self) -> &dyn Material {
        self
    }
}

impl Material for Background {
    fn bounce(&self, ray: &Ray, _hit: &HitResult) -> (Color, Option<Ray>) {
        (
            self.color * ((ray.direction.unit().y + 1.0) / 2.0) as f32,
            None,
        )
    }
}

pub struct DebugMaterial {}

impl Material for DebugMaterial {
    fn bounce(&self, _ray: &Ray, hit: &HitResult) -> (Color, Option<Ray>) {
        let nor = (hit.normal + 1.0) / 2.0;
        (
            Color {
                r: nor.x as fCol,
                g: nor.y as fCol,
                b: nor.z as fCol,
            },
            None,
        )
    }
}

pub struct DiffuseMaterial {
    pub rng: Box<RefCell<dyn RngCore>>,
    pub color: Color,
}

impl Material for DiffuseMaterial {
    fn bounce(&self, ray: &Ray, hit: &HitResult) -> (Color, Option<Ray>) {
        if !hit.is_outside(ray) {
            return (Color::black(), None);
        }
        let scatter_dir = hit.normal + rand_on_unit_sphere(self.rng.borrow_mut().deref_mut());
        (
            self.color,
            Some(Ray::new(
                hit.intersect,
                if scatter_dir.is_tiny(0.0001) {
                    hit.normal
                } else {
                    scatter_dir.unit()
                },
            )),
        )
    }
}

fn rand_on_unit_sphere(rng: &mut (impl RngCore + ?Sized)) -> Vec3 {
    loop {
        let x = Vec3::random(rng, -1.0, 1.0);
        if x*x <= 1.0 {
            break x.unit();
        }
    }
}

pub struct ReflectiveMaterial {
    pub color: Color,
    pub fuzziness: fVec,
    pub rng: Box<RefCell<dyn RngCore>>,
}

impl Material for ReflectiveMaterial {
    fn bounce(&self, ray: &Ray, hit: &HitResult) -> (Color, Option<Ray>) {
        if !hit.is_outside(ray) {
            return (Color::black(), None);
        }

        let unit_dir = ray.direction.unit();
        let reflected_dir = unit_dir.reflect(hit.normal);

        let bounced_dir = if self.fuzziness < 0.01 {
            let rand_dir = rand_on_unit_sphere(self.rng.borrow_mut().deref_mut());
            let mut fuzzy_dir = reflected_dir + rand_dir * self.fuzziness;
            if fuzzy_dir * hit.normal <= 0.0 {
                let scatter_dir = hit.normal + rand_dir;
                if fuzzy_dir.is_tiny(0.001) {
                    fuzzy_dir = hit.normal;
                } else {
                    fuzzy_dir = scatter_dir.unit()
                }
            }
            fuzzy_dir
        } else {
            reflected_dir
        };

        (self.color, Some(Ray::new(hit.intersect, bounced_dir)))
    }
}

pub struct DielectricMaterial {
    pub ior: fVec,
    pub rng: Box<RefCell<dyn RngCore>>,
}

impl Material for DielectricMaterial {
    fn bounce(&self, ray: &Ray, hit: &HitResult) -> (Color, Option<Ray>) {
        let refracted =
            ray.direction
                .refract(hit.normal, self.ior, self.rng.borrow_mut().deref_mut());

        (Color::white(), Some(Ray::new(hit.intersect, refracted)))
    }
}
