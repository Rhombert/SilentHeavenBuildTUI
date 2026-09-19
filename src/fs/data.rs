use std::env;
use std::fs::File;
use std::io;

use serde::{Deserialize, Serialize};

use crate::{level::{LevelPlan, LevelStep}, skill::Skill, types::{Strengths, Weaknesses}};

#[derive(Serialize, Deserialize)]
pub struct SaveData {
    pub strengths: Strengths,
    pub weaknesses: Weaknesses,
    pub plan: Vec<LevelStep>,
}

impl SaveData {
    pub fn copy_plan(
        steps: &LevelPlan,
    ) -> Self {
        Self {
            strengths: *steps.get_strengths(),
            weaknesses: *steps.get_weaknesses(),
            plan: steps.get_steps().clone(),
        }
    }

    pub fn save_to(&self, file_name: &str) -> io::Result<File> {
        let exe_path_res = env::current_exe();
        if exe_path_res.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Failed to resolve executable path."
            ))
       }

        let save_path = exe_path_res.unwrap().parent().unwrap().join(file_name);

        let mut file = File::create(save_path).unwrap();

        serde_json::to_writer_pretty(&file, &self);

        Ok(file)
    }

    pub fn load_from(file_name: &str) -> io::Result<SaveData> {
        let exe_path_res = env::current_exe();
        if exe_path_res.is_err() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Failed to resolve executable path."
            ))
       }

        let save_path = exe_path_res.unwrap().parent().unwrap().join(file_name);

        let file = File::open(save_path)?;

        let data: SaveData = serde_json::from_reader(file)
            .map_err(io::Error::other)?;

        Ok(data)
    }
}
