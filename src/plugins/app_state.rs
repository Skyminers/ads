use std::num::{ParseFloatError, ParseIntError};
use std::sync::{Arc, Mutex};
use std::fs::File;
use std::io::{self, Write, Read};
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

#[derive(Serialize, Deserialize)]
pub struct Log {
    time: DateTime<Utc>,
    function_info: String,
    message: String,
}

impl Log {
    pub fn new(info: String, message: String) -> Self {
        Self {
            time: Utc::now(),
            function_info: info,
            message,
        }
    }

    pub fn to_string(&self) -> String {
        let time = self.time.format("%Y-%m-%d %H:%M:%S").to_string();
        format!("{} - {} \n{}", time, self.function_info, self.message)
    }
}

#[derive(Serialize, Deserialize)]
pub enum TableType {
    FinalScore,
    ProjectScore,
    GroupList,
}

impl Default for TableType {
    fn default() -> Self {
        TableType::FinalScore
    }
}

impl Clone for TableType {
    fn clone(&self) -> Self {
        match self {
            TableType::FinalScore => TableType::FinalScore,
            TableType::ProjectScore => TableType::ProjectScore,
            TableType::GroupList => TableType::GroupList,
        }
    }
}


#[derive(Serialize, Deserialize)]
pub struct GroupItem {
    pub group_id: usize,
    pub group_members: Vec<StudentItem>,
}

impl Default for GroupItem {
    fn default() -> Self {
        Self {
            group_id: 0,
            group_members: vec![
                StudentItem {
                    student_id: "".to_string(),
                    student_name: "default".to_string(),
                },
                StudentItem {
                    student_id: "".to_string(),
                    student_name: "default".to_string(),
                },
                StudentItem {
                    student_id: "".to_string(),
                    student_name: "default".to_string(),
                },
            ],
        }
    }
}

impl GroupItem {
    pub fn to_vec(&self) -> Vec<String> {
        let mut vec = vec![self.group_id.to_string()];
        for member in &self.group_members {
            vec.push(member.student_id.clone());
            vec.push(member.student_name.clone());
        }
        vec
    }

