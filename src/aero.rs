use crate::vec::{Vector, Angle, Kinematics};
use crate::interpolate::Linear;
use crate::rk4::rk4;
use crate::util::*;

use std::f64::consts::PI;

/// `Aerofoil` represents a simplified airfoil or control surface with specified aerodynamic coefficients.
/// Properties include area and pitch relative to the body vehicle. Aerodynamic coefficients are provided using interpolation objects.
/// The `Aerofoil` struct provides methods for calculating aerodynamic forces and moments acting on the airfoil
/// when attached to a `Vehicle`. It also allows setting the pitch angle of the airfoil, simulating control surface deflection.
pub struct Aerofoil<'a> { 
    area: f64,
    chord: f64,
    pitch: Angle,
    cl: &'a Linear<'a>,
    cd: &'a Linear<'a>,
    cm: &'a Linear<'a>,
}

impl<'a> Aerofoil<'a> {

    // Constructor
    pub fn new(area: f64, chord: f64, pitch: Angle, cl: &'a Linear<'a>, cd: &'a Linear<'a>, cm: &'a Linear<'a>) -> Aerofoil<'a> {
        Aerofoil { area, chord, pitch, cl, cd, cm }
    }

    // Control function
    pub fn set_pitch(&mut self, pitch: Angle) {
        self.pitch = pitch;
    }

    // Gets the angle of attack relative to a body vehicle
    #[inline] pub fn aoa(&self, k: &Kinematics, dk: &Kinematics) -> Angle {
        (k.angle() + self.pitch) - dk.direction()
    }

    // Calculates the dynamic pressure experienced, using altitude from k &
    // speed from magnitude of dk
    #[inline] pub fn dyn_pressure(&self, k: &Kinematics, dk: &Kinematics) -> f64 {
        0.5 * atmo_density(k.y()) * dk.magnitude().powi(2)
    }

    // Calculates the lift force if attached to a body vehicle. This is always
    // normal to the direction of motion
    pub fn lift_force(&self, k: &Kinematics, dk: &Kinematics) -> Vector {

        // Get the lift coefficient from the angle of attack 
        // (use orientation from k & velocity from dk)
        let lift_coeff = self.cl.interpolate(self.aoa(k, dk).deg());

        Vector::from_radians(
            self.area * lift_coeff * self.dyn_pressure(k, dk), 
            dk.direction().rad() + (PI/2.0) // Normal to direction of motion
        )
    }

    // Calcuates the lift force if attached to a body vehicle. This is always
    // against the direction of motion
    pub fn drag_force(&self, k: &Kinematics, dk: &Kinematics) -> Vector {

        // Get the drag coefficient from the angle of attack 
        // (use orientation from k & velocity from dk)
        let drag_coeff = self.cd.interpolate(self.aoa(k, dk).deg());

        Vector::from_radians(
            self.area * drag_coeff * self.dyn_pressure(k, dk), 
            dk.direction().rad() + PI // Antitangent direction of motion
        )
    }

    // Calculates the pitching moment generated by the airstream over the wing.
    // This is a free moment.
    pub fn pitching_moment(&self, k: &Kinematics, dk: &Kinematics) -> f64 {

        // Get the pitching moment coefficient from the angle of attack 
        // (use orientation from k & velocity from dk)
        let pitch_coeff = self.cm.interpolate(self.aoa(k, dk).deg());

        self.area * pitch_coeff * self.dyn_pressure(k, dk) * self.chord
    }
}

/// `Vehicle` represents a simplified aerospace vehicle with a massless main wing 
/// and stabilator. The `Vehicle` struct provides methods for applying forces and
/// moments to the vehicle using RK4.
/// 
/// 'a is a lifetime marker, indicating that the references to wings must be valid 
/// for the lifetime of the vehicle.
pub struct Vehicle<'a> {
    pub mass: f64,
    pub length: f64,
    pub moment: f64,
    pub position: Kinematics,
    pub motion: Kinematics,
    pub wing: Aerofoil<'a>,
    pub elev: Aerofoil<'a>
}

// Implementation block for the Vehicle structure
impl<'a> Vehicle<'a> {
    
    // Constructor for a new Vehicle instance
    // Takes in the mass, length, initial position, initial motion, and wing and elevator aerofoils
    pub fn new(mass: f64, length: f64, position: Kinematics, motion: Kinematics, wing: Aerofoil<'a>, elev: Aerofoil<'a>) -> Vehicle<'a> {
        Vehicle { 
            mass,    // Mass of the vehicle
            length,  // Length of the vehicle
            // Moment of inertia is computed using the formula for a rod rotated about its center
            moment: mass * length.powi(2) / 12.0,
            position, // Initial position of the vehicle
            motion,   // Initial motion of the vehicle
            wing,     // Wing aerofoil
            elev,     // Elevator aerofoil
        }
    }
    
    // Returns the angle of attack, the difference between the angle of the vehicle and the direction of its motion
    #[inline] pub fn aoa(&self) -> Angle {
        self.position.angle() - self.motion.direction()
    }

    // Calculates the dynamics of the vehicle given its current position and velocity
    #[allow(non_snake_case)]
    fn calculate_dynamics(&self, k: &Kinematics, dk: &Kinematics) -> Kinematics {

        // Aerofoil references for wing and elevator
        let w: &Aerofoil = &self.wing;
        let e: &Aerofoil = &self.elev;
        
        // Position vector of the elevator
        let r_e = Vector::from_radians(self.length/2.0, k.angle().rad() + PI);
        
        // Unit vector in the direction of the free stream velocity
        let free_stream_unit: Vector = dk.direction().unit();

        // Gravitational force acting on the body
        let W = self.mass * Vector::new(0.0, -9.81);

        // Aerodynamic forces acting on the wing and elevator
        let F_w: Vector = w.lift_force(k, dk) + w.drag_force(k, dk);
        let F_e: Vector = e.lift_force(k, dk) + e.drag_force(k, dk);
        
        // Control force to maintain stability
        let aero_tangent: Vector = (free_stream_unit.cross(F_w + F_e)) * free_stream_unit;
        let T = Vector::from_radians(
            0_000.0, //aero_tangent.magnitude(), 
            k.angle().rad() + PI);

        // Returns the acceleration and the angular acceleration of the vehicle
        Kinematics::new_raw(
            F_w + F_e + T + W,
            w.pitching_moment(k, dk) + e.pitching_moment(k, dk) + r_e.cross(F_e)
        ) / self.mass
    }


    // Use RK4 to apply the calculated forces and moments to the object over the duration of a second
    // The method takes in the number of steps N to discretize the second into
    pub fn apply_dynamics(&mut self, N: u16) {

        // Time step
        let h: f64 = 1.0 / N as f64;

        // Iterate over the time steps
        for i in 0..N {
            
            // Calculate the velocity at each time step using RK4 and the dynamics function  
            // The function "f" calculates the derivative of the motion (velocity), using the
            // dynamics function to get the acceleration
            let f = 
            |t: f64, dk: Kinematics| t * self.calculate_dynamics(&self.position, &dk);

            // RK4 is used to update the vehicle's motion (velocity) based on its 
            // acceleration...
            self.motion = rk4(f, self.motion, 0.0, h);

            // The function "f" calculates the derivative of the position (velocity)
            // It uses the current velocity to get the rate of change of position
            let f = 
                |t: f64, _: Kinematics| t * self.motion;

            // ...and RK4 is used to update the vehicle's position based on its velocity
            self.position = rk4(f, self.position, 0.0, h);
        }
    }
}