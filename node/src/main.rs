use node::MyApp;

fn main() {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "CPandas",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    );
}
