use super::vector::Vector;


#[derive(Debug, Copy, Clone)]
pub struct Particle {
    position: Vector,
    velocity: Vector,
    acceleration: Vector,
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
        self.acceleration += a;

        self
    }

    pub fn calculate(&mut self, time: u128) -> &Particle {
        self.position = self.velocity * time + (self.acceleration * time * time)/2;
        self.velocity += self.acceleration * time;  

        self
    }
}
