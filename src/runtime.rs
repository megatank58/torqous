use std::collections::HashMap;

use crate::structs::{object::Object, particle::Particle};


#[derive(Debug, Clone)]
pub struct Runtime<'a> {
    particles: HashMap<&'a str, Particle>,
    objects: HashMap<&'a str, Object>,
    time: u128,
    steps: u128,
}

impl<'a> Runtime<'a> {
    pub fn new() -> Self {
        Self {
            particles: HashMap::new(),
            objects: HashMap::new(),
            time: 0,
            steps: 1,
        }
    }

    pub fn get_particle(&mut self, name: &'a str) -> Option<&Particle> {
        self.particles.get(&name)
    }

    pub fn get_object(&mut self, name: &'a str) -> Option<&Object> {
        self.objects.get(&name)
    }

    pub fn set_particle(&mut self, name: &'a str, particle: Particle) {
        self.particles.insert(name, particle);
    }

    pub fn set_object(&mut self, name: &'a str, object: Object) {
        self.objects.insert(name, object);
    }

    pub fn run_for<F>(mut self, time: u128, f: F) -> Runtime<'a>
    where
        F: Fn(Runtime<'a>) -> Runtime<'a>,
    {
        for _ in 0..time {
            self = f(self);

            let mut particles = HashMap::new();
            for (name, mut particle) in self.particles {
                particles.insert(name, *particle.calculate(self.steps));
            }
            self.particles = particles;

            let mut objects = HashMap::new();
            for (name, mut object) in self.objects {
                objects.insert(name, *object.calculate(self.steps));
            }
            self.objects = objects;

            self.time += 1;
        }
        self
    }
}
