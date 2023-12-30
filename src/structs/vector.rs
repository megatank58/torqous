#[derive(Debug, Copy, Clone)]
pub struct Vector {
    pub x: f64,
    pub y: f64,
}

impl Vector {
    pub fn new() -> Self {
        Vector { x: 0.0, y: 0.0 }
    }

    pub fn from<T: Into<f64>, Y: Into<f64>>(x: T, y: Y) -> Self {
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

    pub fn mul<T: Into<f64> + Copy>(mut self, other: T) -> Self {
        self.x *= other.into();
        self.y *= other.into();

        self
    }

    pub fn div<T: Into<f64> + Copy>(mut self, other: T) -> Self {
        self.x /= other.into();
        self.y /= other.into();

        self
    }

    pub fn value(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).powf(0.5)
    }

    pub fn is_none(&self) -> bool {
        self.value() == 0.0
    }

    pub fn direction(&self) -> f64 {
        self.y / self.x
    }

    pub fn x_dir(&self) -> f64 {
        if self.x > 0.0 {
            1.0
        } else {
            -1.0
        }
    }

    pub fn y_dir(&self) -> f64 {
        if self.y > 0.0 {
            1.0
        } else {
            -1.0
        }
    }
}
