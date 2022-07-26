use iced::{alignment, Command,  Length};
use iced::pure::{container, text,Application,Element};

pub struct CPandas {}

#[derive(Debug,Clone)]
pub enum Message {
    OK
}

impl Application for CPandas {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        (
            CPandas {},
            Command::none()
        )
    }

    fn title(&self) -> String {
        String::from("CPandas")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        loading_message()
    }
}


fn loading_message<'a>() -> Element<'a, Message> {
    container(
        text("Loading...")
            .horizontal_alignment(alignment::Horizontal::Center)
            .size(50),
    )
        .width(Length::Fill)
        .height(Length::Fill)
        .center_y()
        .into()
}

