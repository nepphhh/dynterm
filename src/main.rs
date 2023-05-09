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
const NMOD: usize = 1;

fn main() {

    // Set up aero coeffs
    let cl_0012_data: Vec<(f64, f64)> = 
        parse_string_as_csv(include_str!("../data/lift.csv"));
    let cd_0012_data: Vec<(f64, f64)> = 
        parse_string_as_csv(include_str!("../data/drag.csv"));
    let cm_0012_data: Vec<(f64, f64)> = 
        parse_string_as_csv(include_str!("../data/moment.csv"));

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
    let mut data = vec![(0.0, 7_300.0, 0.0); N/NMOD];

    // Set up other trackers
    let mut aoa = vec![(0.0, 0.0, 0.0); N/NMOD];
    let mut omega = vec![(0.0, 0.0, 0.0); N/NMOD];
    let mut dx   = vec![(0.0, 0.0, 0.0); N/NMOD];
    let mut dy   = vec![(0.0, 0.0, 0.0); N/NMOD];


    // Loop
    for second in 0..N {

        // Iterate using RK4
        vehicle.apply_dynamics(100);

        // Perform logging
        if second % NMOD == 0 {

            let index = second/NMOD;

            let datum = (
                vehicle.position.x(), 
                vehicle.position.y(), 
                vehicle.aoa().nice_deg().abs()
            );

            println!("{:.0}: {:?}", second/NMOD, datum);

            data[index] = datum;
            aoa[index] = (second as f64, vehicle.aoa().nice_deg(), 0.0);
            omega[index] = (second as f64, vehicle.motion.angle().nice_deg(), 0.0);
            dx[index] = (second as f64, vehicle.motion.x(), 0.0);
            dy[index] = (second as f64, vehicle.motion.y(), 0.0);
        }

        // Terminate if it hits the ground
        if vehicle.position.y() <= 7_300.0 { break; }
    }

    // Plot the data
    match plot_scatter(
        "Trajectory", 
        "Distance [m]", 
        "Altitude [m]",
        true, 
        &data) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Angle of Attack", 
        "Time [s]",
        "Angle [deg]",
        false, 
        &aoa) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Angular Velocity",
        "Time [s]",
        "Rotation [deg/s]", 
        false, 
        &omega) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Horizontal Velocity",
        "Time [s]",
        "Velocity [m/s]", 
        false, 
        &dx) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Vertical Velocity",
        "Time [s]",
        "Velocity [m/s]", 
        false, 
        &dy) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

}