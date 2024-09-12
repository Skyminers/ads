mod pages {
    pub mod startup;
}
mod plugins {
    pub mod app_state;
    pub mod build_table_data;
    pub mod random_group;
    pub mod random_selection;
    pub mod table;
}
pub mod utils;

use pages::startup::MyEguiApp;

fn main() -> Result<(), eframe::Error> {
    let native_options = eframe::NativeOptions::default();
    eframe::run_native(
        "ADS 助教终端",
        native_options,
        Box::new(|cc| Ok(Box::new(MyEguiApp::new(cc)))),
    )
}
