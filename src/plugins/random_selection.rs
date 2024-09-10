use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;

use super::app_state::*;

pub fn random_selection(state: Arc<Mutex<AppState>>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("随机抽取").clicked() {
            let input_text = {
                let state = state.lock().unwrap();
                state.input_text.clone()
            };

            let mut numbers: Vec<&str> = input_text
                .split(|c| c == '\n' || c == ',' || c == '\t')
                .map(|s| s.trim())
                .filter(|s| !s.is_empty())
                .collect();

            let mut rng = rand::thread_rng();
            numbers.shuffle(&mut rng);

            let selected_k = {
                let state = state.lock().unwrap();
                state.selected_k
            };

            let selected_numbers: Vec<&str> =
                numbers.into_iter().take(selected_k).collect();

            {
                let mut state = state.lock().unwrap();
                state.output_text.push(Log::new(
                    format!("随机抽取{}个", selected_k),
                    format!("{:?}", selected_numbers),
                ));
            }
        }

        egui::ComboBox::from_label("选择抽取数量")
            .selected_text(format!("{}", state.lock().unwrap().selected_k))
            .show_ui(ui, |ui| {
                for i in 1..=10 {
                    ui.selectable_value(
                        &mut state.lock().unwrap().selected_k,
                        i,
                        format!("{}", i),
                    );
                }
            });
    });
}