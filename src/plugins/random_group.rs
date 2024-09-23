use std::sync::{Arc, Mutex};
use rand::seq::SliceRandom;

use crate::utils::vec_to_string;

use super::app_state::*;

pub fn random_group(state: Arc<Mutex<AppState>>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("随机组队").clicked() {
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

            let group_size = {
                let state = state.lock().unwrap();
                state.group_size
            };

            let selected_numbers: Vec<Vec<&str>> = numbers
                .chunks(group_size)
                .map(|chunk| chunk.to_vec())
                .collect();

            let selected_numbers = vec_to_string(
                selected_numbers.iter().map(
                    |row| row.iter().map(
                        |s| s.to_string()
                    ).collect()
                ).collect()
            );

            {
                let mut state = state.lock().unwrap();
                state.output_text.push(Log::new(
                    format!("随机分组，大小{}", group_size),
                    selected_numbers,
                ));
            }
        }

        egui::ComboBox::from_label("选择每组大小")
            .selected_text(format!("{}", state.lock().unwrap().group_size))
            .show_ui(ui, |ui| {
                for i in 1..=10 {
                    ui.selectable_value(
                        &mut state.lock().unwrap().group_size,
                        i,
                        format!("{}", i),
                    );
                }
            });
    });
}