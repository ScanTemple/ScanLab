use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
pub struct SaveParams {}

pub type SaveStage = super::StageData<SaveParams>;
