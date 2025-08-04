use serde::{Deserialize, Serialize};
use ts_rs::TS;
// use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
pub struct OpenParams {}

pub type OpenStage = super::StageData<OpenParams>;

impl super::GenericStage for OpenStage {
    fn new() -> Self {
        super::StageData { images: vec![] }
    }
}
