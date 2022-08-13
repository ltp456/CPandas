use std::os::macos::raw::stat;

use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{egui, Storage};
use eframe::egui::{Frame, Hyperlink, Label, Separator, TextStyle};
use once_cell::sync::Lazy;
use uuid::Uuid;

use types::{*};
use types::Item;

use crate::db::Database;
use crate::egui::ScrollArea;

static DB: Lazy<Database> = Lazy::new(|| {
    let database = Database::new(".db").unwrap();
    database
});


mod utils;
mod types;
mod db;
mod constants;

#[derive(Debug, PartialEq)]
enum State {
    Guild,
    Home,
    New,
}

pub struct CPandas {
    items: Vec<Item>,
    input_secret: String,
    new_temp_item: InputItem,
    state: State,
}

impl CPandas {
    pub fn new() -> Self {
        let tmp_items = DB.get_item_list();
        let mut items: Vec<Item> = Vec::new();
        if let Ok(item_list) = tmp_items {
            if let Some(list) = item_list {
                items = list
            }
        };

        Self {
            items,
            input_secret: "abcd1234".to_string(),
            new_temp_item: Default::default(),
            state: State::Guild,
        }
    }
}

impl eframe::App for CPandas {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        log::debug!("egui update");
        match self.state {
            State::Guild => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.vertical_centered(|ui| {
                        ui.heading("CPandas");
                    });
                    let sep = Separator::default().spacing(20.);
                    ui.add(sep);
                    ui.horizontal(|ui| {
                        ui.label("Input Secret: ");
                        ui.text_edit_singleline(&mut self.input_secret);
                    });

                    if ui.button("Confirm").clicked() {
                        log::debug!("confirm submit");

                        let secret_hash_opt = DB.get_secret_hash().unwrap();
                        let secret_key = utils::get_valid_aes_key(self.input_secret.clone()).unwrap();
                        self.input_secret = secret_key.clone();
                        let input_secret_hash = utils::sha256(secret_key.as_bytes()).unwrap();
                        if let Some(secret_hash) = secret_hash_opt {
                            if input_secret_hash == String::from_utf8(secret_hash).unwrap() {
                                self.state = State::Home;
                            } else {
                                log::debug!("password verify fail");
                            }
                        } else {
                            DB.put_secret_hash(input_secret_hash).unwrap();
                            self.state = State::Home;
                        }
                    }
                });
            }

            State::Home => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("CPandas");
                    ui.horizontal(|ui| {
                        if ui.button("New").clicked() {
                            log::debug!("new item");
                            self.state = State::New;
                        }
                        if ui.button("Bakup").clicked() {
                            let result = DB.get_item_list().unwrap();
                            if let Some(list) = result {
                                let bakup_res = serde_json::to_string(&list).unwrap();
                                let hex_bak = hex::encode(bakup_res);
                                let export = Export::new("1".to_string(), hex_bak, "".to_string());
                                let export = serde_json::to_string(&export).unwrap();
                                let mut ctx = ClipboardContext::new().unwrap();
                                ctx.set_contents(export).unwrap();
                            }
                        }
                        if ui.button("Import").clicked() {
                            let mut ctx = ClipboardContext::new().unwrap();
                            let content = ctx.get_contents().unwrap();
                            let export: Export = serde_json::from_slice(content.as_bytes()).unwrap();
                            let hex_data = hex::decode(export.content).unwrap();
                            let list: Vec<Item> = serde_json::from_slice(&hex_data).unwrap();
                            for item in list {
                                DB.put_item(&item).unwrap();
                                self.items.push(item)
                            }
                            log::debug!("import ok")
                        }
                        if ui.button("About").clicked() {
                            log::debug!("new item");
                        }
                    });

                    ScrollArea::vertical().show(ui, |ui| {
                        for item in &self.items {
                            ui.add_space(10.);
                            ui.add(Label::new(&item.account));
                            ui.add(Hyperlink::new(&item.secret));
                            ui.add(Hyperlink::new(&item.desc));
                            ui.add_space(10.);
                        }
                    });
                });
            }

            State::New => {
                egui::CentralPanel::default().show(ctx, |ui| {
                    ui.heading("CPandas");
                    ui.horizontal(|ui| {
                        ui.label("Input Account: ");
                        ui.text_edit_singleline(&mut self.new_temp_item.account_value);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Input Secret: ");
                        ui.text_edit_singleline(&mut self.new_temp_item.secret_value);
                    });
                    ui.horizontal(|ui| {
                        ui.label("Input Desc: ");
                        ui.text_edit_singleline(&mut self.new_temp_item.desc_value);
                    });
                    ui.horizontal(|ui| {
                        if ui.button("Close").clicked() {
                            log::debug!("close");
                            self.new_temp_item.clear();
                            self.state = State::Home;
                        }
                        if ui.button("Submit").clicked() {
                            log::debug!("new submit");
                            let uuid = Uuid::new_v4().to_string();
                            println!("{:?}", self.new_temp_item);
                            println!("{:?}", self.new_temp_item);
                            let (ciphertext, nonce) = utils::aes256_encode(
                                self.new_temp_item.secret_value.as_bytes(),
                                self.input_secret.as_bytes()).unwrap();
                            let item = Item {
                                id: uuid.clone(),
                                account: self.new_temp_item.account_value.clone(),
                                secret: hex::encode(ciphertext),
                                desc: self.new_temp_item.desc_value.clone(),
                                status: 0,
                                nonce: hex::encode(nonce),
                            };
                            DB.put_item(&item).unwrap();
                            self.items.push(item);
                        }
                    });
                });
            }
        }
    }
}
