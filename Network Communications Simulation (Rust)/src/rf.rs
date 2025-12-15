use rand::prelude::*;
use serde::{Deserialize, Serialize};
use statrs::distribution::{ContinuousCDF, Normal};

#[derive(Debug, Clone, Copy, Serialize, Deserialize)]
pub struct ChannelParams {
    pub path_loss_exponent_los: f64, // n_LOS
    pub path_loss_exponent_nlos: f64, // n_NLOS
    pub reference_distance_m: f64,    // d0
    pub reference_loss_db: f64,       // PL(d0)
    pub shadowing_std_db: f64,        // sigma
}

pub fn power_los_dbm(tx_power_dbm: f64, distance_meters: f64, channel_params: &ChannelParams) -> f64 {
    let effective_distance = distance_meters.max(channel_params.reference_distance_m);
    let path_loss_db = channel_params.reference_loss_db + 10.0 * channel_params.path_loss_exponent_los * (effective_distance / channel_params.reference_distance_m).log10();
    tx_power_dbm - path_loss_db
}

pub fn power_nlos_dbm(tx_power_dbm: f64, distance_meters: f64, channel_params: &ChannelParams) -> f64 {
    let effective_distance = distance_meters.max(channel_params.reference_distance_m);
    let path_loss_db = channel_params.reference_loss_db + 10.0 * channel_params.path_loss_exponent_nlos * (effective_distance / channel_params.reference_distance_m).log10();
    tx_power_dbm - path_loss_db
}

pub fn small_scale_fading_db(rng: &mut impl Rng, shadowing_std_db: f64) -> f64 {
    // Log-normal shadowing modeled as Gaussian in dB
    let normal = Normal::new(0.0, shadowing_std_db).unwrap();
    // Sample using inverse CDF (rand only provides uniform RNG)
    let uniform_random: f64 = rng.gen();
    normal.inverse_cdf(uniform_random)
}

/// Calculate fictitious distance for Berg recursive diffraction model
/// 
/// The Berg model accounts for first-order diffraction by computing an effective
/// distance that includes a diffraction penalty term.
/// 
/// # Arguments
/// * `vertical_distance` - Vertical distance component |y2 - y1|
/// * `horizontal_distance` - Horizontal distance component |x2 - x1|
/// 
/// # Returns
/// Fictitious distance for path loss calculation
pub fn berg_diffraction_distance(vertical_distance: f64, horizontal_distance: f64) -> f64 {
    // Berg recursive model parameter q90 = sqrt(0.031 / (4*pi))
    const Q90: f64 = 0.049735919716217296; // Precomputed for efficiency
    
    vertical_distance + horizontal_distance + Q90 * vertical_distance * horizontal_distance
}
