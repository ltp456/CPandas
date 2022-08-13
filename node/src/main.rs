use eframe::egui::Vec2;
use log::LevelFilter;
use node::CPandas;

fn main() {
    env_logger::builder().filter(Some("node"),LevelFilter::Trace).init();
    let mut options = eframe::NativeOptions::default();
    options.initial_window_size = Some(Vec2::new(540., 960.));
    eframe::run_native(
        "CPandas",
        options,
        Box::new(|_cc| Box::new(CPandas::new())),
    );
}
