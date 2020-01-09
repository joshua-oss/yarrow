use ndarray::prelude::*;
use itertools_num::linspace;

pub fn histogram(data: ArrayD<f64>, min: f64, max: f64, num_bins: usize, inclusive_left: bool) -> Array1::<f64> {
    /// Constructs histogram based on `min`, `max`, and `num_bins`.
    /// Supports only equally-sized bins
    ///
    /// # Arguments
    /// * `data` - data array for which you want histogram
    /// * `min` - minimum representable value in histogram (left edge of lowest bin)
    /// * `max` - maximum representable value in histogram (right edge of highest bin)
    /// * `num_bins` - number of desired bins in histogram
    /// * `inclusive_left` - boolean for whether or not left edge of each bin is inclusive; if false, then right edge is inclusive
    ///
    /// # Example
    /// ```
    /// use ndarray::prelude::*;
    /// use crate::utilities::nonprivate_functions;
    ///
    /// // parameter setup
    /// let data: ArrayD<f64> = arr1(&[1., 2., 3., 4., 5., 12., 19., 24., 90., 98.]).into_dyn();
    /// let min: f64 = 0.0;
    /// let max: f64 = 100.0;
    /// let num_bins: usize = 10;
    /// let inclusive_left = true;
    ///
    /// // construct histogram
    /// let hist = nonprivate_functions::histogram(data, min, max, num_bins, inclusive_left);
    /// println!("{:?}", hist);

    // calculate histogram
    let mut hist = histogram(data, min, max, num_bins, true);
    println!("{:?}", hist);
    /// ```
    // construct bin counters
    let mut bin_array = Array1::<f64>::zeros(num_bins);

    // construct bin edges
    let edges: Vec<f64> = linspace::<f64>(min, max, num_bins+1).collect();

    // for each element in the data, increment correct bin
    for elem in data.iter() {
        for i in 0..(edges.len()-1) {
            if elem < &edges[i+1] || (elem == &edges[i+1] && inclusive_left == true) {
                bin_array[i] += 1.0;
                break;
            }
        }
    }
    return bin_array;
}