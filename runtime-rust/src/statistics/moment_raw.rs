use ndarray::prelude::*;
use crate::utilities::noise;

pub fn laplace(
    epsilon: f64, num_records: f64,
    data: ArrayD<f64>,
    minimum: f64, maximum: f64,
    order: u32) -> f64 {

    let sensitivity: f64 = (maximum - minimum).powi(order as i32) / num_records;

    let moment: f64 = data
        .mapv(|v| num::clamp(v, minimum, maximum).powi(order as i32))
        .mean().unwrap();

    let noise: f64 = noise::sample_laplace(0., sensitivity / epsilon);

    moment + noise
}
