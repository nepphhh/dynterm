// Local imports
mod rk4;
mod aero; 
mod vec;
mod interpolate;
mod util;
use crate::aero::{Aerofoil, Vehicle};
use crate::vec::*;
use crate::interpolate::Linear;
use crate::util::*;

// Timestep values
const N: usize = 30_000;
const NMOD: usize = 10;

fn main() {

    // Set up aero coeffs
    let cl_0012_data: Vec<(f64, f64)> = parse_string_as_csv(include_str!("../data/lift.csv"));
    let cd_0012_data: Vec<(f64, f64)> = parse_string_as_csv(include_str!("../data/drag.csv"));
    let cm_0012_data: Vec<(f64, f64)> = parse_string_as_csv(include_str!("../data/moment.csv"));

    // Make aero interpolation models
    let cl: Linear = Linear::new(&cl_0012_data);
    let cd: Linear = Linear::new(&cd_0012_data);
    let cm: Linear = Linear::new(&cm_0012_data);

    // Define the vehicle
    let mut vehicle: Vehicle = Vehicle::new(
        100_00.0,
        46.6,
        Kinematics::new(
            Vector::new(0.0, 7_300.0), 
            Angle::from_degrees(45.0)
        ),
        Kinematics::new(
            Vector::new(26.5, 45.0), 
            Angle::from_degrees(0.0)
        ),
        Aerofoil::new(
            280.0, 
            8.0, 
            Angle::from_degrees(0.0), 
            &cl, &cd, &cm
        ),
        Aerofoil::new(
            40.0, 
            4.0, 
            Angle::from_degrees(0.0),
            &cl, &cd, &cm
        )
    );

    // Set up our record of positions
    let mut data = vec![(0.0, 0.0, 0.0); N/NMOD];

    // Loop
    for i in 0..N {

        // Iterate using RK4
        vehicle.apply_dynamics(100);

        // Perform logging
        if i % NMOD == 0 {
            let datum = (vehicle.position.x(), vehicle.position.y(), vehicle.aoa().nice_deg().abs());

            println!("{:.0}: {:?}", i/NMOD, datum);

            data[i/NMOD] = datum;
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