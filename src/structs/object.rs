use super::{particle::Particle, vector::Vector};

#[derive(Debug, Copy, Clone)]
pub enum ShapeType {
    Circle(f32),
    Triangle(f32,f32,f32),
    Quadrilateral(f32,f32,f32,f32)
}

#[derive(Debug, Copy, Clone)]
pub struct Object {
    pub mass: f32,
    pub shape: ShapeType,

    pub particle: Particle,
}

impl Object {
    pub fn new<T: Into<f32>>(mass: T, shape: ShapeType) -> Self {
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

    pub fn gravity(&mut self, gravity: f32) -> &mut Self {
        if self.on_ground() {
            self
        } else {
            self.force(Vector::from(0.0, -gravity).mul(self.mass))
        }
    }

    pub fn friction(&mut self, s_friction: f32, k_friction: f32, gravity: f32) -> &mut Self {
        if !self.on_ground() {
            return self;
        }

        let static_friction = Vector::from(s_friction * self.mass * gravity, 0.0)
            .mul(-self.particle.acceleration.x_dir());

        if !self.particle.velocity.is_none() || self.particle.acceleration.mul(self.mass).value() > static_friction.value() {
            self.force(
                Vector::from(k_friction * self.mass * gravity, 0.0)
                    .mul(-self.particle.velocity.x_dir()),
            );
        } else {
            self.force(self.particle.acceleration.mul(-self.mass));
        }

        self
    }

    pub fn on_ground(&self) -> bool {
        self.particle.position.y <= 0.0
    }

    pub fn calculate(&mut self, time: f32) -> &mut Self {
        self.particle.calculate(time);

        if self.on_ground() {
            self.particle.velocity.y = 0.0;
            self.particle.position.y = 0.0;
        }

        self
    }
}
