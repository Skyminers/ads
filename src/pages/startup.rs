use crate::{
    plugins::{
        app_state::{AppSingleton, FinalItem, GroupItem, Log, ProjectItem, TableType},
        build_table_data::{
            build_final_from_input, 
            build_group_from_input, 
            build_project_from_group,
            find_student_no_group,
        },
        random_group::random_group,
        random_selection::random_selection,
        table::table_ui,
    },
    utils::{load_fonts, split_to_table},
};
use eframe::egui;

#[derive(Default)]
pub struct MyEguiApp {}

impl MyEguiApp {
    pub fn new(cc: &eframe::CreationContext<'_>) -> Self {
        // Customize egui here with cc.egui_ctx.set_fonts and cc.egui_ctx.set_visuals.
        // Restore app state using cc.storage (requires the "persistence" feature).
        // Use the cc.gl (a glow::Context) to create graphics shaders and buffers that you can use
        // for e.g. egui::PaintCallback.
        load_fonts(&cc.egui_ctx);
        Self::default()
    }
}

impl eframe::App for MyEguiApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            egui::ScrollArea::vertical().show(ui, |ui| {
                let state = AppSingleton::instance();

                ui.horizontal(|ui| {
                    ui.vertical(|ui| {
                        ui.heading("IO 区域");
                        ui.label("输入区域");
                        let mut state = state.lock().unwrap();
                        egui::TextEdit::multiline(&mut state.input_text)
                            .hint_text("输入的数据以英文逗号或换行进行分割")
                            .desired_width(500.0)
                            .desired_rows(10)
                            .show(ui);
                        ui.label("输出区域");
                        egui::ScrollArea::vertical()
                            .min_scrolled_height(100.0)
                            .max_width(500.0)
                            .show(ui, |ui| {
                                for log in (&state).output_text.iter().rev() {
                                    ui.label(log.to_string());
                                }
                            });
                    });
                    ui.separator();
                    ui.vertical(|ui| {
                        ui.heading("功能区域");

                        ui.label("以下功能仅影响输入区");
                        // 随机选取 K 个元素
                        random_selection(state.clone(), ui);

                        // 随机分组，大小 K
                        random_group(state.clone(), ui);

                        // 从名单(学号，姓名)构造成绩表
                        build_final_from_input(state.clone(), ui);

                        // 导入组队名单
                        build_group_from_input(state.clone(), ui);

                        ui.separator();

                        ui.label("以下功能从数据表读取");

                        // 从组队名单构造 Project 表
                        build_project_from_group(state.clone(), ui);

                        // 筛选未组队同学
                        find_student_no_group(state.clone(), ui);
                    });
                });

                ui.separator();

                // table
                {
                    ui.horizontal(|ui| {
                        let mut state = state.lock().unwrap();

                        if ui.button("总评成绩").clicked() {
                            state.table_type = TableType::FinalScore;
                        }
                        if ui.button("Project 成绩").clicked() {
                            state.table_type = TableType::ProjectScore;
                        }
                        if ui.button("组队名单").clicked() {
                            state.table_type = TableType::GroupList;
                        }
                        if ui.button("Swap").clicked() {
                            let input_text = state.input_text.clone();
                            let intput_vec = split_to_table(input_text);
                            match state.table_type {
                                TableType::FinalScore => {
                                    let new_table: Vec<FinalItem> = intput_vec
                                        .iter()
                                        .map(|row| match FinalItem::from_vec(row) {
                                            Ok(item) => item,
                                            Err(e) => {
                                                state.output_text.push(Log::new(
                                                    "Swap Info".to_string(),
                                                    e.to_string(),
                                                ));
                                                FinalItem::default()
                                            }
                                        })
                                        .collect();
                                    let table_vec: Vec<String> = state
                                        .final_table
                                        .iter()
                                        .map(|row| row.to_vec().join(","))
                                        .collect();
                                    state.input_text = table_vec.join("\n");
                                    state.final_table = new_table;
                                }
                                TableType::ProjectScore => {
                                    let new_table: Vec<ProjectItem> = intput_vec
                                        .iter()
                                        .map(|row| match ProjectItem::from_vec(row) {
                                            Ok(item) => item,
                                            Err(e) => {
                                                state.output_text.push(Log::new(
                                                    "Swap Info".to_string(),
                                                    e.to_string(),
                                                ));
                                                ProjectItem::default()
                                            }
                                        })
                                        .collect();
                                    let table_vec: Vec<String> = state
                                        .project_table
                                        .iter()
                                        .map(|row| row.to_vec().join(", "))
                                        .collect();
                                    state.input_text = table_vec.join("\n");
                                    state.project_table = new_table;
                                }
                                TableType::GroupList => {
                                    let new_table: Vec<GroupItem> = intput_vec
                                        .iter()
                                        .map(|row| match GroupItem::from_vec(row) {
                                            Ok(item) => item,
                                            Err(e) => {
                                                state.output_text.push(Log::new(
                                                    "Swap Info".to_string(),
                                                    e.to_string(),
                                                ));
                                                GroupItem::default()
                                            }
                                        })
                                        .collect();
                                    let table_vec: Vec<String> = state
                                        .group_table
                                        .iter()
                                        .map(|row| row.to_vec().join(", "))
                                        .collect();
                                    state.input_text = table_vec.join("\n");
                                    state.group_table = new_table;
                                }
                            }
                        }
                        if ui.button("统计分数").clicked() {
                            match state.table_type {
                                TableType::GroupList => {
                                    state.output_text.push(Log::new(
                                        "统计分数 - GroupList".to_string(),
                                        "统计完毕".to_string(),
                                    ));
                                }
                                TableType::ProjectScore => {
                                    let project_table = &mut state.project_table;
                                    for row in project_table.iter_mut() {
                                        row.calaculate();
                                    }
                                    state.output_text.push(Log::new(
                                        "统计分数 - ProjectScore".to_string(),
                                        "重新计算 Project总分, Bonus分数, ".to_string(),
                                    ));
                                }
                                TableType::FinalScore => {
                                    let project_table: &mut Vec<FinalItem> = &mut state.final_table;
                                    for row in project_table.iter_mut() {
                                        row.calaculate();
                                    }
                                    state.output_text.push(Log::new(
                                        "统计分数 - FinalScore".to_string(),
                                        "重新计算 总分, 平时分, 作业分, 讨论分".to_string(),
                                    ));
                                }
                            }
                        }
                    });
                    use egui_extras::{Size, StripBuilder};
                    StripBuilder::new(ui)
                        .size(Size::remainder().at_least(100.0)) // for the table
                        .vertical(|mut strip| {
                            strip.cell(|ui| {
                                egui::ScrollArea::horizontal().show(ui, |ui| {
                                    table_ui(state.clone(), ui);
                                });
                            });
                        });
                }

                let state = state.lock().unwrap();
                if let Err(e) = AppSingleton::save_state(&state) {
                    eprintln!("Failed to save state: {}", e);
                };
            });
        });
    }
}
