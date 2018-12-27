//! Color variables from wavelengths

mod cdf;
mod wavelength;

use self::cdf::{BLACKBODY_CDF_TEMP, BLACKBODY_CDF_DATA};
use self::wavelength::{FIRST_WAVELENGTH, LAST_WAVELENGTH, WAVELENGTH_TO_RGB};


pub fn wavelength_to_colour(nm: f64) -> (u16, u16, u16) {
    // Special Case: monochromatic white.
    if nm == 0.0 {
        // We don't know why this is 8K yet.
        return (8192, 8192, 8192);
    }

    // Case: Light outside of visible spectrum
    if nm < FIRST_WAVELENGTH || nm > LAST_WAVELENGTH {
        return (0, 0, 0);
    }

    let fp_index: f64 = nm - FIRST_WAVELENGTH;
    let index: usize = fp_index.floor() as usize;
    let frac: f64 = fp_index.fract();
    let inv: f64 = 1.0 - frac;

    let c1: (i16, i16, i16) = WAVELENGTH_TO_RGB[index];
    let c2: (i16, i16, i16) = WAVELENGTH_TO_RGB[index + 1];

    //           <------------LERP Algorithm------------>
    let r: u16 = (inv * c1.0 as f64 + frac * c2.0 as f64) as u16;
    let g: u16 = (inv * c1.1 as f64 + frac * c2.1 as f64) as u16;
    let b: u16 = (inv * c1.2 as f64 + frac * c2.2 as f64) as u16;

    return (r, g, b);
}

pub fn blackbody_wavelength(temp: f64, noise: f64) -> f64 {
    let index: usize = (1 .. BLACKBODY_CDF_DATA.len()).find(|x| BLACKBODY_CDF_DATA[*x]>=noise).expect("Blackbody Index out of range");

    let lower: f64 = BLACKBODY_CDF_DATA[index - 1];
    let upper: f64 = BLACKBODY_CDF_DATA[index];

    // Linear interpolation
    let lerp: f64 = index as f64 + (noise - lower) / (upper - lower);

     // Scale to 'temperature' using Wein's displacement law
    return lerp * (BLACKBODY_CDF_TEMP / temp);
}