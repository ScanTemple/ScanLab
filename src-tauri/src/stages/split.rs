use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
pub struct SplitParams {
    x1: u32,
    x2: u32,
}

pub type SplitStage = super::StageData<SplitParams>;

impl super::GenericStage for SplitStage {
    fn new() -> Self {
        super::StageData { images: vec![] }
    }
}
