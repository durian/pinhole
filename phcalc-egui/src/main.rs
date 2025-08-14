use egui::{Color32, RichText, Vec2};

fn main() -> eframe::Result {
    env_logger::init(); // Log to stderr (if you run with `RUST_LOG=debug`).
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([800.0, 600.0]),
        ..Default::default()
    };
    eframe::run_native(
        "Pinhole Calculations.",
        options,
        Box::new(|_cc| {
            // This gives us image support:

            Ok(Box::<MyApp>::default())
        }),
    )
}

struct MyApp {
    ph_diameter: f32,
    ph_thickness: f32,
    ph_rayleighfactor: f32,
    ph_focallength: f32,
    ph_projradius: f32,
    ph_wavelength: f32,
    ph_magnification: f32,
    ph_subjectdist: f32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self {
            ph_diameter: 0.3,
            ph_thickness: 0.04,
            ph_rayleighfactor: 1.56,
            ph_focallength: 50.,
            ph_projradius: 42.,
            ph_wavelength: 550.,
            ph_magnification: 0.,
            ph_subjectdist: f32::INFINITY,
        }
    }
}

impl MyApp {
    fn calc_optimalsize(&self) -> f32 {
        pinhole::calc_optimalsize(
            self.ph_focallength,
            self.ph_wavelength,
            self.ph_rayleighfactor,
            self.ph_magnification,
        )
    }

    fn calc_viewangle(&self) -> f32 {
        pinhole::calc_viewangle(self.ph_diameter, self.ph_thickness)
    }

    fn calc_vignetting(&self) -> (f32, f32) {
        // projradius is a build parameter, set with slider.
        pinhole::calc_vignetting(self.ph_focallength, self.ph_projradius)
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            //ctx.set_pixels_per_point(3.0);
            ctx.set_zoom_factor(1.5);
            ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 12.0);
            ui.style_mut().spacing.indent = 16.0;
            //
            ui.heading("Pinhole Calculations");
            ui.separator();
            //
            let max_width = ui.max_rect().width();
            //
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_diameter, 0.01..=2.)
                        .drag_value_speed(0.001)
                        .min_decimals(3)
                        .text("Pinhole diameter (mm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_thickness, 0.01..=1.)
                        .drag_value_speed(0.001)
                        .text("Pinhole thickness (mm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_focallength, 1.0..=10000.)
                        .logarithmic(true)
                        .drag_value_speed(1.)
                        .text("Focal length (mm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_projradius, 10.0..=500.)
                        .text("Desired projection radius (mm)"),
                );
            });

            ui.label(format!(
                "View angle {:.1} degrees",
                2. * self.calc_viewangle()
            ));
            ui.label(format!("F-stop {:.1}", 50. / self.ph_diameter));
            //
            ui.label(format!(
                "Vignetting for desired radius {:.1} f-stops at {:.1} degrees angle",
                //self.ph_diagonal * (90. - self.ph_viewangle).to_radians().tan()
                pinhole::stop_equivalent(self.calc_vignetting().0),
                2. * self.calc_vignetting().1
            ));
            //
            ui.separator();
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_wavelength, 350.0..=700.)
                        .text("Wavelength (nm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_rayleighfactor, 1.0..=2.0)
                        .fixed_decimals(2)
                        .text("Rayleight factor"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - 224.;
                ui.add(
                    egui::Slider::new(&mut self.ph_subjectdist, 0.01..=f32::INFINITY) //  10000.)
                        .logarithmic(true)
                        .largest_finite(1000.)
                        .text("Subject distance (m)"),
                );
            });
            self.ph_magnification = self.ph_focallength / (self.ph_subjectdist * 1000.);
            ui.label(format!(
                "Optimal pinhole diameter for this focal length {:.2} mm (at {:.1} magnification)",
                self.calc_optimalsize(),
                self.ph_magnification
            ));
            //
            ui.separator();
            ui.label(
                RichText::new("(c) Peter Berck 2025")
                    .size(4.)
                    .underline()
                    .color(Color32::RED),
            );
        });
    }
}
