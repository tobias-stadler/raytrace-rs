use rand::prelude::*;
use std::io::stdout;
use std::io::Write;

use crate::image::*;
use crate::linalg::*;

#[derive(Clone, Copy)]
pub struct HitResult {
    pub intersect: Vec3,
    pub normal: Vec3,
    pub at: fVec,
}
pub trait Material {
    fn bounce(&self, ray: &Ray, hit: &HitResult) -> (Color, Option<Ray>);
}

pub trait Hit {
    fn hit(&self, ray: &Ray) -> Option<HitResult>;
    fn material(&self) -> &dyn Material;
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub struct Ray {
    pub origin: Vec3,
    pub direction: Vec3,
    pub min: fVec,
    pub max: fVec,
}

impl Ray {
    #[inline]
    pub fn at(&self, pos: fVec) -> Vec3 {
        self.origin + self.direction * pos
    }

    #[inline]
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin: origin,
            direction: direction,
            min: 0.001,
            max: fVec::INFINITY,
        }
    }
}

impl HitResult {
    #[inline]
    pub fn is_outside(&self, ray: &Ray) -> bool {
        let dot = self.normal * ray.direction;
        dot < 0.0
    }

    #[inline]
    pub fn surface_normal(&self, ray: &Ray) -> Vec3 {
        if self.is_outside(ray) {
            self.normal
        } else {
            -self.normal
        }
    }
}

//Left handed coordinate system
//u,v start from Top-Left
pub struct Camera {
    pub origin: Vec3,
    pub direction: Vec3,
    pub viewport_width: fVec,
    pub viewport_height: fVec,
    pub rasterize_width: usize,
    pub rasterize_height: usize,
    pub aperture: fVec,
    temp_right: Vec3,
    temp_up: Vec3,
}

impl Camera {
    pub fn new(look_from: Vec3, look_at: Vec3, width: usize, height: usize, fov: fVec, aperture: fVec) -> Self {
        let dir = (look_at - look_from).unit();
        let temp_right = Vec3::unit_y().cross(dir).unit();
        let temp_up = dir.cross(temp_right).unit();
        let focus_distance = (look_at - look_from).length();
        let v_width = fVec::tan((fov*std::f32::consts::PI)/(360.0))*focus_distance*2.0;

        Self {
            origin: look_from,
            direction: dir * focus_distance,
            viewport_width: v_width,
            viewport_height: (height as fVec / width as fVec) * v_width,
            rasterize_width: width,
            rasterize_height: height,
            temp_right: temp_right,
            temp_up: temp_up,
            aperture: aperture,
        }
    }

    #[inline]
    pub fn ray_through(&self, u: usize, v: usize, offset_origin: (fVec, fVec), offset_target: (fVec, fVec)) -> Ray {
        let u_step = self.viewport_width / self.rasterize_width as fVec;
        let v_step = self.viewport_height / self.rasterize_height as fVec;
        let top_left = self.origin + self.direction
            + self.temp_right * (self.viewport_width / -2.0)
            + self.temp_up * (self.viewport_height / 2.0);

        let from = self.origin + self.temp_up * (offset_origin.0*self.aperture) + self.temp_right * (offset_origin.1*self.aperture);
        let to = top_left + self.temp_right * (u_step * (u as fVec + offset_target.0)) + (-self.temp_up) * (v_step * (v as fVec + offset_target.1));
        Ray::new(
            from,
            to - from
        )
    }
}

pub struct Scene {
    objects: Vec<Box<dyn Hit>>,
}

impl Scene {
    pub fn new() -> Scene {
        Scene {
            objects: Vec::new(),
        }
    }

    pub fn add(&mut self, obj: Box<dyn Hit>) {
        self.objects.push(obj);
    }

    fn hit(&self, ray: &Ray) -> Option<(HitResult, &dyn Hit)> {
        let mut temp_ray = *ray;
        let mut hit_res = None;

        for obj in self.objects.iter() {
            let res = obj.hit(&temp_ray);
            match res {
                None => {}
                Some(r) => {
                    hit_res = Some((r, obj.as_ref()));
                    temp_ray.max = r.at;
                }
            }
        }

        hit_res
    }
}

pub struct Renderer {
    samples: usize,
    bounces: usize,
}

impl Renderer {
    pub fn new(samples: usize, bounces: usize) -> Renderer {
        Renderer {
            samples: samples,
            bounces: bounces,
        }
    }

    pub fn render(&self, scene: &Scene, cam: &Camera) -> Image {
        let width = cam.rasterize_width;
        let height = cam.rasterize_height;
        let samples = self.samples;

        let mut img = Image::new(width, height);
        let mut rng = rand::rngs::SmallRng::from_entropy();

        for y in 0..height {
            print!("\rCurrent line: {}", y);
            stdout().flush().unwrap();
            for x in 0..width {
                let mut sum = Color::black();
                let px = img.px_mut(x, y).unwrap();

                for _ in 0..samples {
                    let rnum: fVec = rng.gen_range(0.0..1.0);
                    let rnum2: fVec = rng.gen_range(0.0..1.0);

                    let ray = cam.ray_through(x, y, rand_on_unit_disc(&mut rng),(rnum, rnum2));

                    sum = sum + self.colorize_ray(scene, &ray, self.bounces);
                }
                *px = (sum * (1.0 / samples as f32)).gamma2().into();
            }
        }

        img
    }

    fn colorize_ray(&self, scene: &Scene, ray: &Ray, bounces: usize) -> Color {
        if bounces <= 0 {
            return Color::from_rgb(245, 66, 129);
        }

        let res = scene.hit(ray);
        match res {
            Some((r, obj)) => {
                let (col, bounced_ray) = obj.material().bounce(ray, &r);
                if let Some(b) = bounced_ray {
                    col * self.colorize_ray(scene, &b, bounces - 1)
                } else {
                    col
                }
            }
            None => Color::black(),
        }
    }
}

fn rand_on_unit_disc(rng: &mut impl RngCore ) -> (fVec, fVec) {
    loop {
        let x:(fVec, fVec) = (rng.gen_range(0.0..1.0), rng.gen_range(0.0..1.0));
        if x.0 * x.0 + x.1 * x.1 <= 1.0 {
            break x;
        }
    }
}