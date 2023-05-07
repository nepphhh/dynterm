pub struct Linear<'a> {
    // Define a data field that is a reference to a slice of tuples (f64, f64)
    // The 'a lifetime parameter ensures that the reference remains valid for the lifetime of the struct.
    data: &'a Vec<(f64, f64)>,
}

// Implement methods for the `Linear` struct.
impl<'a> Linear<'a> {

    // Constructor, takes a vector of f64 tuples
    pub const fn new(data: &'a Vec<(f64, f64)>) -> Self {
        Linear { data }
    }

    // Define a public method named `interpolate` that takes 
    // an f64 value `x` and returns an Option<f64>. The `Option`
    // type is used to represent a value that might be missing 
    // (None) or available (Some(value)).
    pub fn interpolate(&self, x: f64) -> f64 {
        let data = self.data;

        // Find the indices of the two points that will be used 
        // for interpolation. Use the `binary_search_by` method, 
        // which returns a `Result` type. In case the exact value
        // is found, the `Ok` variant contains the index of the value.
        // Otherwise, the `Err` variant contains the index where 
        // the value would be inserted.
        let i = data.binary_search_by(|probe| probe.0.partial_cmp(&x).unwrap())
            .unwrap_or_else(|i| i - 1); // If the exact value is not found, subtract 1 from the index.
        let j = i + 1;

        // Interpolate between the two points.
        // Calculate the interpolation factor `t` based on the 
        // x-values of the two points.
        let t = (x - data[i].0) / (data[j].0 - data[i].0);

        // Calculate the interpolated y-value based on the y-values 
        // of the two points and the interpolation factor `t`.
        data[i].1 + t * (data[j].1 - data[i].1)
    }
}