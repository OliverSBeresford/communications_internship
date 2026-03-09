use core::panic;

// Compute CCDF of provided values with fixed number of bins.
pub fn ccdf(values: &[f64], num_bins: usize) -> (Vec<f64>, Vec<f64>) {
    assert!(num_bins > 1);
    
    if values.is_empty() {
        panic!("Cannot compute CCDF of empty values");
    }

    let min_value: f64 = values
        .iter()
        .copied()
        .filter(|&v| !v.is_infinite())
        .fold(f64::INFINITY, |a, b| a.min(b));
    let max_value = values
        .iter()
        .copied()
        .fold(f64::NEG_INFINITY, |a, b| a.max(b));

    let span = (max_value - min_value).max(std::f64::EPSILON);
    let bin_width = span / num_bins as f64;

    let mut counts = vec![0usize; num_bins];
    for &value in values {
        let mut idx = ((value - min_value) / bin_width).floor() as isize;
        if idx < 0 { idx = 0; }
        if idx as usize >= num_bins { idx = (num_bins as isize) - 1; }
        counts[idx as usize] += 1;
    }
    // cumulative distribution (CDF)
    let total = values.len() as f64;
    let mut cdf = Vec::with_capacity(num_bins);
    let mut accumulator = 0usize;
    for count in &counts {
        accumulator += *count;
        cdf.push(accumulator as f64 / total);
    }
    // CCDF = 1 - CDF
    let complementary_cdf: Vec<f64> = cdf.into_iter().map(|prob| 1.0 - prob).collect();
    // Bin centers
    let mut x_values = Vec::with_capacity(num_bins);
    for i in 0..num_bins {
        let left = min_value + i as f64 * bin_width;
        x_values.push(left + bin_width / 2.0);
    }
    (x_values, complementary_cdf)
}
