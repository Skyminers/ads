use std::sync::{Arc, Mutex};

use crate::utils::{split_to_table, vec_to_string};

use super::app_state::*;

pub fn build_final_from_input(state: Arc<Mutex<AppState>>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("从名单(学号，成绩)构造成绩表").clicked() {
            let input_text = {
                let state = state.lock().unwrap();
                state.input_text.clone()
            };

            let table = split_to_table(input_text);
            let new_text: Vec<Vec<String>> = table.iter().map(|row| {
                FinalItem::from_id_name(row).to_vec()
            }).collect();
            let new_text = vec_to_string(new_text);
            let mut state = state.lock().unwrap();
            state.input_text = new_text;
        }
    });
}

pub fn build_group_from_input(state: Arc<Mutex<AppState>>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("组队名单(id 学号1 姓名1...)格式转换").clicked() {
            let input_text = {
                let state = state.lock().unwrap();
                state.input_text.clone()
            };

            let table = split_to_table(input_text);
            let new_text: Vec<Vec<String>> = table.iter().map(|row| {
                GroupItem::from_raw(row).to_vec()
            }).collect();
            let new_text = vec_to_string(new_text);
            let mut state = state.lock().unwrap();
            state.input_text = new_text;
        }
    });
}

pub fn build_project_from_group(state: Arc<Mutex<AppState>>, ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        if ui.button("从组队名单构造 Project 表").clicked() {
            let group_num = {
                let state = state.lock().unwrap();
                state.group_table.len()
            };

            let new_text: Vec<Vec<String>> = (1..=group_num).map(|id: usize| {
                let mut item = ProjectItem::default();
                item.group_id = id;
                item.to_vec()
            }).collect();
            let new_text = vec_to_string(new_text);
            let mut state = state.lock().unwrap();
            state.input_text = new_text;
        }
    });
}