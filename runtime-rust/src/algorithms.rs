use ndarray::prelude::*;
use ndarray_stats::SummaryStatisticsExt;
use ndarray::Zip;
use std::collections::HashMap;
use std::string::String;

use crate::utilities::noise;
use crate::utilities::nonprivate_functions;

pub fn dp_mean_laplace(
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

pub fn dp_variance_laplace(
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

pub fn dp_moment_raw_laplace(
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

pub fn dp_covariance(
    epsilon: f64, num_records: f64,
    data_x: ArrayD<f64>, data_y: ArrayD<f64>,
    minimum_x: f64, minimum_y: f64,
    maximum_x: f64, maximum_y: f64) -> f64 {

    let sensitivity: f64 = 2. * (num_records - 1.)
        / num_records * (maximum_x - minimum_x) * (maximum_y - minimum_y);

    let data_x = data_x.mapv(|v| num::clamp(v, minimum_x, maximum_x)).into_dimensionality::<Ix1>().unwrap();
    let data_y = data_y.mapv(|v| num::clamp(v, minimum_y, maximum_y)).into_dimensionality::<Ix1>().unwrap();

    let mean_x = data_x.mean().unwrap();
    let mean_y = data_y.mean().unwrap();

    let mut products = Array1::<f64>::zeros(data_x.len());
    Zip::from(&mut products).and(&data_x).and(&data_y)
        .apply(|total, &x, &y| *total += (x - mean_x) * (y - mean_y));

    let covariance = products.mean().unwrap();
    let noise: f64 = noise::sample_laplace(0., sensitivity / epsilon);

    covariance + noise
}

pub fn dp_histogram_laplace(
    epsilon: f64, data: ArrayD<f64>,
    minimum: f64, maximum: f64,
    num_bins: usize, inclusive_left: bool) -> HashMap::<String, f64> {

    // set sensitivity
    // NOTE: The sensitivity is set at 2 because we consider releasing the entire vector of counts at once.
    //       Changing one data point can change the counts of two bins, each by at most one.
    //       Thus, the maximum difference in l1 norm is 2.
    let sensitivity: f64 = 2.0;

    // clamp data
    let clamped_data: ArrayD<f64> = data.mapv(|v| num::clamp(v, minimum, maximum));

    // calculate non-private histogram
    let mut hist = nonprivate_functions::histogram(clamped_data, minimum, maximum, num_bins, inclusive_left);

    // add laplace noise to each bin's count
    for (_, count) in hist.iter_mut() {
        *count += noise::sample_laplace(0., sensitivity / epsilon);
    }

    // return DP histogram
    return hist;
}

pub fn dp_histogram_stability(
    epsilon: f64, delta: f64, data: ArrayD<f64>,
    minimum: f64, maximum: f64,
    num_bins: usize, inclusive_left: bool) -> HashMap::<String, f64> {

    // set sensitivity
    // NOTE: The sensitivity is set at 2 because we consider releasing the entire vector of counts at once.
    //       Changing one data point can change the counts of two bins, each by at most one.
    //       Thus, the maximum difference in l1 norm is 2.
    let sensitivity: f64 = 2.0;

    // clamp data
    let clamped_data: ArrayD<f64> = data.mapv(|v| num::clamp(v, minimum, maximum));

    // calculate non-private histogram
    let mut hist = nonprivate_functions::histogram(clamped_data, minimum, maximum, num_bins, inclusive_left);

    // add laplace noise to each bin's count that is not 0
    // then, check threshold and set count to 0 if it is too small
    for (_, count) in hist.iter_mut() {
        if *count != 0.0 {
            *count += noise::sample_laplace(0., sensitivity / epsilon);
            if *count < 2.0*(2.0 / delta).ln()/epsilon + 1.0 {
                *count = 0.0;
            }
        }
    }

    // return DP histogram
    return hist;
}