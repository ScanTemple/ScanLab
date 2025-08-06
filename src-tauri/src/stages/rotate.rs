use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct RotateParams {
    angle: f32,
}

pub type RotateStage = super::StageData<RotateParams>;
