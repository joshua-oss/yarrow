use ndarray::prelude::*;
use itertools_num::linspace;
use std::collections::HashMap;
use std::string::String;

pub fn histogram(data: ArrayD<f64>, min: f64, max: f64, num_bins: usize, inclusive_left: bool) -> HashMap::<String, f64> {
    /// Constructs histogram based on `min`, `max`, and `num_bins`.
    /// Automatically enforces equally-sized bins.
    ///
    /// NOTE: Histogram automatically closes bins at extremes if the inclusion rule does not make it so.
    /// For example, if inclusive_left == false, then the leftmost bin is still closed on the left
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
    /// ```

    // initialize histogram names and value hashmap
    let mut hist_hashmap: HashMap::<String, f64> = HashMap::new();

    // initialize array
    let mut bin_array = Array1::<f64>::zeros(num_bins);

    // construct bin edges
    let edges: Vec<f64> = linspace::<f64>(min, max, num_bins+1).collect();

    // construct bin names
    let mut bin_names = Array2::zeros((num_bins, 2));
    for i in 0..num_bins {
        bin_names[[i, 0]] = edges[i];
        bin_names[[i, 1]] = edges[i+1];
    }

    // for each element in the data, increment correct bin
    for elem in data.iter() {
        for i in 0..(edges.len()-1) {
            // check if elem is out of given bounds
            // NOTE: this probably shouldn't be happening, given that the histogram is
            //       likely getting min/max from the data bounds,
            //       so the data should be clamped to the [min,max] interval already
            if elem < &min || elem > &max {
                break
            } else if
            // element is smaller than right bin edge
            elem < &edges[i+1] ||
            // element is equal to the right bin edge and we are building our histogram to be 'right-edge inclusive'
            (elem == &edges[i+1] && inclusive_left == false) ||
            // element is equal to the right bin edge and we are checking our rightmost bin
            (elem == &edges[i+1] && i == (edges.len()-2)) {
                bin_array[i] += 1.0;
                break;
            }
        }
    }

    // construct hashmap of histogram bin names and values
    for i in 0..bin_array.len() {
        let mut bin_name = String::new();
        if (i == 0 && inclusive_left == false) {
            // leftmost bin must be left inclusive even if overall strategy is to be right inclusive
            bin_name = format!("[{}, {}]", bin_names[[i,0]], bin_names[[i,1]]);
        } else if (i == (bin_array.len()-1) && inclusive_left == true) {
            // rightmost bin must be right inclusive even if overall strategy is to be left inclusive
           bin_name = format!("[{}, {}]", bin_names[[i,0]], bin_names[[i,1]]);
        } else if inclusive_left == true {
            bin_name = format!("[{}, {})", bin_names[[i,0]], bin_names[[i,1]]);
        } else {
            bin_name = format!("({}, {}]", bin_names[[i,0]], bin_names[[i,1]]);
        }
        hist_hashmap.insert(bin_name, bin_array[i]);
    }

    return hist_hashmap;
}