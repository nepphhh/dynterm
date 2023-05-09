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

use std::f64::consts::PI;

// Timestep values
const MAX_SECONDS: usize = 180;
const STEPS_PER_SECOND: usize = 100;
const MAX_INCREMENTS: usize =  MAX_SECONDS * STEPS_PER_SECOND;

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
        100_000.0,
        46.6,
        Kinematics::new(
            Vector::new(0.0, 7_300.0), 
            Angle::from_degrees(-45.5)
        ),
        Kinematics::new_raw(
            Vector::from_degrees(280.0, -45.0), 
            1.4_f64.to_radians()
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
        ),
        280_000.0
    );

    // Set up our record of positions
    let mut data = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];

    // Set up other trackers
    let mut aoa = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut om = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut dx = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut dy = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut re = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut gs = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut th = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut ddx = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut ddy = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut ddn = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];
    let mut ddt = vec![(f64::NAN, f64::NAN, f64::NAN); MAX_INCREMENTS];

    // For finding acceleration
    let mut old_motion = vehicle.motion;

    // Loop
    for i in 0..MAX_INCREMENTS {

        let second = i as f64 / STEPS_PER_SECOND as f64;
        
        // Pull up?
        vehicle.elev.set_pitch(
            Angle::from_degrees(
                if vehicle.position.y() < 7_300.0 { -3.0} else { 0.0 }));

        // Iterate using RK4
        vehicle.apply_dynamics(1.0 / STEPS_PER_SECOND as f64, 50);

        // Perform logging & plotting
        let datum = (
            vehicle.position.x(), 
            vehicle.position.y(), 
            vehicle.aoa().nice_deg().abs()
        );

        println!("{:.2}: {:.3?}", second, datum);

        data[i] = datum;
        aoa[i] = (second, vehicle.aoa().nice_deg(), 0.0);
        om[i] = (second, vehicle.motion.ang, 0.0);
        dx[i] = (second, vehicle.motion.x(), 0.0);
        dy[i] = (second, vehicle.motion.y(), 0.0);
        re[i] = (second, 
            isa_density(vehicle.position.y()) 
                * vehicle.motion.magnitude() 
                * 8.0 
                / isa_dynamic_viscosity(vehicle.position.y()),
                0.0);

        // A little evil, but not very
        th[i] = (second, unsafe { *vehicle.last_thrust.get() }, 0.0);

        let accel: Kinematics = 
            STEPS_PER_SECOND as f64 * (vehicle.motion - old_motion);
        gs[i] = (second,
            Vector::new(accel.x(), accel.y() + 9.81).magnitude() / 9.81, 
            0.0);
        ddx[i] = (second, accel.x(), 0.0);
        ddy[i] = (second, accel.y(), 0.0);
        ddt[i] = (second, accel.vec.dot(vehicle.position.vec.unit()), 0.0);
        ddn[i] = (second, 
            accel.vec.dot(
                Vector::from_radians(
                    1.0, 
                    vehicle.position.direction().rad() + PI/2.0)), 
            0.0);
        
        // Update old motion 
        old_motion = vehicle.motion;

        // Terminate if it hits the ground
        if vehicle.position.y() <= 0.0 { break; }
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
        &om) 
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
    match plot_scatter(
        "Reynolds Number",
        "Time [s]",
        "Re [1]", 
        false, 
        &re) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "G-force",
        "Time [s]",
        "G-force [1]", 
        false, 
        &gs) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Thrust Force",
        "Time [s]",
        "Thrust [kN]", 
        false, 
        &th) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Horizontal Acceleration",
        "Time [s]",
        "Acceleration [m/s2]", 
        false, 
        &ddx) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Vertical Acceleration",
        "Time [s]",
        "Acceleration [m/s2]", 
        false, 
        &ddy) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Tangental Acceleration",
        "Time [s]",
        "Acceleration [m/s2]", 
        false, 
        &ddt) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }
    match plot_scatter(
        "Normal Acceleration",
        "Time [s]",
        "Acceleration [m/s2]", 
        false, 
        &ddn) 
    {
        Ok(()) => {},
        Err(e) => eprintln!("Error generating plot: {}", e),
    }

}