use super::{vector::Vector, particle::Particle};

#[derive(Debug, Copy, Clone)]
pub enum ShapeType {
    Cube,
    Cuboid,
    Sphere,
}


#[derive(Debug, Copy, Clone)]
pub struct Object {
    mass: u128,
    shape: ShapeType,

    particle: Particle,
}

impl Object {
    pub fn new(mass: u128, shape: ShapeType) -> Self {
        Self {
            mass,
            shape,
            particle: Particle::new(),
        }
    }

    pub fn force(&mut self, force: Vector) -> &Object {
        self.particle.accelerate(force/self.mass);
        
        self
    }

    pub fn calculate(&mut self, time: u128) -> &Object {
        self.particle.calculate(time);

        self
    }
}