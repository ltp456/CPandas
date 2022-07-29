use std::borrow::BorrowMut;

use iced::{alignment, Color, Command, Length, Renderer};
use iced::pure::{Application, button, column, container, Element, row, scrollable, text, text_input};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use db::Database;

#[derive(Debug)]
pub enum CPandas {
    HomePage(State),
    NewPage(State),
}


#[derive(Debug, Clone)]
pub struct State {
    items: Vec<Item>,
    input_item: InputItem,
   // db: Database,
    secret: String,
    // todo
}


#[derive(Debug, Clone, Default)]
pub struct InputItem {
    pub account_value: String,
    pub secret_value: String,
    pub desc_value: String,
}


impl InputItem {
    pub fn clear(&mut self) {
        self.account_value = "".to_string();
        self.secret_value = "".to_string();
        self.desc_value = "".to_string();
    }
}


#[derive(Debug, Clone, Default, Deserialize, Serialize)]
pub struct Item {
    id: String,
    account: String,
    secret: String,
    desc: String,
    status: usize,
}


#[derive(Debug, Clone)]
pub enum Message {
    New,
    CloseNew,
    DelItem(usize),
    DecodeItem(usize),
    AccountValueEdited(String),
    SecretValueEdited(String),
    DescValueEdited(String),
    NewItemComplete,
    Saved,
}

impl Application for CPandas {
    type Executor = iced::executor::Default;
    type Message = Message;
    type Flags = ();

    fn new(flags: Self::Flags) -> (Self, Command<Self::Message>) {
        log::debug!("new..");
        (
            CPandas::HomePage(State {
                items: vec![],
                input_item: InputItem::default(),
              //  db: Database::new(".db").unwrap(),
                secret: "".to_string(),
            }),
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
            CPandas::NewPage(state) => {
                match message {
                    Message::CloseNew => {
                        *self = CPandas::HomePage(state.clone());
                    }
                    Message::AccountValueEdited(input_value) => {
                        state.input_item.account_value = input_value
                    }
                    Message::SecretValueEdited(value) => {
                        state.input_item.secret_value = value
                    }
                    Message::DescValueEdited(value) => {
                        state.input_item.desc_value = value
                    }
                    Message::NewItemComplete => {
                        let uuid = Uuid::new_v4().to_string();
                        let item = Item {
                            id: uuid.clone(),
                            account: state.input_item.account_value.clone(),
                            secret: state.input_item.secret_value.clone(),
                            desc: state.input_item.desc_value.clone(),
                            status: 0,
                        };
                        // let data = serde_json::to_string(&item).unwrap();
                        // state.db.put(uuid, data).unwrap();
                        state.items.push(item);
                        state.input_item.clear();
                        log::debug!("input complete")
                    }
                    Message::New => {}
                    _ => {}
                }
            }
            CPandas::HomePage(state) => {
                match message {
                    Message::New => {
                        *self = CPandas::NewPage(state.clone());
                    }
                    Message::DelItem(index) => {
                        state.items.remove(index);
                    }
                    Message::DecodeItem(index) => {
                        log::debug!("decode info");
                        state.items.get_mut(index).unwrap().secret = "decode".to_string();
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
            CPandas::HomePage(state) => { home_page_view(state) }
            CPandas::NewPage(state) => { new_page_view(state) }
        }
    }
}


fn home_page_view<'a>(state: &State) -> Element<'a, Message> {
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

    let items: Element<_> = state.items.iter()
        .enumerate()
        .fold(column()
                  .spacing(20)
              , |column, (i, item)| {
                column.push(
                    row()
                        .spacing(10)
                        .width(Length::Fill)
                        .align_items(alignment::Alignment::Center)
                        .push(
                            text(item.account.as_str())
                                .width(Length::Fill)
                                .color(Color::from([0.9, 0.1, 0.1]))
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .vertical_alignment(alignment::Vertical::Center)
                                .size(20)
                        )
                        .push(
                            text(item.secret.as_str())
                                .width(Length::Fill)
                                .color(Color::from([0.9, 0.1, 0.1]))
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .vertical_alignment(alignment::Vertical::Center)
                                .size(20)
                        )
                        .push(
                            text(item.desc.as_str())
                                .width(Length::Fill)
                                .color(Color::from([0.9, 0.1, 0.1]))
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .vertical_alignment(alignment::Vertical::Center)
                                .size(20)
                        )
                        .push(
                            button(
                                text("info")
                                    .width(Length::Fill)
                                    .horizontal_alignment(alignment::Horizontal::Center)
                                    .vertical_alignment(alignment::Vertical::Center)
                                    .size(20)
                                    .color(Color::from([0.1, 0.6, 0.8]))
                            )
                                .width(Length::Fill)
                                .on_press(Message::DecodeItem(i))
                        )
                        .push(button(
                            text("del")
                                .width(Length::Fill)
                                .horizontal_alignment(alignment::Horizontal::Center)
                                .vertical_alignment(alignment::Vertical::Center)
                                .size(20)
                                .color(Color::from([0.9, 0.6, 0.8]))
                        )
                            .width(Length::Fill)
                            .on_press(Message::DelItem(i)))
                )
            })
        .width(Length::Fill)
        .align_items(alignment::Alignment::Center)
        .into();


    let content = column()
        .spacing(20)
        .push(title)
        .push(add)
        .push(items);

    scrollable(container(content)
        .width(Length::Fill)
        .padding(40)
        .center_x()).into()
}


fn new_page_view<'a>(state: &State) -> Element<'a, Message> {
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

    let input = column()
        .spacing(10)
        .push(
            text_input("Input Name", &state.input_item.account_value, Message::AccountValueEdited)
                .padding(15)
                .size(20)
        )
        .push(
            text_input("Input secret", &state.input_item.secret_value, Message::SecretValueEdited)
                .padding(15)
                .size(20)
        )
        .push(
            text_input("Input desc", &state.input_item.desc_value, Message::DescValueEdited)
                .padding(15)
                .size(20)
        )
        .push(
            button(
                text("save")
                    .size(20)
                    .width(Length::Fill)
                    .horizontal_alignment(alignment::Horizontal::Center)
                    .vertical_alignment(alignment::Vertical::Center)
                    .color(Color::from([0.8, 0.3, 0.9]))
            )
                .width(Length::Fill)
                .padding(15)
                .on_press(Message::NewItemComplete)
        )
        .padding(15)
        .width(Length::Fill);

    let content = row()
        .spacing(20)
        .push(title)
        .push(close);
    let input_table = column()
        .spacing(10)
        .push(content)
        .push(input);

    scrollable(container(input_table)
        .width(Length::Fill)
        .padding(40)
        .center_x()).into()
}

