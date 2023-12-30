use super::{particle::Particle, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub enum ShapeType {
    Cube,
    Cuboid,
    Sphere,
}

#[derive(Debug, Copy, Clone)]
pub struct Object {
    pub mass: f64,
    pub shape: ShapeType,

    pub particle: Particle,
}

impl Object {
    pub fn new<T: Into<f64>>(mass: T, shape: ShapeType) -> Self {
        Self {
            mass: mass.into(),
            shape,
            particle: Particle::new(),
        }
    }

    pub fn set_position(&mut self, position: Vector) -> &mut Self {
        self.particle.position = position;

        self
    }

    pub fn force(&mut self, force: Vector) -> &mut Self {
        self.particle.accelerate(force.div(self.mass));

        self
    }

    pub fn gravity(&mut self, gravity: f64) -> &mut Self {
        if self.on_ground() {
            self
        } else {
            self.force(Vector::from(0, -gravity).mul(self.mass))
        }
    }

    pub fn friction(&mut self, s_friction: f64, k_friction: f64, gravity: f64) -> &mut Self {
        if !self.on_ground() {
            return self;
        }
        let static_friction = Vector::from(s_friction * self.mass * gravity, 0)
            .mul(-self.particle.acceleration.x_dir());
        if self.particle.acceleration.mul(self.mass).value() > static_friction.value() {
            self.force(
                Vector::from(k_friction * self.mass * gravity, 0)
                    .mul(-self.particle.acceleration.x_dir()),
            );
        } else {
            self.force(self.particle.acceleration.mul(self.mass * -1.0));
        }

        self
    }

    pub fn on_ground(&self) -> bool {
        self.particle.position.y == 0.0
    }

    pub fn calculate(&mut self, time: f64) -> &mut Self {
        self.particle.calculate(time);

        self
    }
}
