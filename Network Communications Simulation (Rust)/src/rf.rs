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

pub fn diffraction_loss_db(distance_meters: f64, knife_edge_height_m: f64, wavelength_m: f64) -> f64 {
    // Simplified knife-edge diffraction model using Fresnel parameter
    let fresnel_parameter = knife_edge_height_m * (2.0 / (wavelength_m * distance_meters)).sqrt();
    // Approximate loss in dB
    if fresnel_parameter <= -0.78 {
        0.0
    } else {
        6.9 + 20.0 * ((fresnel_parameter - 0.1).sqrt() + fresnel_parameter - 1.0).log10()
    }
}
