// Digitized using https://apps.automeris.io/wpd/ (WebPlotDigitizer)

use std::f64::consts::PI;

mod vec;
use crate::vec::{Vector, Angle};

mod rk4;
use crate::rk4::rk4;

mod util;
use crate::util::*;

mod coeffs;
use crate::coeffs::*;

mod interpolate;
use crate::interpolate::Linear;

const G: Vector = Vector::new(0.0, -9.81); // m/s2
const N: usize = 10_000_000;
const NMOD: usize = 10_000;

struct Vehicle<'a> {
    mass: f64,
    position: Vector,
    velocity: Vector,
    orientation: Angle,
    lift: Linear<'a>,
    drag: Linear<'a>,
    moment: Linear<'a>,
}

impl<'a> Vehicle<'a> {
    fn new(mass: f64, position: Vector, velocity: Vector, orientation: Angle) -> Self {
        Vehicle { 
            mass, 
            position, 
            velocity, 
            orientation,
            lift: Linear::new(&CL_DATA),
            drag: Linear::new(&CL_DATA),
            moment: Linear::new(&CL_DATA),
        }
    }

    pub fn apply_force(&mut self, force: Vector) {

        // Apply acceleration to velocity
        let f = 
            |t: f64, _: Vector| t * force / self.mass;
        self.velocity = rk4(f, self.velocity, 0.0, 1e-2);

        // Apply velocity to position
        let f = 
            |t: f64, _: Vector| t * self.velocity;
        self.position = rk4(f, self.position, 0.0, 1e-2);
    }

    #[inline] fn direction(&self) -> Angle {
        self.velocity.orientation()
    }

    #[inline] fn speed(&self) -> f64 {
        self.velocity.magnitude()
    }

    #[inline] fn atmo_density(&self) -> f64 {
        isa_density(self.position.y()) / isa_density(0.0)
    }

    #[inline] fn angle_of_attack(&self) -> Angle {
        self.orientation - self.velocity.orientation()
    }

    fn drag_force(&self) -> Vector {
        let dynamic_pressure = 2.0 * self.atmo_density() * self.speed().powi(2);
        let drag_coeff = self.drag.interpolate(
                self.angle_of_attack().degrees()
            );
        
        Vector::from_radians(
            dynamic_pressure * drag_coeff, 
            self.direction().radians() + PI
        )
    }
    fn thrust_force(&self) -> Vector {
        Vector::from_radians(
            10_000.0, 
            self.orientation.radians()
        )
    }
    fn lift_force(&self) -> Vector {
        let dynamic_pressure = 2.0 * self.atmo_density() * self.speed().powi(2);
        let lift_coeff = self.lift.interpolate(
                self.angle_of_attack().degrees()
            );

        Vector::from_radians(
            dynamic_pressure * lift_coeff, 
            self.orientation.radians() + (PI/2.0)
        )
    }

    // fn lift_force(&self) -> Vector {
    //     // Insert your lift equation here
    // }
}

fn main() {
    let mut vehicle = Vehicle::new(
        10_000.0,
        Vector::new(0.0, 6_000.0), 
        Vector::from_degrees(250.0, 45.0), 
        Angle::from_degrees(45.0)
    );

    // Set up our log of positions
    let mut data = vec![(0.0, 0.0); N/NMOD];

    for i in 0..N {
        let weight = vehicle.mass * G;
        let drag = vehicle.drag_force();
        let thrust = vehicle.thrust_force();
        let lift = vehicle.lift_force();

        vehicle.apply_force(weight + drag + thrust + lift);
        vehicle.orientation = vehicle.velocity.orientation() + Angle::from_radians(0.01 * PI);

        if i % NMOD == 0 {
            vehicle.velocity.print("Vehicle velocity"); 
            data[i/NMOD] = (vehicle.position.x(), vehicle.position.y());
        }

        if vehicle.position.y() < 0.0 { break; }
    }

    // Plot the data
    match plot_scatter(&data) {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

}