use crate::stages::ProcessingStage;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub files: Vec<ImageDirectory>,
    pub stages: Vec<ProcessingStage>,

    pub last_seen_stage: u32,
    pub last_seen_image: u32,

    #[serde(skip)]
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageDirectory {
    pub path: String,
    pub images: Vec<ImageFile>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageFile {
    pub path: String,
    pub size: (u32, u32),
}

impl Project {
    pub fn new() -> Self {
        Self {
            files: vec![],
            stages: vec![],

            last_seen_stage: 0,
            last_seen_image: 0,

            file_path: None,
        }
    }

    pub fn save_to_file(&mut self, path: Option<PathBuf>) -> Result<()> {
        let file_path = match path {
            Some(p) => {
                self.file_path = Some(p.clone());
                p
            }
            None => self
                .file_path
                .as_ref()
                .context("No file path specified")?
                .clone(),
        };

        let json = serde_json::to_string_pretty(self)?;
        fs::write(&file_path, json)?;
        Ok(())
    }

    pub fn load_from_file(path: PathBuf) -> Result<Self> {
        let json = fs::read_to_string(&path)?;
        let mut project: Project = serde_json::from_str(&json)?;
        project.file_path = Some(path);
        Ok(project)
    }

    pub fn save(&mut self) -> Result<()> {
        self.save_to_file(None)
    }

    // pub fn add_stage(&mut self, stage: ProcessingStage) {
    //     self.stages.push(stage);
    // }
}
