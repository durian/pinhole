#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

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
        Box::new(|_cc| Ok(Box::<MyApp>::default())),
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
            ph_projradius: 44.,
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
        pinhole::calc_vignetting(self.ph_focallength, self.ph_projradius / 2.)
    }

    fn calc_coverage_radius(&self) -> f32 {
        pinhole::coverage_radius(self.ph_focallength, self.calc_viewangle())
    }

    fn diameter_to_filmsize(&self, diameter: f32) -> String {
        match diameter {
            x if (43.0..45.0).contains(&x) => "35mm".to_string(),
            x if (74.0..76.0).contains(&x) => "645".to_string(),
            x if (84.0..86.0).contains(&x) => "6x6".to_string(),
            x if (92.0..94.0).contains(&x) => "6x7".to_string(),
            x if (108.0..110.0).contains(&x) => "6x9".to_string(),
            x if (134.0..136.0).contains(&x) => "6x12".to_string(),
            x if (162.0..164.0).contains(&x) => "4\"x5\"".to_string(),
            x if (180.0..182.0).contains(&x) => "6x17".to_string(),
            x if (218.0..220.0).contains(&x) => "5\"x7\"".to_string(),
            x if (325.0..327.0).contains(&x) => "8\"x10\"".to_string(),
            x if (452.0..454.0).contains(&x) => "11\"x14\"".to_string(),
            x if (650.0..652.0).contains(&x) => "16\"x20\"".to_string(),
            x if (793.0..795.0).contains(&x) => "20\"x24\"".to_string(),
            x => format!("{:.1} sq", ((x * x) / 2.0).sqrt()), // _ => "Custom".to_string(),
        }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            let txt_width = 224.;
            //ctx.set_pixels_per_point(3.0);
            ctx.set_zoom_factor(1.4);
            ui.style_mut().spacing.item_spacing = Vec2::new(4.0, 12.0);
            ui.style_mut().spacing.indent = 16.0;
            //
            ui.heading("Pinhole Calculations");
            ui.separator();
            //
            let max_width = ui.max_rect().width();
            //
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - txt_width;
                ui.add(
                    egui::Slider::new(&mut self.ph_diameter, 0.01..=2.)
                        .drag_value_speed(0.001)
                        .min_decimals(3)
                        //.custom_formatter(|n, _| format!("{:.3} / {:.0}", n, n * 1000.))
                        //.text("Pinhole Ø (mm/micron)"),
                        .text("Pinhole Ø (mm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - txt_width;
                ui.add(
                    egui::Slider::new(&mut self.ph_thickness, 0.01..=1.)
                        .drag_value_speed(0.001)
                        .text("Pinhole thickness (mm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - txt_width;
                ui.add(
                    egui::Slider::new(&mut self.ph_focallength, 1.0..=10000.)
                        .logarithmic(true)
                        .drag_value_speed(1.)
                        .text("Focal length (mm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - (txt_width + 84.);
                ui.add(
                    egui::Slider::new(&mut self.ph_projradius, 10.0..=1000.)
                        .fixed_decimals(0)
                        .drag_value_speed(1.)
                        .text("Desired projection Ø (mm)"),
                );
                //});
                egui::ComboBox::from_label("")
                    //.selected_text(format!("{:.0}", self.ph_projradius))
                    .selected_text(self.diameter_to_filmsize(self.ph_projradius))
                    .show_ui(ui, |ui| {
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(24., 36.),
                            "35mm",
                        ); // is diameter!
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(60., 45.),
                            "645",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(60., 60.),
                            "6x6",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(60., 70.),
                            "6x7",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(60., 90.),
                            "6x9",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(60., 120.),
                            "6x12",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(4. * 25.4, 5. * 25.4),
                            "4\"x5\"",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(60., 170.),
                            "6x17",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(5. * 25.4, 7. * 25.4),
                            "5\"x7\"",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(8. * 25.4, 10. * 25.4),
                            "8\"x10\"",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(11. * 25.4, 14. * 25.4),
                            "11\"x14\"",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(16. * 25.4, 20. * 25.4),
                            "16\"x20\"",
                        );
                        ui.selectable_value(
                            &mut self.ph_projradius,
                            pinhole::projection_diameter(20. * 25.4, 24. * 25.4),
                            "20\"x24\"",
                        );
                        // 5"x7"/219, 8"x10"/326, 11"x14", 16"x20", 20"x24"
                        //ui.selectable_value(&mut self.ph_projradius, 93., "6x7");
                    });
            });
            ui.label(format!(
                "View angle is {:.1}˚ which covers a diameter of {:.1}mm",
                2. * self.calc_viewangle(),
                2. * self.calc_coverage_radius(),
            ));
            ui.label(format!(
                "F-stop is f/{:.1} which is {:.1} f-stops from f/32 (t · {:.1})",
                self.ph_focallength / self.ph_diameter,
                pinhole::delta_thirds(32f32, self.ph_focallength / self.ph_diameter),
                (self.ph_focallength / self.ph_diameter / 32.)
                    * (self.ph_focallength / self.ph_diameter / 32.)
            ));
            ////
            /*
            ui.horizontal(|ui| {
                ui.label(RichText::new("F-stop"));
                ui.label(RichText::new("167").color(Color32::RED));
                ui.label("is");
                ui.label(RichText::new("12").color(Color32::BLUE));
                ui.label("f-stops from f/32");
            });
            */
            ////
            ui.label(format!(
                "Vignetting for desired projection Ø is {:.1} f-stops ({:.2}) at a {:.1}˚ angle",
                //self.ph_diagonal * (90. - self.ph_viewangle).to_radians().tan()
                pinhole::stop_equivalent(self.calc_vignetting().0),
                self.calc_vignetting().0,
                self.calc_vignetting().1
            ));
            //
            ui.separator();
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - txt_width;
                ui.add(
                    egui::Slider::new(&mut self.ph_wavelength, 350.0..=700.)
                        .text("Wavelength (nm)"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - txt_width;
                ui.add(
                    egui::Slider::new(&mut self.ph_rayleighfactor, 1.0..=2.0)
                        .fixed_decimals(2)
                        .text("Rayleigh factor"),
                );
            });
            ui.horizontal(|ui| {
                ui.spacing_mut().slider_width = max_width - txt_width;
                ui.add(
                    egui::Slider::new(&mut self.ph_subjectdist, 0.01..=f32::INFINITY) //  10000.)
                        .logarithmic(true)
                        .largest_finite(1000.)
                        .text("Subject distance (m)"),
                );
            });
            self.ph_magnification = self.ph_focallength / (self.ph_subjectdist * 1000.);
            ui.label(format!(
                "Optimal pinhole Ø for this focal length is {:.2} mm (at {:.1} magnification)",
                self.calc_optimalsize(),
                self.ph_magnification
            ));
            //
            ui.separator();
            ui.label(
                RichText::new("(c) Peter Berck 2025")
                    .size(8.)
                    .color(Color32::GRAY),
            );
        });
    }
}