    pub fn from_vec(vec: &Vec<String>) -> Result<Self, String> {
        if vec.len() < 1 || (vec.len() - 1) % 2 != 0 {
            return Err("Input vector length is invalid".to_string());
        }

        let group_id = vec[0].parse::<usize>().map_err(|e| e.to_string())?;
        let mut group_members = Vec::new();

        for i in (1..vec.len()).step_by(2) {
            if i + 1 >= vec.len() {
                return Err("Missing student name for student ID".to_string());
            }
            group_members.push(StudentItem {
                student_id: vec[i].clone(),
                student_name: vec[i + 1].clone(),
            });
        }

        Ok(Self {
            group_id,
            group_members,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct StudentItem {
    pub student_id: String,
    pub student_name: String,
}

#[derive(Serialize, Deserialize)]
pub struct ProjectItem {
    pub group_id: usize,
    pub total_score: f32,
    pub presentation_score: f32,
    pub bonus_score: f32,
    pub report_score_sum: f32,
    pub pr_score_sum: f32,
    pub presentation_id: usize,
    pub report_score: Vec<f32>,
    pub pr_score: Vec<f32>,
}

impl Default for ProjectItem {
    fn default() -> Self {
        Self {
            group_id: 0,
            total_score: 0.0,
            presentation_score: 0.0,
            bonus_score: 0.0,
            report_score_sum: 0.0,
            pr_score_sum: 0.0,
            presentation_id: 0,
            report_score: vec![0.0, 0.0, 0.0],
            pr_score: vec![0.0, 0.0, 0.0],
        }
    }
}

impl ProjectItem {
    pub fn calaculate(&mut self) {
        self.pr_score_sum = (self.pr_score.iter().sum::<f32>()) / self.pr_score.len() as f32 / 10.0;
        self.report_score_sum = 0.0;
        let (mut max_score, mut sum_score): (f32, f32) = (0.0, 0.0);
        for (index, score) in self.report_score.iter().enumerate() {
            if index + 1 == self.presentation_id {
                self.report_score_sum += score * 0.5;
            } else {
                max_score = max_score.max(*score);
                sum_score += score;
            }
        }
        sum_score -= max_score;
        self.bonus_score = sum_score / 20.0;
        self.report_score_sum += max_score * 0.5;
        self.total_score = self.report_score_sum + self.pr_score_sum + self.presentation_score;
    }

    pub fn to_vec(&self) -> Vec<String> {
        let mut vec = vec![self.group_id.to_string()];
        vec.push(self.total_score.to_string());
        vec.push(self.presentation_score.to_string());
        vec.push(self.bonus_score.to_string());
        vec.push(self.report_score_sum.to_string());
        vec.push(self.pr_score_sum.to_string());
        vec.push(self.presentation_id.to_string());
        for score in &self.report_score {
            vec.push(score.to_string());
        }
        vec.push("reportEnd".to_string());
        for score in &self.pr_score {
            vec.push(score.to_string());
        }
        vec.push("prEnd".to_string());
        vec
    }

    pub fn from_vec(vec: &Vec<String>) -> Result<Self, String> {
        if vec.len() < 8 {
            return Err("Input vector is too short".to_string());
        }

        let parse_usize = |s: &str| -> Result<usize, ParseIntError> {
            s.parse::<usize>()
        };
        let parse_f32 = |s: &str| -> Result<f32, ParseFloatError> {
            s.parse::<f32>()
        };

        let group_id = parse_usize(&vec[0]).map_err(|e| e.to_string())?;
        let total_score = parse_f32(&vec[1]).map_err(|e| e.to_string())?;
        let presentation_score = parse_f32(&vec[2]).map_err(|e| e.to_string())?;
        let bonus_score = parse_f32(&vec[3]).map_err(|e| e.to_string())?;
        let report_score_sum = parse_f32(&vec[4]).map_err(|e| e.to_string())?;
        let pr_score_sum = parse_f32(&vec[5]).map_err(|e| e.to_string())?;
        let presentation_id = parse_usize(&vec[6]).map_err(|e| e.to_string())?;

        let mut report_score = Vec::new();
        let mut pr_score = Vec::new();
        let mut i = 7;

        while i < vec.len() && vec[i] != "reportEnd" {
            report_score.push(parse_f32(&vec[i]).map_err(|e| e.to_string())?);
            i += 1;
        }

        if i >= vec.len() || vec[i] != "reportEnd" {
            return Err("Missing 'reportEnd' marker".to_string());
        }

        i += 1;

        while i < vec.len() && vec[i] != "prEnd" {
            pr_score.push(parse_f32(&vec[i]).map_err(|e| e.to_string())?);
            i += 1;
        }

        if i >= vec.len() || vec[i] != "prEnd" {
            return Err("Missing 'prEnd' marker".to_string());
        }

        Ok(Self {
            group_id,
            total_score,
            presentation_score,
            bonus_score,
            report_score_sum,
            pr_score_sum,
            presentation_id,
            report_score,
            pr_score,
        })
    }
}

#[derive(Serialize, Deserialize)]
pub struct FinalItem {
    pub student_info: StudentItem,
    pub final_score: f32,
    pub general_score: f32,
    pub final_exam_score: f32,
    pub project_score_sum: f32,
    pub midterm_score: f32,
    pub homework_score_sum: f32,
    pub discussion_score_sum: f32,
    pub bonus_score: f32,
    pub discussion_score: Vec<f32>,
    pub homework_score: Vec<f32>,
}

impl Default for FinalItem {
    fn default() -> Self {
        Self {
            student_info: StudentItem {
                student_id: "".to_string(),
                student_name: "default".to_string(),
            },
            final_score: 0.0,
            general_score: 0.0,
            final_exam_score: 0.0,
            project_score_sum: 0.0,
            midterm_score: 0.0,
            homework_score_sum: 0.0,
            discussion_score_sum: 0.0,
            bonus_score: 0.0,
            discussion_score: vec![0.0, 0.0, 0.0],
            homework_score: vec![0.0, 0.0, 0.0],
        }
    }
}

impl FinalItem {
    pub fn calaculate(&mut self) {
        self.discussion_score_sum = ((self.discussion_score.iter().sum::<f32>()) / self.discussion_score.len() as f32) * 10.0;
        self.homework_score_sum = (self.homework_score.iter().sum::<f32>()) / 25.0;
        self.general_score = self.discussion_score_sum 
                            + self.homework_score_sum 
                            + self.final_exam_score.max(self.midterm_score) / 10.0
                            + self.project_score_sum
                            + self.bonus_score;
        self.general_score = self.general_score.min(60.0);
        self.final_score = if self.final_exam_score < 40.0 {
            0.0
        } else {
            self.general_score + self.final_exam_score / 100.0 * 40.0
        }
    }
    pub fn to_vec(&self) -> Vec<String> {
        let mut vec = vec![self.student_info.student_id.clone(), self.student_info.student_name.clone()];
        vec.push(self.final_score.to_string());
        vec.push(self.general_score.to_string());
        vec.push(self.final_exam_score.to_string());
        vec.push(self.project_score_sum.to_string());
        vec.push(self.midterm_score.to_string());
        vec.push(self.homework_score_sum.to_string());
        vec.push(self.discussion_score_sum.to_string());
        vec.push(self.bonus_score.to_string());
        for score in &self.discussion_score {
            vec.push(score.to_string());
        }
        vec.push("discussionEnd".to_string());
        for score in &self.homework_score {
            vec.push(score.to_string());
        }
        vec.push("homeworkEnd".to_string());
        vec
    }

    pub fn from_vec(vec: &Vec<String>) -> Result<Self, String> {
        if vec.len() < 10 {
            return Err("数据不完整".to_string());
        }
        let student_info = StudentItem {
            student_id: vec[0].clone(),
            student_name: vec[1].clone(),
        };
        let parse_f32 = |s: &str| -> Result<f32, ParseFloatError> {
            s.parse::<f32>()
        };
        let final_score = parse_f32(&vec[2]).map_err(|e| e.to_string())?;
        let general_score = parse_f32(&vec[3]).map_err(|e| e.to_string())?;
        let final_exam_score = parse_f32(&vec[4]).map_err(|e| e.to_string())?;
        let project_score_sum = parse_f32(&vec[5]).map_err(|e| e.to_string())?;
        let midterm_score = parse_f32(&vec[6]).map_err(|e| e.to_string())?;
        let homework_score_sum = parse_f32(&vec[7]).map_err(|e| e.to_string())?;
        let discussion_score_sum = parse_f32(&vec[8]).map_err(|e| e.to_string())?;
        let bonus_score = parse_f32(&vec[9]).map_err(|e| e.to_string())?;

        let mut discussion_score = Vec::new();
        let mut homework_score = Vec::new();
        let mut i = 10;
        while i < vec.len() && vec[i] != "discussionEnd" {
            discussion_score.push(parse_f32(&vec[i]).map_err(|e| e.to_string())?);
            i += 1;
        }
        if i >= vec.len() || vec[i] != "discussionEnd" {
            return Err("Missing 'discussionEnd' marker".to_string());
        }
        i += 1;

        while i < vec.len() && vec[i] != "homeworkEnd" {
            homework_score.push(parse_f32(&vec[i]).map_err(|e| e.to_string())?);
            i += 1;
        }

        if i >= vec.len() || vec[i] != "homeworkEnd" {
            return Err("Missing 'homeworkEnd' marker".to_string());
        }
        Ok(Self {
            student_info,
            final_score,
            general_score,
            final_exam_score,
            project_score_sum,
            midterm_score,
            homework_score_sum,
            discussion_score_sum,
            bonus_score,
            discussion_score,
            homework_score,
        })
    }
}

#[derive(Serialize, Deserialize, Default)]
pub struct AppState {
    // 在这里添加你需要存储的应用程序信息
    pub input_text: String,
    pub selected_k: usize,
    pub group_size: usize,
    pub output_text: Vec<Log>,
    pub table_type: TableType,
    pub final_table: Vec<FinalItem>,
    pub project_table: Vec<ProjectItem>,
    pub group_table: Vec<GroupItem>,
}

pub struct AppSingleton;

impl AppSingleton {
    pub fn instance() -> Arc<Mutex<AppState>> {
        static mut SINGLETON: Option<Arc<Mutex<AppState>>> = None;
        static INIT: std::sync::Once = std::sync::Once::new();

        unsafe {
            INIT.call_once(|| {
                let state = AppSingleton::load_state().unwrap_or_default();
                SINGLETON = Some(Arc::new(Mutex::new(state)));
            });
            SINGLETON.clone().unwrap()
        }
    }

    pub fn load_state() -> io::Result<AppState> {
        let mut file = File::open("app_state.json").unwrap_or_else(|_| File::create("app_state.json").unwrap());
        let mut data = String::new();
        file.read_to_string(&mut data)?;
        let state: AppState = serde_json::from_str(&data).unwrap_or_default();
        Ok(state)
    }

    pub fn save_state(state: &AppState) -> io::Result<()> {
        let data = serde_json::to_string(state)?;
        let mut file = File::create("app_state.json")?;
        file.write_all(data.as_bytes())?;
        Ok(())
    }
}