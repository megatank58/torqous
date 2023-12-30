use std::collections::HashMap;

use crate::structs::{object::Object, particle::Particle};


#[derive(Debug, Clone)]
pub struct Runtime<'a> {
    particles: HashMap<&'a str, Particle>,
    objects: HashMap<&'a str, Object>,
    k_friction: f64,
    s_friction: f64,
    gravity: f64,
    time: f64,
    steps: f64,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
            objects: HashMap::new(),
            k_friction: 0.0,
            s_friction: 0.0,
            gravity: 10.0,
            time: 0.0,
            steps: 1.0,
        }
    }

    pub fn get_particle(&mut self, name: &'a str) -> Option<&Particle> {
        self.particles.get(&name)
    }

    pub fn get_object(&mut self, name: &'a str) -> Option<&Object> {
        self.objects.get(&name)
    }

    pub fn set_particle(&mut self, name: &'a str, particle: Particle) -> &mut Self {
        self.particles.insert(name, particle);

        self
    }

    pub fn set_object(&mut self, name: &'a str, object: Object) -> &mut Self {
        self.objects.insert(name, object);

        self
    }

    pub fn set_k_friction(&mut self, k_friction: f64) -> &mut Self {
        self.k_friction = k_friction;

        self
    }

    pub fn set_s_friction(&mut self, s_friction: f64) -> &mut Self {
        self.s_friction = s_friction;

        if self.k_friction == 0.0 {
            self.k_friction = s_friction;
        }

        self
    }

    pub fn set_gravity(&mut self, gravity: f64) -> &mut Self {
        self.gravity = gravity;

        self
    }

    pub fn run(self, time: f64) -> Self {
        self.run_for(time, |a| a)
    }

    pub fn run_for<F>(mut self, time: f64, f: F) -> Self
    where
        F: Fn(Runtime<'a>) -> Runtime<'a>,
    {
        while self.time != time {
            self = f(self);

            let mut particles = HashMap::new();
            for (name, mut particle) in self.particles {
                particles.insert(name, *particle.calculate(self.steps));
            }
            self.particles = particles;

            let mut objects = HashMap::new();
            for (name, mut object) in self.objects {
                object.gravity(self.gravity);
                object.friction(self.s_friction, self.k_friction, self.gravity);
                objects.insert(name, *object.calculate(self.steps));
            }
            self.objects = objects;

            self.time += self.steps;
        }
        self
    }
}
