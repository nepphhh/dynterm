mod vec;
mod rk4;

use crate::vec::{Vector, Angle};
use crate::rk4::rk4;

const G: Vector = Vector::new(0.0, -9.81); // m/s2

struct Vehicle {
    mass: f64,
    position: Vector,
    velocity: Vector,
    orientation: Angle,
}

impl Vehicle {
    fn new(mass: f64, position: Vector, velocity: Vector, orientation: Angle) -> Self {
        Vehicle { mass, position, velocity, orientation }
    }

    pub fn apply_force(&mut self, force: Vector) {

        // Apply acceleration to velocity
        let f = 
            |t: f64, _: Vector| t * force / self.mass;
        self.velocity = rk4(f, self.velocity, 0.0, 1e-3);

        // Apply velocity to position
        let f = 
            |t: f64, _: Vector| t * self.velocity;
        self.position = rk4(f, self.position, 0.0, 1e-3);
    }

    // IO functions
    pub fn print_position(&self) {
        println!("Vehicle position: ({:.2}, {:.2})", self.position.x(), self.position.y());
    }
    pub fn print_velocity(&self) {
        println!("Vehicle velocity: ({:.2}, {:.2})", self.velocity.x(), self.velocity.y());
    }

    // #[inline] fn altitude(&self) -> f64 {
    //     self.position.y()
    // }

    // #[inline] fn speed(&self) -> f64 {
    //     self.velocity.magnitude()
    // }

    // #[inline] fn angle_of_attack(&self) -> Angle {
    //     self.orientation - self.velocity.orientation()
    // }

    // fn drag_force(&self, angle_of_attack: f64, air_density: f64) -> f64 {
    //     // Insert your drag equation here
    // }

    // fn lift_force(&self, angle_of_attack: f64, air_density: f64) -> f64 {
    //     // Insert your lift equation here
    // }
}

fn main() {
    let mut vehicle = Vehicle::new(
        10_000.0,
        Vector::new(0.0, 10_000.0), 
        Vector::new(200.0, 200.0), 
        Angle::from_degrees(45.0)
    );

    for i in 0..1_000_000_000 {
        vehicle.apply_force(vehicle.mass * G);

        if i % 100_000 == 0 { vehicle.print_position(); }

        if vehicle.position.y() < 0.0 { break; }
    }

    vehicle.print_velocity();
}