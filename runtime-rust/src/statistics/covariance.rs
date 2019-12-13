use yarrow_validator::yarrow;
use ndarray::prelude::*;
use ndarray::Zip;
use crate::utilities::noise;

trait Laplace {
    fn privatize() -> f64;
}

impl Laplace for yarrow::Dpmean {
    fn privatize () -> f64 {
        22.3
    }
}

pub fn laplace(
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