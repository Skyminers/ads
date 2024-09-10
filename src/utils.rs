

pub fn load_fonts(ctx: &egui::Context) {
    let mut fonts = egui::FontDefinitions::default();
    fonts.font_data.insert("my_font".to_owned(),
    egui::FontData::from_static(include_bytes!("fonts/hei.ttf")));
    fonts.families.get_mut(&egui::FontFamily::Proportional).unwrap()
        .insert(0, "my_font".to_owned());
    fonts.families.get_mut(&egui::FontFamily::Monospace).unwrap()
        .push("my_font".to_owned());
    ctx.set_fonts(fonts);
}

pub fn split_to_table(input_text: String) -> Vec<Vec<String>>{
    input_text
        .lines()
        .map(|line| {
            line.split(|c| c == ',' || c == '\t')
                .map(|s| s.trim().to_string())
                .collect()
        })
        .collect()
}
