pub fn calc_optimalsize(
    ph_focallength: f32, // mm
    ph_wavelength: f32,  // nm * 1e6
    ph_rayleighfactor: f32,
    ph_magnification: f32,
) -> f32 {
    ph_rayleighfactor
        * (((ph_wavelength / 1000000.) * ph_focallength) / (1. + ph_magnification)).sqrt()
}

// Returns half the total view angle.
pub fn calc_viewangle(ph_diameter: f32, ph_thickness: f32) -> f32 {
    let div = ph_diameter / ph_thickness;
    let viewangle: f32 = div.atan();
    viewangle.to_degrees()
}

// from https://cral-perso.univ-lyon1.fr/labo/fc/cdroms/cdrom2004/cd_venus/documents/pinhole/pinhole_imaging.html
// Note that our projection radius in egui code is diameter...
pub fn calc_vignetting(ph_focallength: f32, film_radius: f32) -> (f32, f32) {
    let div = ph_focallength / film_radius; // opposite / adjacent
    let angle = 90. - div.atan().to_degrees(); // 90 - angle because we want the other one
    let cos4 = angle.to_radians().cos(); //.to_degrees();
    let cos4 = cos4.powi(4);
    (cos4, angle)
}

// 0.56 (eg from cos^4) is 0.84 stops darker.
pub fn stop_equivalent(fract: f32) -> f32 {
    -fract.log2()
}
//
// Decimal part * 3 will give a "thirds of a stop" approximation.
// Factor was 6. for 1/3rd stops, now we return stops.
pub fn delta_thirds(fstop0: f32, fstop1: f32) -> f32 {
    2. * (fstop1 / fstop0).log2() // multiply by 6 for 1/3rd stops.
}

// Coverage for a given focal length with a given view angle.
// This radius needs to cover the film, so to speak.
pub fn _coverage_radius(ph_focallength: f32, ph_viewangle: f32) -> f32 {
    ph_focallength / (90.0 - (ph_viewangle)).to_radians().tan()
}
pub fn coverage_radius(ph_focallength: f32, ph_viewangle: f32) -> f32 {
    ph_focallength * ph_viewangle.to_radians().tan()
}

// To cover this radius, we need a focal length of ...
pub fn needed_focallength(ph_radius: f32, ph_viewangle: f32) -> f32 {
    ph_radius * (90. - ph_viewangle).to_radians().tan()
}

// This is the diameter needed to cover the x by y film size.
pub fn projection_diameter(x: f32, y: f32) -> f32 {
    ((x * x) + (y * y)).sqrt().ceil()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn optimalsize() {
        let os = calc_optimalsize(50.0, 550., 1.56, 0.0);
        let os = (os * 100.0).round() / 100.0;
        assert_eq!(os, 0.26);
    }

    #[test]
    fn viewangle() {
        let va = calc_viewangle(1.0, 1.0);
        assert_eq!(va, 45.);
    }

    #[test]
    fn vignetting() {
        let v = stop_equivalent(0.25);
        assert_eq!(v, 2.);
    }

    #[test]
    fn two_stops() {
        let diff = delta_thirds(32., 64.);
        assert_eq!(diff, 2.);
    }

    #[test]
    fn four_stops() {
        let diff = delta_thirds(16., 64.);
        assert_eq!(diff, 4.);
    }

    #[test]
    fn coverage() {
        let cr = coverage_radius(50., 45.);
        assert_eq!(cr, 50.);
    }

    #[test]
    fn needed_f() {
        let nf = needed_focallength(22., 45.);
        assert_eq!(nf, 22.);
    }

    #[test]
    fn proj_diameter() {
        let pd = projection_diameter(3., 4.);
        assert_eq!(pd, 5.);
    }
}
