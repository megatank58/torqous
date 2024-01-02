use super::vector::Vector;


#[derive(Debug, Copy, Clone)]
pub struct Particle {
    pub position: Vector,
    pub velocity: Vector,
    pub acceleration: Vector,
}

impl Particle {
    pub fn new() -> Self {
        Self {
            position: Vector::new(),
            velocity: Vector::new(),
            acceleration: Vector::new(),
        }
    }

    pub fn accelerate(&mut self, a: Vector) -> &Particle {
        self.acceleration = self.acceleration.add(a);

        self
    }

    pub fn calculate(&mut self, time: f32) -> &Particle {
        self.position = self.position.add(self.velocity.mul(time)).add(self.acceleration.mul(time.powi(2)).div(2.0));
        self.velocity = self.velocity.add(self.acceleration.mul(time));
        self.acceleration = Vector::new();

        self
    }
}
