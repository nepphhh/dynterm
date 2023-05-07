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

    let mut vehicle: Vehicle = Vehicle::new(
        100.0,
        10_000.0,
        Vector::new(0.0, 6_000.0), 
        Vector::from_degrees(250.0, 45.0), 
        Aerofoil::new(20.0, Angle::from_degrees(0.0), &cl, &cd, &cm),
        Aerofoil::new(1.0, Angle::from_degrees(-5.0), &cl, &cd, &cm),
    );

    // Set up our log of positions
    let mut data = vec![(0.0, 0.0, 0.0); N/NMOD];

    for i in 0..N {

        let weight_force: Vector = vehicle.mass * G;
        let thrust_force: Vector = Vector::from_radians(5_000.0, vehicle.orientation.rad());

        let wing_forces: Vector  = vehicle.wing.lift_force(&vehicle) + vehicle.wing.drag_force(&vehicle);
        let elev_forces: Vector  = vehicle.elev.lift_force(&vehicle) + vehicle.elev.drag_force(&vehicle);

        let wing_moment: f64 = vehicle.wing.pitching_moment(&vehicle);
        let elev_moment: f64 = vehicle.elev.pitching_moment(&vehicle);

        let elev_position: Vector = Vector::from_radians(vehicle.length/2.0, vehicle.orientation.rad() + PI);

        vehicle.apply_moment(wing_moment + elev_moment + elev_position.cross(elev_forces));
        vehicle.apply_force(weight_force + thrust_force + wing_forces + elev_forces);

        if i % NMOD == 0 {
            let mut dir = vehicle.direction().deg();
            if dir > 180.0 { dir -= 360.0; dir = dir.abs(); }

            let mut aoa = vehicle.wing.aoa(&vehicle).deg();
            if aoa > 180.0 { aoa -= 360.0; aoa = aoa.abs(); }

            println!("Speed: {:.2} Direction: {:.2} AoA: {:.2}", vehicle.speed(), dir, aoa);

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