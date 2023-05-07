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
    moment: f64,
    wing_area: f64,
    position: Vector,
    velocity: Vector,
    orientation: Angle,
    ang_velocity: f64,
    lift: Linear<'a>,
    drag: Linear<'a>,
    pitching: Linear<'a>,
}

impl<'a> Vehicle<'a> {
    fn new(mass: f64, wing_area: f64, position: Vector, velocity: Vector, orientation: Angle) -> Self {
        Vehicle { 
            mass, 
            moment: wing_area * mass / 12.0, // L = sqrt(wing_area), so L^2 is just wing_area
            wing_area,
            position, 
            velocity, 
            orientation,
            ang_velocity: 0.0,
            lift: Linear::new(&CL_DATA),
            drag: Linear::new(&CD_DATA),
            pitching: Linear::new(&CM_DATA),
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

    pub fn apply_moment(&mut self, moment: f64) {

        // Apply ang acceleration to ang velocity
        let f = 
            |t: f64, _: f64| t * moment / self.moment;
        self.ang_velocity = rk4(f, self.ang_velocity, 0.0, 1e-2);

        // Apply ang velocity to ang position
        let f = 
            |t: f64, _: f64| t * self.ang_velocity;
        self.orientation = Angle::from_radians(
            rk4(f, self.orientation.radians(), 0.0, 1e-2)
        );
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
    #[inline] fn aoa_degrees(&self) -> f64 {
        (self.orientation - self.velocity.orientation()).degrees()
    }
    #[inline] fn dynamic_pressure(&self) -> f64 {
        0.5 * self.atmo_density() * self.speed().powi(2)
    }

    fn drag_force(&self) -> Vector {
        let drag_coeff = self.drag.interpolate(self.aoa_degrees());
        Vector::from_radians(
            self.wing_area * drag_coeff * self.dynamic_pressure(), 
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
        let lift_coeff = self.lift.interpolate(self.aoa_degrees());
        Vector::from_radians(
            self.wing_area *  lift_coeff * self.dynamic_pressure(), 
            self.orientation.radians() + (PI/2.0)
        )
    }
    fn aero_moment(&self) -> f64 {
        let pitching_coeff = self.pitching.interpolate(self.aoa_degrees());
        let chord = 1.0;
        self.wing_area * pitching_coeff * self.dynamic_pressure() * chord
    }

    // fn lift_force(&self) -> Vector {
    //     // Insert your lift equation here
    // }
}

fn main() {
    let mut vehicle = Vehicle::new(
        10_000.0,
        100.0,
        Vector::new(0.0, 6_000.0), 
        Vector::from_degrees(250.0, 45.0), 
        Angle::from_degrees(45.0)
    );

    // Set up our log of positions
    let mut data = vec![(0.0, 0.0, 0.0); N/NMOD];

    for i in 0..N {
        let weight = vehicle.mass * G;
        let aero = vehicle.drag_force() + vehicle.lift_force();
        let thrust = vehicle.thrust_force();
        let moment = vehicle.aero_moment();

        let lever_arm = Vector::from_radians(0.0, vehicle.orientation.radians() + PI);
        let torque = lever_arm.x() * aero.y() - lever_arm.y() * aero.x();

        vehicle.apply_moment(moment + torque);
        vehicle.apply_force(weight + aero + thrust);

        if i % NMOD == 0 {
            // vehicle.position.print("Vehicle position");
            // vehicle.velocity.print("Vehicle velocity");
            let mut aoa = vehicle.aoa_degrees();
            if aoa > 180.0 { aoa -= 360.0; } 
            aoa = aoa.abs();
            println!("Vehicle AoA: {}", aoa);
            data[i/NMOD] = (vehicle.position.x(), vehicle.position.y(), aoa);
        }

        if vehicle.position.y() < 0.0 { break; }
    }

    // Plot the data
    match plot_scatter(&data) {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

}