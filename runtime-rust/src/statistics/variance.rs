use ndarray::prelude::*;
use ndarray_stats::SummaryStatisticsExt;
use crate::utilities::noise;

pub fn laplace(
    epsilon: f64, num_records: f64,
    data: ArrayD<f64>,
    minimum: f64, maximum: f64) -> f64 {

    let sensitivity: f64 = (num_records - 1.0) * ((maximum - minimum) / num_records).powi(2);

    let variance: f64 = data
        .mapv(|v| num::clamp(v, minimum, maximum))
        .central_moment(2).unwrap();

    let noise: f64 = noise::sample_laplace(0., sensitivity / epsilon);

    variance + noise
}