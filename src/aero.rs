use crate::vec::{Vector, Angle};
use crate::interpolate::Linear;
use crate::rk4::rk4;
use crate::util::*;

use std::f64::consts::PI;

pub struct Aerofoil<'a> { 
    area: f64,
    pitch: Angle,
    cl: &'a Linear<'a>,
    cd: &'a Linear<'a>,
    cm: &'a Linear<'a>,
}

impl<'a> Aerofoil<'a> {
    pub fn new(area: f64, pitch: Angle, cl: &'a Linear<'a>, cd: &'a Linear<'a>, cm: &'a Linear<'a>) -> Aerofoil<'a> {
        Aerofoil { area, pitch, cl, cd, cm }
    }
    #[inline] pub fn aoa(&self, v: &Vehicle) -> Angle {
        v.orientation + self.pitch - v.velocity.orientation()
    }
    #[inline] pub fn dyn_pressure(&self, v: &Vehicle) -> f64 {
        0.5 * atmo_density(v.position.y()) * v.speed().powi(2)
    }
    pub fn lift_force(&self, v: &Vehicle) -> Vector {
        let lift_coeff = self.cl.interpolate(self.aoa(v).deg());
        Vector::from_radians(
            self.area * lift_coeff * self.dyn_pressure(v), 
            v.orientation.rad() + (PI/2.0)
        )
    }
    pub fn drag_force(&self, v: &Vehicle) -> Vector {
        let drag_coeff = self.cd.interpolate(self.aoa(v).deg());
        Vector::from_radians(
            self.area * drag_coeff * self.dyn_pressure(v), 
            v.direction().rad() + PI
        )
    }
    pub fn pitching_moment(&self, v: &Vehicle) -> f64 {
        let pitch_coeff = self.cm.interpolate(self.aoa(v).deg());
        let chord = 1.0;
        self.area * pitch_coeff * self.dyn_pressure(v) * chord
    }
}

pub struct Vehicle<'a> {
    pub length: f64,
    pub mass: f64,
    pub position: Vector,
    pub velocity: Vector,
    pub moment: f64,
    pub orientation: Angle,
    pub ang_velocity: f64,
    pub wing: Aerofoil<'a>,
    pub elev: Aerofoil<'a>
}

impl<'a> Vehicle<'a> {
    pub fn new(length: f64, mass: f64, position: Vector, velocity: Vector, wing: Aerofoil<'a>, elev: Aerofoil<'a>) -> Vehicle<'a> {
        Vehicle { 
            length,
            mass, 
            moment: mass * length.powi(2) / 12.0,
            position, 
            velocity, 
            orientation: velocity.orientation(),
            ang_velocity: 0.0,
            wing,
            elev,
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
            rk4(f, self.orientation.rad(), 0.0, 1e-2)
        );
    }

    #[inline] pub fn direction(&self) -> Angle {
        self.velocity.orientation()
    }
    #[inline] pub fn speed(&self) -> f64 {
        self.velocity.magnitude()
    }
}