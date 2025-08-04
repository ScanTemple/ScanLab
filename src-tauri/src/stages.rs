use anyhow::Result;
use serde::{Deserialize, Serialize};
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
}

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ImageInfo<T> {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub params: T,
}

pub trait GenericStage {
    fn new() -> Self;
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum ProcessingStage {
    #[serde(rename = "open")]
    Open(open::OpenStage),

    #[serde(rename = "rotate")]
    Rotate(rotate::RotateStage),

    #[serde(rename = "split")]
    Split(split::SplitStage),

    #[serde(rename = "save")]
    Save(save::SaveStage),
}

impl ProcessingStage {
    pub fn new_stage(stage_type: &str) -> Result<Self> {
        match stage_type {
            "open" => Ok(ProcessingStage::Open(open::OpenStage::new())),
            "rotate" => Ok(ProcessingStage::Rotate(rotate::RotateStage::new())),
            "split" => Ok(ProcessingStage::Split(split::SplitStage::new())),
            "save" => Ok(ProcessingStage::Save(save::SaveStage::new())),
            _ => Err(anyhow::anyhow!("Unknown stage type: {}", stage_type)),
        }
    }
}
