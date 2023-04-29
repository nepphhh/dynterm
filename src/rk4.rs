use crate::vec::Vector;

/// Fourth-order Runge-Kutta (RK4) method for solving an ordinary differential
/// equation (ODE).
///
/// The RK4 method is a widely used numerical method for solving initial value
/// problems. It approximates the solution of an ODE of the form `dx/dt = f(t, x)` 
/// at discrete time steps.
///
/// # Type Parameters
/// * `F`: A function or closure that takes a f64 and a vector argument and returns 
///        a vector value. It represents the differential equation `dx/dt = f(t, x)`.
///
/// # Arguments
/// * `f`: The function or closure representing the differential equation.
/// * `x0`: The initial value of `x` at time `t0`.
/// * `t0`: The initial time value.
/// * `h`: The time step size.
/// * `n`: The number of integration steps.
///
/// # Returns
/// A vector of f64 values representing the approximated solution of the ODE at 
/// each time step.
pub fn rk4_iter<F>(f: F, x0: Vector, t0: f64, h: f64, n: usize) -> Vec<Vector>
where F: Fn(f64, Vector) -> Vector,
{
    // Initialize the output vector with the given size
    let mut x = vec![Vector::new(0.0, 0.0); n + 1]; 

    // Set the initial condition
    x[0] = x0; 

    // Set the initial time value
    let mut t = t0; 

    // Iterate over each integration step
    for i in 0..n {

        // Calculate the four intermediate RK4 values (k1, k2, k3, and k4)
        let k1 = h * f(t, x[i]);
        let k2 = h * f(t + 0.5 * h, x[i] + (0.5 * k1));
        let k3 = h * f(t + 0.5 * h, x[i] + (0.5 * k2));
        let k4 = h * f(t + h, x[i] + k3);

        // Update the solution vector with the weighted sum of the intermediate values
        x[i + 1] = x[i] + (k1 + (2.0 * k2) + (2.0 * k3) + k4) / 6.0;

        // Update the time value for the next integration step
        t += h; 
    }

    x // Return the solution vector
}
pub fn rk4<F>(f: F, x: Vector, t: f64, h: f64) -> Vector
where F: Fn(f64, Vector) -> Vector,
{
    // Calculate the four intermediate RK4 values (k1, k2, k3, and k4)
    let k1 = h * f(t, x);
    let k2 = h * f(t + 0.5 * h, x + (0.5 * k1));
    let k3 = h * f(t + 0.5 * h, x + (0.5 * k2));
    let k4 = h * f(t + h, x + k3);

    // Update the solution vector with the weighted sum of the intermediate values
    x + (k1 + (2.0 * k2) + (2.0 * k3) + k4) / 6.0
}