use iced::{
    widget::{column, horizontal_rule, row, slider, text},
    window, Element, Length, Task,
};

#[derive(Debug, Default)]
struct AppState {
    ph_diameter: f32,
    ph_thickness: f32,
    ph_viewangle: f32,
    ph_diagonal: f32,
    ph_focallength: f32,
    ph_wavelength: f32,
    ph_rayleighfactor: f32,
}

#[derive(Debug, Clone)]
enum Message {
    UpdatePhDiameter(f32),
    UpdatePhThickness(f32),
    UpdatePhDiagonal(f32),
    UpdatePhWavelength(f32),
    UpdatePhRayleighFactor(f32),
    UpdatePhFocallength(f32),
}

impl AppState {
    fn new() -> (Self, Task<Message>) {
        let mut state = Self {
            ph_diameter: 0.30,
            ph_thickness: 0.05,
            ph_viewangle: 0.0,
            ph_diagonal: 42.0,
            ph_focallength: 50.0,
            ph_wavelength: 550.,
            ph_rayleighfactor: 1.56,
        };

        state.ph_viewangle = state.calc_viewangle();

        (state, Task::none())
    }

    fn _close(id: window::Id) -> Task<Message> {
        window::close(id)
    }

    // Note that this is half the total view angle.
    fn calc_viewangle(&self) -> f32 {
        let div = self.ph_diameter / self.ph_thickness;
        let viewangle: f32 = div.atan();
        viewangle.to_degrees()
    }

    fn calc_optimalsize(&self) -> f32 {
        self.ph_rayleighfactor * (self.ph_wavelength * self.ph_focallength / 1000000.).sqrt()
    }

    // from https://cral-perso.univ-lyon1.fr/labo/fc/cdroms/cdrom2004/cd_venus/documents/pinhole/pinhole_imaging.html
    fn calc_vignetting(&self) -> (f32, f32) {
        // diagonal is radius of projected circle
        let div = self.ph_focallength / self.ph_diagonal;
        let angle = 90. - div.atan().to_degrees();
        let cos4 = angle.to_radians().cos(); //.to_degrees();
        let cos4 = cos4.powi(4);
        (cos4, angle)
    }

    fn update(&mut self, message: Message) {
        match message {
            Message::UpdatePhDiameter(v) => {
                self.ph_diameter = v;
                self.ph_viewangle = self.calc_viewangle();
            }
            Message::UpdatePhThickness(v) => {
                self.ph_thickness = v;
                self.ph_viewangle = self.calc_viewangle();
            }
            Message::UpdatePhDiagonal(v) => {
                self.ph_diagonal = v;
            }
            Message::UpdatePhWavelength(v) => {
                self.ph_wavelength = v;
            }
            Message::UpdatePhRayleighFactor(v) => {
                self.ph_rayleighfactor = v;
            }
            Message::UpdatePhFocallength(v) => {
                self.ph_focallength = v;
            }
        }
    }

    fn view(&self) -> Element<'_, Message> {
        column![
            text("Pinhole Calculations").size(32),
            horizontal_rule(48),
            column![
                // Label for this pane.
                text("View angle").size(32),
                // Value and slider.
                row![
                    text(format!("diameter {:.2} mm  ", self.ph_diameter))
                        .width(Length::FillPortion(1)),
                    slider(0.01..=1.00, self.ph_diameter, |v| {
                        Message::UpdatePhDiameter(v)
                    })
                    .step(0.01)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                row![
                    text(format!("thickness {:.2} mm  ", self.ph_thickness))
                        .width(Length::FillPortion(1)),
                    slider(0.01..=1.00, self.ph_thickness, |v| {
                        Message::UpdatePhThickness(v)
                    })
                    .step(0.01)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                text(format!("View angle {:.0} degrees", 2. * self.ph_viewangle)),
                // Calculated value.
                row![
                    text(format!("film radius {:.0} mm  ", self.ph_diagonal))
                        .width(Length::FillPortion(1)),
                    slider(10.0..=200., self.ph_diagonal, |v| {
                        Message::UpdatePhDiagonal(v)
                    })
                    .step(1.)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                text(format!(
                    "Focal length needed to cover radius is {:.0} mm",
                    self.ph_diagonal * (90. - self.ph_viewangle).to_radians().tan()
                )),
                text(format!(
                    "Vignetting {:.1} f-stops at {:.1} degrees view angle",
                    //self.ph_diagonal * (90. - self.ph_viewangle).to_radians().tan()
                    pinhole::stop_equivalent(self.calc_vignetting().0),
                    2. * self.calc_vignetting().1
                )),
            ],
            horizontal_rule(48),
            // Optimal size calculation.
            column![
                text("Optimal size").size(32),
                row![
                    text(format!("Wavelength {:.0} nm  ", self.ph_wavelength))
                        .width(Length::FillPortion(1)),
                    slider(400.0..=700.0, self.ph_wavelength, |v| {
                        Message::UpdatePhWavelength(v)
                    })
                    .step(1.)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                row![
                    text(format!("Focal length {:.0} mm  ", self.ph_focallength))
                        .width(Length::FillPortion(1)),
                    slider(1.0..=500., self.ph_focallength, |v| {
                        Message::UpdatePhFocallength(v)
                    })
                    .step(1.)
                    .shift_step(0.1)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                row![
                    text(format!("Rayleigh factor {:.2}  ", self.ph_rayleighfactor))
                        .width(Length::FillPortion(1)),
                    slider(0.10..=3.00, self.ph_rayleighfactor, |v| {
                        Message::UpdatePhRayleighFactor(v)
                    })
                    .step(0.01)
                    .width(Length::FillPortion(4)),
                ]
                .padding(8),
                text(format!(
                    "Optimal pinhole diameter {:.2} mm",
                    self.calc_optimalsize()
                )),
                text(format!(
                    "Coverage radius based on focal length and view angle {:.0} mm",
                    self.ph_focallength / (90.0 - (self.ph_viewangle)).to_radians().tan()
                )),
                text(format!(
                    "Effective f-stop based on focal length and pinhole diameter f/{:.0}",
                    self.ph_focallength / self.ph_diameter
                )),
                text(format!(
                    "Distance from f/32 is {:.1} f-stops",
                    pinhole::delta_thirds(32f32, self.ph_focallength / self.ph_diameter)
                )),
            ],
            horizontal_rule(48),
        ]
        .spacing(20)
        .padding(20)
        .into()
    }
}

fn main() -> iced::Result {
    iced::application("Pinhole Calculations", AppState::update, AppState::view)
        .theme(|_| iced::Theme::KanagawaWave)
        .run_with(AppState::new)
}

fn _third_stops(fstop: f32) -> f32 {
    let base = 6f32.sqrt();

    let n = 3.0 * fstop.log2() / base.log2();
    n
}
