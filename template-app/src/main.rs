#![warn(clippy::all, rust_2018_idioms)]

struct App {}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App {}
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Hello world!");
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let mut web_options = eframe::WebOptions::default();
    web_options.webgl_context_option = eframe::WebGlContextOption::CompatibilityFirst;
    eframe::start_web(
        "main_canvas", // hardcode it
        web_options,
        Box::new(|cc| Box::new(App::new(cc))),
    )
    .expect("failed to start eframe");
}
