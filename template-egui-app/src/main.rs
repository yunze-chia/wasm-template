#![warn(clippy::all, rust_2018_idioms)]

struct App {
    counter: i32,
}

impl App {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        App { counter: 0 }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(&ctx, |ui| {
            ui.label("Hello world!");
            if ui.button("+1").clicked() {
                self.counter += 1;
            }
            ui.label(self.counter.to_string())
        });
    }
}

#[cfg(target_arch = "wasm32")]
fn main() {
    console_error_panic_hook::set_once();

    let mut web_options = eframe::WebOptions::default();
    web_options.webgl_context_option = eframe::WebGlContextOption::CompatibilityFirst;

    wasm_bindgen_futures::spawn_local(async {
        eframe::start_web(
            "main_canvas", // hardcode it
            web_options,
            Box::new(|cc| Box::new(App::new(cc))),
        )
        .await
        .expect("failed to start eframe");
    });
}
