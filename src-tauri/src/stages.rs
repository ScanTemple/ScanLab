use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;

mod open;
mod rotate;
mod save;
mod split;

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct StageData<T> {
    pub images: Vec<ImageInfo<T>>,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export)]
pub struct ImageInfo<T> {
    pub id: Uuid,
    pub parent_id: Option<Uuid>,
    pub params: T,
}

#[derive(Debug, Clone, Serialize, Deserialize, TS)]
#[serde(tag = "type")]
#[ts(export)]
pub enum ProcessingStage {
    #[serde(rename = "open")]
    Open(StageData<open::OpenParams>),

    #[serde(rename = "rotate")]
    Rotate(StageData<rotate::RotateParams>),

    #[serde(rename = "split")]
    Split(StageData<split::SplitParams>),

    #[serde(rename = "save")]
    Save(StageData<save::SaveParams>),
}
