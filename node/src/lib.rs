use std::os::macos::raw::stat;
use std::time::Duration;

use copypasta::{ClipboardContext, ClipboardProvider};
use eframe::{egui, Storage};
use eframe::egui::{Button, Frame, Hyperlink, Label, Layout, Separator, TextStyle, TopBottomPanel, Ui};
use once_cell::sync::Lazy;
use uuid::Uuid;

use types::{*};
use types::Item;

use crate::db::Database;
use crate::egui::{Align, ScrollArea};

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
    Detail(usize),
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
        render_top_panel(ctx);
        egui::CentralPanel::default().show(ctx, |ui| {
            match self.state {
                State::Guild => { guild_view(self, ctx, ui) }
                State::Home => { home_view(self, ctx, ui) }
                State::New => { new_view(self, ctx, ui) }
                State::Detail(index) => { detail_view(self, ctx, ui, index) }
            }
        });
        render_bottom_panel(ctx)
    }
}


fn guild_view(cp: &mut CPandas, ctx: &egui::Context, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Input Secret: ");
        ui.text_edit_singleline(&mut cp.input_secret);
    });


    if ui.button("Confirm").clicked() {
        log::debug!("confirm submit");

        if cp.input_secret == "" {
            return;
        }


        let secret_hash_opt = DB.get_secret_hash().unwrap();
        let secret_key = utils::get_valid_aes_key(cp.input_secret.clone()).unwrap();
        cp.input_secret = secret_key.clone();
        let input_secret_hash = utils::sha256(secret_key.as_bytes()).unwrap();
        if let Some(secret_hash) = secret_hash_opt {
            if input_secret_hash == String::from_utf8(secret_hash).unwrap() {
                cp.state = State::Home;
            } else {
                log::debug!("password verify fail");
            }
        } else {
            DB.put_secret_hash(input_secret_hash).unwrap();
            cp.state = State::Home;
        }
    }
}

fn home_view(cp: &mut CPandas, ctx: &egui::Context, ui: &mut Ui) {
    navigate_menu_view(cp, ui, ctx);
    ScrollArea::vertical().show(ui, |ui| {
        let result = DB.get_item_list().unwrap();
        for index in 0..cp.items.len() {
            let item = cp.items.get(index).unwrap();
            ui.horizontal(|ui| {
                ui.with_layout(Layout::top_down(Align::LEFT), |ui| {
                    ui.add_space(10.);
                    ui.add(Label::new(format!("{}", index)));
                    ui.add(Label::new(&item.account));
                    ui.add(Hyperlink::new(&item.secret));
                    ui.add(Hyperlink::new(&item.desc));
                    ui.add_space(10.);
                });
                // controls
                ui.with_layout(Layout::right_to_left(), |ui| {
                    // let delete_bt = Button::new("Delete");
                    // if ui.add(delete_bt).clicked() {
                    //     DB.del_item(&item.id).unwrap();
                    // }
                    if ui.add(Button::new("View")).clicked() {
                        cp.state = State::Detail(index);
                    }
                });
            });
        }
    });
}


fn detail_view(cp: &mut CPandas, ctx: &egui::Context, ui: &mut Ui, index: usize) {
    let item = cp.items.get(index).unwrap();
    ui.label(format!("{:?}", index));
    ui.label(&item.account);
    ui.label(&item.secret);
    ui.label(&item.desc);
    let item = item.clone();
    ui.horizontal(|ui| {
        if ui.button("View").clicked() {
            if item.status == 0 {
                let password = utils::aes256_decode(&hex::decode(&item.secret).unwrap(),
                                                    cp.input_secret.clone().as_bytes(),
                                                    &hex::decode(&item.nonce).unwrap()).unwrap();

                println!("{:?}", String::from_utf8(password));
            } else {
                let secret = utils::aes256_encode_with_nonce(
                    item.secret.as_bytes(),
                    cp.input_secret.as_bytes(),
                    &hex::decode(&item.nonce).unwrap(),
                ).unwrap();
                println!("{:?}", String::from_utf8(secret));
            }
        }

        if ui.button("Delete").clicked() {
            DB.del_item(&item.id).unwrap();
            cp.items.remove(index);
            cp.state = State::Home;
        }
        if ui.button("Close").clicked() {
            cp.state = State::Home;
        }
    });
}


