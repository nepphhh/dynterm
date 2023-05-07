use lazy_static::lazy_static;
use std::str::FromStr;

const CL_CSV: &str = include_str!("../data/lift.csv");
const CD_CSV: &str = include_str!("../data/drag.csv");
const CM_CSV: &str = include_str!("../data/moment.csv");

fn parse_csv_data(csv: &str) -> Vec<(f64, f64)> {
    csv.lines().map(|line| {
        let values: Vec<&str> = line.split(',').collect();
        let x = f64::from_str(values[0].trim()).unwrap();
        let y = f64::from_str(values[1].trim()).unwrap();
        (x, y)
    }).collect()
}

// This all happens at compile-time
lazy_static! {
    pub static ref CL_DATA: Vec<(f64, f64)> = parse_csv_data(CL_CSV);
    pub static ref CD_DATA: Vec<(f64, f64)> = parse_csv_data(CD_CSV);
    pub static ref CM_DATA: Vec<(f64, f64)> = parse_csv_data(CM_CSV);
}