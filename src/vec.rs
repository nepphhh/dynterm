use std::f64::consts::PI;
use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }

    // Getters
    pub fn x(&self) -> &f64 {
        &self.x
    }
    pub fn y(&self) -> &f64 {
        &self.y
    }
    pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    pub fn orientation(&self) -> Angle {
        Angle::from_radians(self.y.atan2(self.x))
    }
}

// Implement Add trait for Vector2d
impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

// Implement Sub trait for Vector2d
impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

// Implement Mul trait for Vector2d (dot product)
impl Mul<Vector> for Vector {
    type Output = f64;

    fn mul(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }
}

// Implement Mul trait for Vector2d and f64 (scalar multiplication)
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self,
            y: vector.y * self,
        }
    }
}

// Implement Div trait for Vector2d and f64 (scalar division)
impl Div<f64> for Vector {
    type Output = Vector;

    fn div(self, scalar: f64) -> Vector {
        Vector {
            x: self.x / scalar,
            y: self.y / scalar,
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Angle {
    radians: f64,
}

impl Angle {

    // Constructors
    pub fn from_degrees(degrees: f64) -> Self {
        let radians = degrees.to_radians() % (2.0 * PI);
        if radians < 0.0 { radians += 2.0 * PI; }
        Self { radians }
    }
    pub fn from_radians(radians: f64) -> Self {
        radians %= (2.0 * PI);
        if radians < 0.0 { radians += 2.0 * PI; }
        Self { radians }
    }
    pub fn from_vector(vector: &Vector) -> Self {
        vector.orientation()
    }

    // Getters
    pub fn radians(&self) -> &f64 {
        &self.radians
    }
    pub fn degrees(&self) -> f64 {
        self.radians.to_degrees()
    }
}

// Implement Add trait for Angle
impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle::from_radians(self.radians + other.radians)
    }
}

// Implement Sub trait for Angle
impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        Angle::from_radians(self.radians - other.radians)
    }
}
