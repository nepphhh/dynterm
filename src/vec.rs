use std::f64::consts::PI;
use std::ops::{Add, AddAssign, Sub, SubAssign, Mul, Div};

#[derive(Debug, Copy, Clone)]
pub struct Vector {
    x: f64,
    y: f64,
}

impl Vector {
    pub const fn new(x: f64, y: f64) -> Self {
        Vector { x, y }
    }
    pub fn from_radians(m: f64, r: f64) -> Self {
        Vector { 
            x: m * r.cos(), 
            y: m * r.sin() 
        }
    }
    pub fn from_degrees(m: f64, r: f64) -> Self {
        Vector { 
            x: m * r.to_radians().cos(), 
            y: m * r.to_radians().sin() 
        }
    }
    pub fn unit(&self) -> Vector {
        Vector::from_radians(1.0, self.orientation().rad())
    }

    // Math
    #[inline] pub fn dot(self, other: Vector) -> f64 {
        self.x * other.x + self.y * other.y
    }

    #[inline] pub fn cross(self, other: Vector) -> f64 {
        self.x * other.y - self.y * other.x
    }

    // Getters
    #[inline] pub fn x(&self) -> f64 {
        self.x
    }
    #[inline] pub fn y(&self) -> f64 {
        self.y
    }
    #[inline] pub fn magnitude(&self) -> f64 {
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
    #[inline] pub fn orientation(&self) -> Angle {
        Angle::from_radians(self.y.atan2(self.x))
    }
    
}

// Implement Add trait for Vector
impl Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Vector {
        Vector {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}
impl AddAssign<Vector> for Vector {
    fn add_assign(self: &mut Vector, other: Vector) {
        self.x += other.x;
        self.y += other.y;
    }
}

// Implement Sub trait for Vector
impl Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Vector {
        Vector {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}
impl SubAssign<Vector> for Vector {
    fn sub_assign(self: &mut Vector, other: Vector) {
        self.x -= other.x;
        self.y -= other.y;
    }
}

// Implement Mul trait for Vector and f64 (scalar multiplication)
impl Mul<Vector> for f64 {
    type Output = Vector;

    fn mul(self, vector: Vector) -> Vector {
        Vector {
            x: vector.x * self,
            y: vector.y * self,
        }
    }
}

// Implement Div trait for Vector and f64 (scalar division)
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
        Self { radians: Angle::clamp(degrees.to_radians()) }
    }
    pub fn from_radians(radians: f64) -> Self {
        Self { radians: Angle::clamp(radians) }
    }
    pub fn from_vector(vector: &Vector) -> Self {
        vector.orientation()
    }
    pub fn pi() -> Self {
        Self { radians: PI }
    }
    pub fn unit(&self) -> Vector {
        Vector::from_radians(1.0, self.radians)
    }

    // Helper function
    fn clamp(mut radians: f64) -> f64 { 
        radians %= 2.0 * PI;
        if radians < 0.0 { radians += 2.0 * PI; }
        radians
    }

    // Getters
    #[inline] pub fn rad(&self) -> f64 {
        self.radians
    }
    #[inline] pub fn deg(&self) -> f64 {
        let mut deg = self.radians.to_degrees();
        if deg == 360.0 { deg = 0.0; }
        deg
    }
    #[inline] pub fn nice_deg(&self) -> f64 {
        let mut deg = self.radians.to_degrees();
        if deg > 180.0 { deg -= 360.0; }
        deg
    }
}

// Implement Add trait for Angle
impl Add for Angle {
    type Output = Angle;

    fn add(self, other: Angle) -> Angle {
        Angle::from_radians(self.radians + other.radians)
    }
}
impl AddAssign for Angle {
    fn add_assign(&mut self, other: Angle) {
        self.radians = Angle::clamp(self.radians + other.radians);
    }
}

// Implement Sub trait for Angle
impl Sub for Angle {
    type Output = Angle;

    fn sub(self, other: Angle) -> Angle {
        Angle::from_radians(self.radians - other.radians)
    }
}
impl SubAssign for Angle {
    fn sub_assign(&mut self, other: Angle) {
        self.radians = Angle::clamp(self.radians - other.radians);
    }
}
