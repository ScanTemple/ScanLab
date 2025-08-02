use crate::pipeline::{open, rotate, save};
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::{fs, path::PathBuf};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub pipeline: ProcessingPipeline,

    #[serde(skip)]
    pub file_path: Option<PathBuf>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProcessingPipeline {
    pub steps: Vec<ProcessingStage>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProcessingStage {
    Open(open::OpenParams),
    Rotate(rotate::RotateParams),
    Save(save::SaveParams),
}

impl Project {
    pub fn new() -> Self {
        Self {
            pipeline: ProcessingPipeline { steps: vec![] },
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

    pub fn get_file_path(&self) -> Option<&PathBuf> {
        self.file_path.as_ref()
    }

    pub fn set_file_path(&mut self, path: PathBuf) {
        self.file_path = Some(path)
    }
}
