use iced::{Settings, window};
use iced::pure::Application;
fn main() -> iced::Result {
    node::CPandas::run(Settings {
        window: window::Settings {
            size: (500, 500),
            ..window::Settings::default()
        },
        ..Settings::default()
    })
}
