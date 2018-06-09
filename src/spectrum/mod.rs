//! Color variables from wavelengths

mod cdf;
mod wavelength;

use self::cdf::{kBlackbodyCDFTemperature, blackbodyCDF};
use self::wavelength::{kFirstWavelength, kLastWavelength, wavelengthToRGB};


pub fn wavelength_to_colour(nm: f64) -> (u16, u16, u16) {
    // Special Case: monochromatic white.
    if nm == 0.0 {
        // We don't know why this is 8K yet.
        return (8192, 8192, 8192);
    }

    // Case: Light outside of visible spectrum
    if nm < kFirstWavelength || nm > kLastWavelength {
        return (0, 0, 0);
    }

    let fp_index: f64 = nm - kFirstWavelength;
    let index: usize = fp_index.floor() as usize;
    let frac: f64 = fp_index - fp_index.floor();
    let inv: f64 = 1.0 - frac;

    let c1: (i16, i16, i16) = wavelengthToRGB[index];
    let c2: (i16, i16, i16) = wavelengthToRGB[index + 1];

    let r: u16 = (inv * c1.0 as f64 + frac * c2.0 as f64) as u16;
    let g: u16 = (inv * c1.1 as f64 + frac * c2.1 as f64) as u16;
    let b: u16 = (inv * c1.2 as f64 + frac * c2.2 as f64) as u16;

    return (r, g, b);
}

pub fn blackbody_wavelength(temp: f64, noise: f64) -> f64 {
    let index: usize = (1 .. blackbodyCDF.len()).find(|x| blackbodyCDF[*x]>=noise).unwrap();

    let lower: f64 = blackbodyCDF[index - 1];
    let upper: f64 = blackbodyCDF[index];

    // Linear interpolation
    let lerp: f64 = index as f64 + (noise - lower) / (upper - lower);

     // Scale to 'temperature' using Wein's displacement law
    return lerp * (kBlackbodyCDFTemperature / temp);

}