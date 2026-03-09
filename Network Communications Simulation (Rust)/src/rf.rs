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
    const Q90: f64 = 0.04966791336390505; // Precomputed for efficiency
    
    vertical_distance + horizontal_distance + Q90 * vertical_distance * horizontal_distance
}