fn new_view(cp: &mut CPandas, ctx: &egui::Context, ui: &mut Ui) {
    ui.horizontal(|ui| {
        ui.label("Input Account: ");
        ui.text_edit_singleline(&mut cp.new_temp_item.account_value);
    });
    ui.horizontal(|ui| {
        ui.label("Input Secret: ");
        ui.text_edit_singleline(&mut cp.new_temp_item.secret_value);
    });
    ui.horizontal(|ui| {
        ui.label("Input Desc: ");
        ui.text_edit_singleline(&mut cp.new_temp_item.desc_value);
    });
    ui.horizontal(|ui| {
        if ui.button("Close").clicked() {
            log::debug!("close");
            cp.new_temp_item.clear();
            cp.state = State::Home;
        }
        if ui.button("Submit").clicked() {
            log::debug!("new submit");
            let uuid = Uuid::new_v4().to_string();
            println!("{:?}", cp.new_temp_item);
            println!("{:?}", cp.new_temp_item);
            let (ciphertext, nonce) = utils::aes256_encode(
                cp.new_temp_item.secret_value.as_bytes(),
                cp.input_secret.as_bytes()).unwrap();
            let item = Item {
                id: uuid.clone(),
                account: cp.new_temp_item.account_value.clone(),
                secret: hex::encode(ciphertext),
                desc: cp.new_temp_item.desc_value.clone(),
                status: 0,
                nonce: hex::encode(nonce),
            };
            DB.put_item(&item).unwrap();
            cp.items.push(item);
        }
    });
}


fn navigate_menu_view(cp: &mut CPandas, ui: &mut Ui, ctx: &egui::Context) {
    // define a TopBottomPanel widget
    ui.add_space(10.);
    egui::menu::bar(ui, |ui| {
        // logo
        ui.with_layout(Layout::left_to_right(), |ui| {
            if ui.add(Button::new("Home")).clicked() {}
        });
        // controls
        ui.with_layout(Layout::right_to_left(), |ui| {
            let export_bt = ui.button("Export").on_hover_text("Export to Clipboard");
            if export_bt.clicked() {
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
            let import_bt = ui.button("Import").on_hover_text("Import from Clipboard");

            if import_bt.clicked() {
                let mut ctx = ClipboardContext::new().unwrap();
                let content = ctx.get_contents().unwrap();
                let export: Export = serde_json::from_slice(content.as_bytes()).unwrap();
                let hex_data = hex::decode(export.content).unwrap();
                let list: Vec<Item> = serde_json::from_slice(&hex_data).unwrap();
                for item in list {
                    DB.put_item(&item).unwrap();
                    cp.items.push(item)
                }
                log::debug!("import ok")
            }
            let new_bt = ui.button("New ✚").on_hover_text("Add new Secret");
            if new_bt.clicked() {
                log::debug!("new item");
                cp.state = State::New;
            }
        });
    });
    ui.add_space(10.);
}

fn render_top_panel(ctx: &egui::Context) {
    TopBottomPanel::top("top_panel").show(ctx, |ui| {
        ui.add_space(30.);
        ui.vertical_centered(|ui| {
            ui.heading("CPandas");
        });
        ui.add_space(30.);
    });
}

fn render_bottom_panel(ctx: &egui::Context) {
    TopBottomPanel::bottom("footer").show(ctx, |ui| {
        ui.vertical_centered(|ui| {
            ui.add_space(10.);
            ui.add(Label::new("API source: newsapi.org"));
            ui.add(
                Hyperlink::new("https://github.com/emilk/egui")
            );
            ui.add(
                Hyperlink::new("https://github.com/creativcoder/headlines")
            );
            ui.add_space(10.);
        })
    });
}