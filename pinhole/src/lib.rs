pub fn calc_optimalsize(
    ph_focallength: f32,
    ph_wavelength: f32,
    ph_rayleighfactor: f32,
    ph_magnification: f32,
) -> f32 {
    ph_rayleighfactor
        * (((ph_wavelength / 1000000.) * ph_focallength) / (1. + ph_magnification)).sqrt()
}

pub fn calc_viewangle(ph_diameter: f32, ph_thickness: f32) -> f32 {
    let div = ph_diameter / ph_thickness;
    let viewangle: f32 = div.atan();
    viewangle.to_degrees()
}

// from https://cral-perso.univ-lyon1.fr/labo/fc/cdroms/cdrom2004/cd_venus/documents/pinhole/pinhole_imaging.html
pub fn calc_vignetting(ph_focallength: f32, ph_diagonal: f32) -> (f32, f32) {
    let div = ph_focallength / ph_diagonal;
    let angle = 90. - div.atan().to_degrees();
    let cos4 = angle.to_radians().cos(); //.to_degrees();
    let cos4 = cos4.powi(4);
    (cos4, angle)
}

// 0.56 (eg from cos^4) is 0.84 stops darker.
pub fn stop_equivalent(fract: f32) -> f32 {
    -fract.log2()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let _os = calc_optimalsize(1.0, 1.0, 1.0);
        let _va = calc_viewangle(1.0, 1.0);

        assert_eq!(4, 4);
    }
}
