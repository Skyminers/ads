use super::app_state::AppState;
use super::app_state::TableType;
use egui_extras::{Column, TableBuilder};
use std::sync::{Arc, Mutex};

pub fn table_ui(state: Arc<Mutex<AppState>>, ui: &mut egui::Ui) {
    let available_height = ui.available_height();
    let table_type = {
        let state = state.lock().unwrap();
        state.table_type.clone()
    };
    match table_type {
        TableType::FinalScore => {
            let table = TableBuilder::new(ui)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("学号");
                    });
                    header.col(|ui| {
                        ui.strong("姓名");
                    });
                    header.col(|ui| {
                        ui.strong("总分");
                    });
                    header.col(|ui| {
                        ui.strong("平时分");
                    });
                    header.col(|ui| {
                        ui.strong("期末(40%)");
                    });
                    header.col(|ui| {
                        ui.strong("期中(10%)");
                    });
                    header.col(|ui| {
                        ui.strong("作业(10%)");
                    });
                    header.col(|ui| {
                        ui.strong("讨论(10%)");
                    });
                    header.col(|ui| {
                        ui.strong("Bonus");
                    });
                    header.col(|ui| {
                        ui.strong("讨论分项");
                    });
                    header.col(|ui| {
                        ui.strong("作业分项");
                    });
                })
                .body(|mut body| {
                    let state = state.lock().unwrap();
                    for student in &state.final_table {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(student.student_info.student_id.clone());
                            });
                            row.col(|ui| {
                                ui.label(student.student_info.student_name.clone());
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.final_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.general_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.final_exam_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.midterm_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.homework_score_sum));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.discussion_score_sum));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.bonus_score));
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for score in &student.discussion_score {
                                        ui.label(format!("{:.2}", score));
                                    }
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for score in &student.homework_score {
                                        ui.label(format!("{:.2}", score));
                                    }
                                });
                            });
                        });
                    }
                });
        }
        TableType::ProjectScore => {
            let table = TableBuilder::new(ui)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);
            table.header(20.0, |mut header| {
                header.col(|ui| {
                    ui.strong("组号");
                });
                header.col(|ui| {
                    ui.strong("Project 总分(30)");
                });
                header.col(|ui| {
                    ui.strong("展示分数(6)");
                });
                header.col(|ui| {
                    ui.strong("Bonus 分数");
                });
                header.col(|ui| {
                    ui.strong("Report 总分(20)");
                });
                header.col(|ui| {
                    ui.strong("PR 总分(4)");
                });
                header.col(|ui| {
                    ui.strong("展示报告 ID");
                });
                header.col(|ui| {
                    ui.strong("Report(20)");
                });
                header.col(|ui| {
                    ui.strong("PR(40)");
                });
            }).body(|mut body| {
                let state = state.lock().unwrap();
                    for student in &state.project_table {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(format!("{}", student.group_id.clone()));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.total_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.presentation_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.bonus_score));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.report_score_sum));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:.2}", student.pr_score_sum));
                            });
                            row.col(|ui| {
                                ui.label(format!("{:}", student.presentation_id));
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for score in &student.report_score {
                                        ui.label(format!("{:.2}", score));
                                    }
                                });
                            });
                            row.col(|ui| {
                                ui.horizontal(|ui| {
                                    for score in &student.pr_score {
                                        ui.label(format!("{:.2}", score));
                                    }
                                });
                            });
                        });
                    }
            });
        }
        TableType::GroupList => {
            let table = TableBuilder::new(ui)
                .resizable(true)
                .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .column(Column::auto())
                .min_scrolled_height(0.0)
                .max_scroll_height(available_height);
            table
                .header(20.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("组号");
                    });
                    header.col(|ui| {
                        ui.strong("组长学号");
                    });
                    header.col(|ui| {
                        ui.strong("组长姓名");
                    });
                    header.col(|ui| {
                        ui.strong("组员学号");
                    });
                    header.col(|ui| {
                        ui.strong("组员姓名");
                    });
                    header.col(|ui| {
                        ui.strong("组员学号");
                    });
                    header.col(|ui| {
                        ui.strong("组员姓名");
                    });
                })
                .body(|mut body| {
                    let state = state.lock().unwrap();
                    for student in &state.group_table {
                        body.row(20.0, |mut row| {
                            row.col(|ui| {
                                ui.label(student.group_id.to_string());
                            });
                            row.col(|ui| {
                                ui.label(student.group_members[0].student_id.clone());
                            });
                            row.col(|ui| {
                                ui.label(student.group_members[0].student_name.clone());
                            });
                            row.col(|ui| {
                                ui.label(student.group_members[1].student_id.clone());
                            });
                            row.col(|ui| {
                                ui.label(student.group_members[1].student_name.clone());
                            });
                            if student.group_members.len() > 2 {
                                row.col(|ui| {
                                    ui.label(student.group_members[2].student_id.clone());
                                });
                                row.col(|ui| {
                                    ui.label(student.group_members[2].student_name.clone());
                                });
                            }
                        });
                    }
                });
        }
    }
}
