mod app;

fn main() -> Result<(), eframe::Error> {
    let options = eframe::NativeOptions {
        initial_window_size: Some(egui::vec2(1280.0, 1024.0).into()),
        #[cfg(feature = "wgpu")]
        renderer: eframe::Renderer::Wgpu,
        ..Default::default()
    };
    eframe::run_native(
        "ShowOff",
        options,
        Box::new(|cc| Box::new(app::ShowOffApp::new(cc))),
    )
}
