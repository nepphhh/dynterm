mod vec;
mod rk4;

use crate::vec::{Vector, Angle};

const G: f64 = 9.81; // m/s2

struct Vehicle {
    position: Vector,
    velocity: Vector,
    orientation: Angle,
}

impl Vehicle {
    fn new(position: Vector, velocity: Vector, orientation: Angle) -> Self {
        Vehicle { position, velocity, orientation }
    }

    fn altitude(&self) -> &f64 {
        self.position.y()
    }

    fn speed(&self) -> &f64 {
        &self.velocity.magnitude()
    }

    fn angle_of_attack(&self) -> Angle {
        self.orientation - self.velocity.orientation()
    }

    fn drag_force(&self, angle_of_attack: f64, air_density: f64) -> f64 {
        // Insert your drag equation here
    }

    fn lift_force(&self, angle_of_attack: f64, air_density: f64) -> f64 {
        // Insert your lift equation here
    }
}

fn main() {
    let vehicle = 
    Vehicle::new(
        Vector::new(0.0, 10_000.0), 
        Vector::new(200.0, 200.0), 
        Angle::from_degrees(45.0)
    );
}