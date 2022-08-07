use iced::{Settings, window};
use iced::pure::Application;
use log::LevelFilter;

fn main() -> iced::Result {
    env_logger::builder().filter(Some("node"),LevelFilter::Trace).init();
    node::CPandas::run(Settings {
        window: window::Settings {
            size: (600, 600),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
