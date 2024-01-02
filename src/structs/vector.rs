#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub fn new() -> Self {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn from<T: Into<f32>, Y: Into<f32>>(x: T, y: Y) -> Self {
        Vector {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn add(mut self, other: Self) -> Self {
        self.x += other.x;
        self.y += other.y;

        self
    }

    pub fn mul(mut self, other: f32) -> Self {
        self.x *= other;
        self.y *= other;

        self
    }

    pub fn div(mut self, other: f32) -> Self {
        self.x /= other;
        self.y /= other;

        self
    }

    pub fn value(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    pub fn is_none(&self) -> bool {
        self.value() == 0.0
    }

    pub fn direction(&self) -> f32 {
        self.y / self.x
    }

    pub fn x_dir(&self) -> f32 {
        if self.x > 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn y_dir(&self) -> f32 {
        if self.y > 0.0 {
            1.0
        } else {
            -1.0
        }
    }
}
