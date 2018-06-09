use cdf::{kBlackbodyCDFTemperature, blackbodyCDF};
use wavelength::{kFirstWavelength, kLastWavelength, wavelengthToRGB};

//! Color variables from wavelengths

pub fn wavelength_to_colour(nm: f64) -> (u16, u16, u16) {
    // Special Case: monochromatic white.
    if (nm == 0) {
        // We don't know why this is 8K yet.
        return (8192, 8192, 8192);
    }

    // Case: Light outside of visible spectrum
    if (nm < kFirstWavelength || nm > kLastWavelength) {
        return (0, 0, 0);
    }

    let fp_index: f64 = nm - kFirstWavelength;
    let index: usize = fp_index.floor();
    let frac: f64 = fp_index - index;
    let inv: f64 = 1.0 - frac;

    let c1 = (u16, u16, u16) = wavelenthToRGB[index];
    let c2 = (u16, u16, u16) = wavelenthToRGB[index + 1];

    let r: u16 = inv * c1[0] + frac * c2[0];
    let g: u16 = inv * c1[1] + frac * c2[1];
    let b: u16 = inv * c1[2] + frac * c2[2];

    return (r, g, b);
}

pub fn blackbody_wavelength(temp: f64, noise: f64) -> f64 {
    let index = for i in (1 .. blackbodyCDF.len()) {
        if (blackbodyCDF>=noise) {
            i,
        }
    }

    let lower: f64 = blackbodyCDF[index - 1];
    let upper: f64 = blackbodyCDF[index];

    // Linear interpolation
    let lerp: f64 = index + (noise - lower) / (upper - lower);

     // Scale to 'temperature' using Wein's displacement law
    return lerp * (kBlackbodyCDFTemperature / temperature);

}