#![allow(dead_code)]
mod hit;
mod image;
mod linalg;
mod material;
mod tracer;

use std::{cell::RefCell, io, rc::Rc};

use hit::*;
use image::*;
use linalg::*;
use material::*;
use rand::SeedableRng;
use tracer::*;

fn main() -> io::Result<()> {
    let renderer = Renderer::new(300, 20);
    let cam = Camera::new(
        Vec3::new(0.0, 3.0, -5.0), Vec3::new(0.0, 0.0, 2.0),
        640,
        360,
        45.0,
        0.1
    );


    let mut scene = create_scene();

    scene.add(Box::new(Background {
        color: Color::from_rgb(156, 233, 255),
    }));

    let img = renderer.render(&scene, &cam);
    img.save_bmp("outimage.bmp")?;
    Ok(())
}


fn create_scene() ->  Scene {
    let mut scene = Scene::new();

    let mat = Rc::new(DiffuseMaterial {
        rng: Box::new(RefCell::new(rand::rngs::SmallRng::from_entropy())),
        color: Color::new(0.3, 0.3, 0.3),
    });

    let mat2 = Rc::new(ReflectiveMaterial {
        color: Color::new(1.0, 1.0, 0.9),
        fuzziness: 0.0,
        rng: Box::new(RefCell::new(rand::rngs::SmallRng::from_entropy())),
    });
    let mat3 = Rc::new(DielectricMaterial {
        ior: 1.5,
        rng: Box::new(RefCell::new(rand::rngs::SmallRng::from_entropy())),
    });

    scene.add(Box::new(Sphere {
        origin: Vec3 {
            x: -2.0,
            y: 0.5,
            z: 2.0,
        },
        radius: 0.5,
        material: mat3.clone(),
    }));
    scene.add(Box::new(Sphere {
        origin: Vec3 {
            x: 0.0,
            y: 1.0,
            z: 2.0,
        },
        radius: 1.0,
        material: mat2.clone(),
    }));
    scene.add(Box::new(Sphere {
        origin: Vec3 {
            x: 1.5,
            y: 0.5,
            z: 1.5,
        },
        radius: 0.5,
        material: mat.clone(),
    }));
    scene.add(Box::new(Sphere {
        origin: Vec3 {
            x: 0.0,
            y: -100.0,
            z: 0.0,
        },
        radius: 100.0,
        material: mat.clone(),
    }));

    let mut rng = rand::rngs::SmallRng::from_entropy();
    for _ in 0..20 {
        let r = Vec3::random(&mut rng, 0.0, 1.0);
        let m = Rc::new(DiffuseMaterial {
            rng: Box::new(RefCell::new(rand::rngs::SmallRng::from_entropy())),
            color: Color::new(r.x, r.y, r.z) 
        });
        let mut pos = Vec3::random(&mut rng, -5.0, 5.0);
        
        pos.y = 0.05;
        scene.add(Box::new(Sphere {
            origin: pos,
            radius: 0.1,
            material: m
        }))

    }

    scene
}
