use std::ops::{Add, Mul, Div};

pub fn rk4<F, T>(f: F, x: T, t: f64, h: f64) -> T
where 
    F: Fn(f64, T) -> T,
    T: Copy
        + Add<T, Output = T>
        + Div<f64, Output = T>,
    f64: Mul<T, Output = T>,
{
    let half_h = h/2.0;

    // Calculate the four intermediate RK4 values (k1, k2, k3, and k4)
    let k1 = h * f(t, x);
    let k2 = h * f(t + half_h, x + (0.5 * k1));
    let k3 = h * f(t + half_h, x + (0.5 * k2));
    let k4 = h * f(t + h, x + k3);

    // Update the solution vector with the weighted sum of the intermediate values
    x + (k1 + (2.0 * k2) + (2.0 * k3) + k4) / 6.0
}