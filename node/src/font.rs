use eframe::egui;

pub struct FontManager {}

impl FontManager {
    pub fn load_custom_fonts(ctx: &egui::Context, path: String) {
        // Start with the default fonts (we will be adding to them rather than replacing them).
        let mut fonts = egui::FontDefinitions::default();

        // Install my own font (maybe supporting non-latin characters).
        // .ttf and .otf files supported.
        fonts.font_data.insert(
            "my_font".to_owned(),
            egui::FontData::from_static(include_bytes!("../../fonts/Hack-Regular.ttf")),
        );

        // Put my font first (highest priority) for proportional text:
        fonts
            .families
            .entry(egui::FontFamily::Proportional)
            .or_default()
            .insert(0, "my_font".to_owned());

        // Put my font as last fallback for monospace:
        fonts
            .families
            .entry(egui::FontFamily::Monospace)
            .or_default()
            .push("my_font".to_owned());

        // Tell egui to use these fonts:
        ctx.set_fonts(fonts);
    }
}

