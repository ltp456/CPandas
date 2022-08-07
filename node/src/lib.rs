use std::borrow::{Borrow, BorrowMut};

use anyhow::Result;
use iced::{alignment, Color, Command, Length, Renderer};
use iced::pure::{Application, button, column, container, Element, row, scrollable, text, text_input, tooltip};
use iced::tooltip::Position;
use once_cell::sync::Lazy;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use db::Database;
use types::{*};
use utils::MemData;

mod db;
mod constants;
mod types;
mod utils;


static DB: Lazy<Database> = Lazy::new(|| {
    let database = Database::new(".db").unwrap();
    database
});


#[derive(Debug)]
pub enum CPandas {
    HomePage(State),
    NewPage(State),
    Guild(State),
}


#[derive(Debug, Clone)]
pub enum Message {
    New,
    CloseNew,
    DelItem(usize),
    DecodeItem(usize),
    PasswordComplete,
    PasswordValueEdited(String),
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
        let tmp_items = DB.get_item_list();
        let mut items: Vec<Item> = Vec::new();
        if let Ok(item_list) = tmp_items {
            if let Some(list) = item_list {
                items = list
            }
        };

        let mut guild_tips_info = String::new();
        let secret_hash_opt = DB.get_secret_hash().unwrap();
        if let Some(secret_hash) = secret_hash_opt {
            guild_tips_info = "please input your password".to_string();
        } else {
            guild_tips_info = "please setting a new password".to_string();
        }

        (
            CPandas::Guild(State {
                items,
                input_item: InputItem::default(),
                input_secret: "".to_string(),
                secret: Default::default(),
                guild_tips_msg: guild_tips_info,
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

                        let (ciphertext, nonce) = utils::aes256_encode(state.input_item.secret_value.as_bytes(), state.input_secret.as_bytes()).unwrap();
                        let item = Item {
                            id: uuid.clone(),
                            account: state.input_item.account_value.clone(),
                            secret: hex::encode(ciphertext),
                            desc: state.input_item.desc_value.clone(),
                            status: 0,
                            nonce: hex::encode(nonce),
                        };

                        DB.put_item(&item).unwrap();
                        state.items.push(item);
                        state.input_item.clear();
                        log::debug!("input complete")
                    }
                    _ => {}
                }
            }
            CPandas::HomePage(state) => {
                match message {
                    Message::New => {
                        *self = CPandas::NewPage(state.clone());
                    }
                    Message::DelItem(index) => {
                        let x = state.items.get(index).unwrap();
                        DB.del_item(&x.id).unwrap();
                        state.items.remove(index);
                    }
                    Message::DecodeItem(index) => {
                        log::debug!("decode info");
                        let item = state.items.get_mut(index).unwrap();
                        if item.status == 0 {
                            let password = utils::aes256_decode(&hex::decode(&item.secret).unwrap(),
                                                                state.input_secret.as_bytes(),
                                                                &hex::decode(&item.nonce).unwrap()).unwrap();
                            item.secret = String::from_utf8(password).unwrap();
                            item.status = 1;
                        } else {
                            let secret = utils::aes256_encode_with_nonce(
                                item.secret.as_bytes(),
                                state.input_secret.as_bytes(),
                                &hex::decode(&item.nonce).unwrap(),
                            ).unwrap();
                            item.secret = hex::encode(secret);
                            item.status = 0;
                        }

                        // check
                    }
                    _ => {}
                }
            }

            CPandas::Guild(state) => {
                match message {
                    Message::PasswordValueEdited(value) => {
                        log::debug!("password: {}",value);
                        state.input_secret = value
                    }
                    Message::PasswordComplete => {
                        log::debug!("password completed");
                        if state.input_secret.len() > 32 {
                            state.guild_tips_msg = "password length max is 32".to_string();
                            return Command::none();
                        }
                        let secret_hash_opt = DB.get_secret_hash().unwrap();
                        // todo
                        let secret_key = utils::get_valid_aes_key(state.input_secret.clone()).unwrap();
                        state.input_secret = secret_key.clone();
                        let input_secret_hash = utils::sha256(secret_key.as_bytes()).unwrap();
                        if let Some(secret_hash) = secret_hash_opt {
                            if input_secret_hash == String::from_utf8(secret_hash).unwrap() {
                                *self = CPandas::HomePage(state.clone());
                            } else {
                                state.guild_tips_msg = "password verify error,plase retry again".to_string();
                            }
                        } else {
                            DB.put_secret_hash(input_secret_hash).unwrap();
                            *self = CPandas::HomePage(state.clone());
                        }
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
            CPandas::Guild(state) => { guild_page_view(state) }
        }
    }
}

fn guild_page_view(state: &State) -> Element<Message> {
    let title = text("CPandas")
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Center)
        .vertical_alignment(alignment::Vertical::Center)
        .size(40)
        .color(Color::from([0.5, 0.5, 0.5]));


    let hint_info = text(&state.guild_tips_msg)
        .width(Length::Fill)
        .horizontal_alignment(alignment::Horizontal::Left)
        .vertical_alignment(alignment::Vertical::Center)
        .size(15)
        .color(Color::from([0.9, 0.3, 0.9]));

    let input = text_input("Input secret", &state.input_secret, Message::PasswordValueEdited)
        .padding(15)
        .size(20)
        .on_submit(Message::PasswordComplete);

    let confirm = button(
        text("Confirm")
            .size(20)
            .width(Length::Fill)
            .horizontal_alignment(alignment::Horizontal::Center)
            .vertical_alignment(alignment::Vertical::Center)
            .color(Color::from([0.8, 0.3, 0.9])))
        .width(Length::Fill)
        .padding(10)
        .on_press(Message::PasswordComplete);
    let content = column()
        .spacing(15)
        .width(Length::Fill)
        .push(title)
        .push(hint_info)
        .push(input)
        .push(confirm);


    container(content)
        .width(Length::Fill)
        .height(Length::Fill)
        .padding(20)
        .into()
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
                let mut secret_info = "******".to_string();
                if item.status == 1 {
                    secret_info = item.secret.clone();
                }
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
                            text(&secret_info)
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


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test() {}
}