use plotters::prelude::*;
use once_cell::sync::Lazy;
use std::str::FromStr;

// Define a function that takes an array of tuples and generates a scatter plot
pub fn plot_scatter(data: &[(f64, f64, f64)]) -> Result<(), Box<dyn std::error::Error>> {

    // Calculate the minimum and maximum x and y values in the data array
    let (x_min, x_max) = data.iter().map(|(x, _, _)| x)
        .fold(
            (f64::INFINITY, f64::NEG_INFINITY), 
            // Find the minimum and maximum values for x
            |acc, &x| (acc.0.min(x), acc.1.max(x))
        );
    let (y_min, y_max) = data.iter().map(|(_, y, _)| y)
        .fold(
            (f64::INFINITY, f64::NEG_INFINITY), 
            // Find the minimum and maximum values for y
            |acc, &y| (acc.0.min(y), acc.1.max(y))
        );
    let aspect_ratio = (x_max-x_min) / (y_max-y_min);

    // Create a new bitmap backend with a specified filename and dimensions
    let root = BitMapBackend::new(
        "scatter.png", 
        (600 * aspect_ratio.ceil() as u32, 600)
    ).into_drawing_area();

    // Fill the backend with white color
    root.fill(&WHITE)?;

    // Create a new chart builder with specified dimensions and margins
    let mut chart = ChartBuilder::on(&root)
        .x_label_area_size(40)
        .y_label_area_size(40)
        .margin(5)
        .caption("Scatter Plot", ("sans-serif", 30))
        // Set the limits of the chart to the calculated minimum and maximum values for x and y
        .build_cartesian_2d(x_min..x_max, y_min..y_max)?;

    // Configure the chart's mesh (grid lines) and draw it
    chart.configure_mesh().draw()?;

    // Draw the data points as circles with radius 5 and a color corresponding to the AoA
    // indicated as the third element of the tuple in the input data
    chart.draw_series(
        data.iter().map(
            |(x, y, aoa)| Circle::new(
                (*x, *y), 
                5, 
                RGBColor((255.0 * aoa/90.0) as u8, 0, 0).filled())
        )
    ).unwrap();

    // Return success status
    Ok(())
}

// https://ntrs.nasa.gov/archive/nasa/casi.ntrs.nasa.gov/19770009539.pdf
// https://www.icao.int/environmental-protection/CarbonOffset/Documents/CarbonNeutral/IcaoDoc8643_en.pdf
// https://web.mit.edu/drela/Public/web/qprop/atmos.pdf
fn isa_density(altitude: f64) -> f64 {
    const RHO0: f64 = 1.225; // Density at sea level, kg/m^3
    const T0: f64 = 288.15; // Temperature at sea level, K
    const L: f64 = 0.0065;  // Temperature lapse rate, K/m
    const G: f64 = 9.80665; // Acceleration due to gravity, m/s^2
    const R: f64 = 287.058; // Gas constant for air, J/(kg K)

    let temp = if altitude <= 11000.0 {
        T0 - L * altitude
    } else {
        216.65
    };

    let press = if altitude <= 11000.0 {
        101325.0 * (temp / T0).powf(G / (L * R))
    } else {
        22632.0 * (-G * (altitude - 11000.0) / (R * 216.65)).exp()
    };

    RHO0 * (press / 101325.0)
}

const SEA_LEVEL_DENSITY: Lazy<f64> = Lazy::new(|| isa_density(0.0));
#[inline] pub fn atmo_density(altitude: f64) -> f64 {
    isa_density(altitude) / *SEA_LEVEL_DENSITY
}

// Reads a string literal as if it were a csv
pub fn parse_string_as_csv(s: &str) -> Vec<(f64, f64)> {
    s.lines().map(|line| {
            let values: Vec<&str> = line.split(',').collect();
            let x = f64::from_str(values[0].trim()).unwrap();
            let y = f64::from_str(values[1].trim()).unwrap();
            (x, y)
        })
        .collect::<Vec<(f64, f64)>>()
}