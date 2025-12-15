use crate::geom::Point;
use crate::rf::{power_los_dbm, power_nlos_dbm, small_scale_fading_db, ChannelParams};
use rand::prelude::*;

pub fn dbm_to_mw(dbm: f64) -> f64 { 10f64.powf(dbm / 10.0) }
pub fn mw_to_dbm(mw: f64) -> f64 { 10.0 * mw.log10() }

pub fn sinr_db(signal_dbm: f64, interference_dbm_list: &[f64], noise_dbm: f64) -> f64 {
    let signal_mw = dbm_to_mw(signal_dbm);
    let noise_mw = dbm_to_mw(noise_dbm);
    let interference_mw: f64 = interference_dbm_list.iter().map(|&x| dbm_to_mw(x)).sum();
    let sinr = signal_mw / (interference_mw + noise_mw);
    10.0 * sinr.log10()
}

pub enum LinkType { LOS, NLOS }

pub struct BaseStation {
    pub pos: Point,
    pub tx_power_dbm: f64,
}

pub fn received_power_dbm(
    rng: &mut impl Rng,
    user: Point,
    base_station: &BaseStation,
    link: LinkType,
    channel_params: &ChannelParams,
) -> f64 {
    let distance = ((user.x - base_station.pos.x).powi(2) + (user.y - base_station.pos.y).powi(2)).sqrt();
    let base_power = match link {
        LinkType::LOS => power_los_dbm(base_station.tx_power_dbm, distance, channel_params),
        LinkType::NLOS => power_nlos_dbm(base_station.tx_power_dbm, distance, channel_params),
    };
    base_power + small_scale_fading_db(rng, channel_params.shadowing_std_db)
}

// Compute CCDF of provided values with fixed number of bins.
pub fn ccdf(values: &[f64], num_bins: usize) -> (Vec<f64>, Vec<f64>) {
    assert!(num_bins > 1);
    if values.is_empty() {
        return (Vec::new(), Vec::new());
    }

    let min_value = values
        .iter()
        .copied()
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
