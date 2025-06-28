use gerber_types::Coordinates;

#[derive(Debug, Default, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn new_normalize(x: f64, y: f64) -> Self {
        let len = (x * x + y * y).sqrt();
        Self {
            x: x / len,
            y: y / len,
        }
    }

    pub fn mult(&mut self, scalar: f64) {
        self.x *= scalar;
        self.y *= scalar;
    }

    pub fn inv(&mut self) {
        self.x *= -1.0;
        self.y *= -1.0;
    }

    pub fn set(&mut self, coord: &Coordinates) {
        if let Some(x) = coord.x {
            self.x = x.into();
        }

        if let Some(y) = coord.y {
            self.y = y.into();
        }
    }
}

impl std::ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::Output {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl Into<clipper2::Point> for Vec2 {
    fn into(self) -> clipper2::Point {
        clipper2::Point::new(self.x, self.y)
    }
}

impl From<&Coordinates> for Vec2 {
    fn from(coord: &Coordinates) -> Self {
        Self {
            x: if let Some(x) = coord.x { x.into() } else { 0.0 },
            y: if let Some(y) = coord.y { y.into() } else { 0.0 },
        }
    }
}
