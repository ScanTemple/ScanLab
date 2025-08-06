use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use ts_rs::TS;
use uuid::Uuid;

mod open;
mod rotate;
mod save;
mod split;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StageData<T> {
    pub images: Vec<ImageInfo<T>>,
    pub images_lookup: HashMap<Uuid, usize>,
    pub current_image: Option<Uuid>,
}

impl<T> StageData<T> {
    pub fn new() -> Self {
        StageData {
            images: vec![],
            images_lookup: HashMap::new(),
            current_image: None,
        }
    }
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ImageInfo<T> {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub params: T,
    pub mode: ImageMode,
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub enum ImageMode {
    #[default]
    Auto,
    Manual,
    Skip,
}

pub trait GenericStage {
    fn new() -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type", rename_all = "camelCase")]
#[ts(export)]
pub enum ProcessingStage {
    Open(open::OpenStage),
    Rotate(rotate::RotateStage),
    Split(split::SplitStage),
    Save(save::SaveStage),
}

impl ProcessingStage {
    pub fn new_stage(stage_type: &str) -> Result<Self> {
        match stage_type {
            "open" => Ok(ProcessingStage::Open(open::OpenStage::new())),
            "rotate" => Ok(ProcessingStage::Rotate(rotate::RotateStage::new())),
            "split" => Ok(ProcessingStage::Split(split::SplitStage::new())),
            "save" => Ok(ProcessingStage::Save(save::SaveStage::new())),
            _ => Err(anyhow::anyhow!("Unknown stage type: {stage_type}")),
        }
    }
}
