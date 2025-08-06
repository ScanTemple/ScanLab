use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
pub struct RotateParams {
    angle: f32,
}

pub type RotateStage = super::StageData<RotateParams>;
