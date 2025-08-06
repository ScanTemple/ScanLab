use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
#[ts(export, export_to = "index.ts")]
pub struct SaveParams {}

pub type SaveStage = super::StageData<SaveParams>;
