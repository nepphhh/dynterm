// Digitized using https://apps.automeris.io/wpd/ (WebPlotDigitizer)
mod rk4;
mod aero; 
mod vec;
mod interpolate;
mod util;

use crate::aero::{Aerofoil, Vehicle};
use crate::vec::{Vector, Angle};
use crate::interpolate::Linear;
use crate::util::*;

use std::f64::consts::PI;

const G: Vector = Vector::new(0.0, -9.81); // m/s2
const N: usize = 10_000_000;
const NMOD: usize = 10_000;


fn main() {

    // Set up aero coeffs
    let cl_0012_data: Vec<(f64, f64)> = parse_string_as_csv(include_str!("../data/lift.csv"));
    let cd_0012_data: Vec<(f64, f64)> = parse_string_as_csv(include_str!("../data/drag.csv"));
    let cm_0012_data: Vec<(f64, f64)> = parse_string_as_csv(include_str!("../data/moment.csv"));

    let cl: Linear = Linear::new(&cl_0012_data);
    let cd: Linear = Linear::new(&cd_0012_data);
    let cm: Linear = Linear::new(&cm_0012_data);

    // Define the vehicle
    let mut vehicle: Vehicle = Vehicle::new(
        44.22,
        112_000.0,
        Vector::new(0.0, 6_000.0), 
        Vector::from_degrees(250.0, 0.0), 
        Aerofoil::new(226.0, Angle::from_degrees(0.0), &cl, &cd, &cm),
        Aerofoil::new(24.0, Angle::from_degrees(-1.8), &cl, &cd, &cm),
    );

    // Set up our record of positions
    let mut data = vec![(0.0, 0.0, 0.0); N/NMOD];

    for i in 0..N {

        // Aero forces
        let wing_forces: Vector = vehicle.wing.lift_force(&vehicle) + vehicle.wing.drag_force(&vehicle);
        let elev_forces: Vector = vehicle.elev.lift_force(&vehicle) + vehicle.elev.drag_force(&vehicle);

        let wing_moment: f64 = vehicle.wing.pitching_moment(&vehicle);
        let elev_moment: f64 = vehicle.elev.pitching_moment(&vehicle);
        
        // Position of control surface
        let elev_position: Vector = Vector::from_radians(vehicle.length/2.0, vehicle.orientation.rad() + PI);

        // Total aero forces
        let total_aero_force = wing_forces + elev_forces;
        let total_aero_moment = wing_moment + elev_moment + elev_position.cross(elev_forces);

        // Components?
        let orientation_unit: Vector = Vector::from_radians(1.0, vehicle.velocity.orientation().rad());
        let aero_tangent: Vector = (orientation_unit.cross(total_aero_force)) * orientation_unit;

        // Control force
        let thrust_force: Vector = Vector::from_radians(aero_tangent.magnitude(), aero_tangent.orientation().rad() + PI);

        // Body force
        let weight_force: Vector = vehicle.mass * G;

        // Apply physics
        vehicle.apply_moment(total_aero_moment);
        vehicle.apply_force(thrust_force + total_aero_force + weight_force);

        // Control
        let orientation = vehicle.orientation();
        let target = total_aero_force.orientation() + Angle::pi();
        vehicle.elev.set_pitch(Angle::from_degrees(0.1 * (target-orientation).nice_deg()));

        // Perform logging
        if i % NMOD == 0 {
            println!("Speed: {:.2} Direction: {:.2} AoA: {:.2}", vehicle.speed(), vehicle.direction().nice_deg(), vehicle.aoa().nice_deg());
            println!("Aero: ({:.0}, {:.0}) Thrust: ({:.0}, {:.0})",
                total_aero_force.magnitude(), total_aero_force.orientation().nice_deg(),
                thrust_force.magnitude(), thrust_force.orientation().nice_deg()
            );

            let resultant = thrust_force + total_aero_force;
            println!("Resultant force (Gs): ({:.2}, {:.0})",
                resultant.magnitude() / weight_force.magnitude(), resultant.orientation().nice_deg()
            );

            data[i/NMOD] = (vehicle.position.x(), vehicle.position.y(), vehicle.aoa().nice_deg().abs());
        }

        // Terminate if it hits the ground
        if vehicle.position.y() <= 0.0 { break; }
    }

    // Plot the data
    match plot_scatter(&data) {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

}