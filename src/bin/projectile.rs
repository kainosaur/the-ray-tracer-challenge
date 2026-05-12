use std::ops::Add;

use ray_tracer::features::tuple::{Point, Vector};
use ray_tracer::features::operators::Normalize;

#[derive(Clone, Copy, Debug)]
struct Projectile {
    pub position: Point<f32>,
    pub velocity: Vector<f32>,
}

impl Projectile {
    fn new(position: Point<f32>, velocity: Vector<f32>) -> Self {
        Projectile { position, velocity }
    }
    fn tick(&self, env: Environment) -> Self {
        let position = self.position.add(self.velocity);
        let velocity = self.velocity.add(env.gravity).add(env.wind);
        return Projectile { position, velocity }
    }
}

#[derive(Clone, Copy, Debug)]
struct Environment {
    pub gravity: Vector<f32>,
    pub wind: Vector<f32>
}

impl Environment {
    fn new(gravity: Vector<f32>, wind: Vector<f32>) -> Self {
        Environment { gravity, wind }
    }
}

fn main() {
    let start_pos = Point::new(0., 1., 0.);
    let start_velocity = Vector::new(1.,1., 0.).normalize();
    let gravity = Vector::new(0., -0.1, 0.);
    let wind = Vector::new(-0.01, 0., 0.);

    let mut p = Projectile::new(start_pos, start_velocity);
    let e = Environment::new(gravity, wind);

    loop {
        println!("{:#?}", p);
        p = p.tick(e);
        if p.position.y() <= 0. {
            break;
        }
    }
}