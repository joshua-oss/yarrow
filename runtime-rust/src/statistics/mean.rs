use ndarray::prelude::*;
use crate::utilities::noise;

pub fn laplace(
    epsilon: f64, num_records: f64,
    data: ArrayD<f64>,
    minimum: f64, maximum: f64) -> f64 {

    let sensitivity: f64 = (maximum - minimum) / num_records;

    let mean: f64 = data
        .mapv(|v| num::clamp(v, minimum, maximum))
        .mean().unwrap();

    let noise: f64 = noise::sample_laplace(0., sensitivity / epsilon);

    mean + noise
}