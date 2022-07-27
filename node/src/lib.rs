use iced::{alignment, Color, Command, Length};
use iced::pure::{Application, button, column, container, Element, row, scrollable, text};
use iced::pure::widget::Text;

#[derive(Debug)]
pub enum CPandas {
    Home,
    New,
}


#[derive(Debug, Clone)]
pub enum Message {
    New,
    CloseNew,
}

impl Application for CPandas {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::debug!("new..");
        (
            CPandas::New,
            Command::none()
        )
    }

    fn title(&self) -> String {
        log::debug!("title...");
        String::from("CPandas")
    }

    fn update(&mut self, message: Self::Message) -> Command<Self::Message> {
        log::debug!("update...");
        match self {
            CPandas::New => {
                match message {
                    Message::CloseNew => {
                        *self = CPandas::Home;
                    }
                    _ => {}
                }
            }
            CPandas::Home => {
                match message {
                    Message::New => {
                        *self = CPandas::New
                    }
                    _ => {}
                }
            }
            _ => {}
        }


        Command::none()
    }

    fn view(&self) -> Element<'_, Self::Message> {
        log::debug!("view...");
        match self {
            CPandas::Home => { home_view() }
            CPandas::New => { new_view() }
        }
    }
}


fn home_view<'a>() -> Element<'a, Message> {
    let title = text("CPandas")
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
        .size(40)
        .color(Color::from([0.5, 0.5, 0.5]));

    let add = button(text("New")
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
        .size(20)
        .color(Color::from([0.8, 0.1, 0.9])))
        .width(Length::Fill)
        .padding(8)
        .on_press(Message::New);

    let content = column()
        .spacing(20)
        .max_width(800)
        .push(title)
        .push(add);

    scrollable(container(content)
        .width(Length::Fill)
        .padding(40)
        .center_x()).into()
}


fn new_view<'a>() -> Element<'a, Message> {
    let title = text("AddNew")
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
        .size(30)
        .color(Color::from([0.5, 0.5, 0.5]));

    let close = button(text("Close")
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
        .size(20)
        .color(Color::from([0.8, 0.1, 0.9])))
        .width(Length::Fill)
        .padding(8)
        .on_press(Message::CloseNew);

    let content = row()
        .spacing(20)
        .push(title)
        .push(close);

    scrollable(container(content)
        .width(Length::Fill)
        .padding(40)
        .center_x()).into()
}

