pub mod open;
pub mod rotate;
pub mod save;

use open::OpenParams;
use rotate::RotateParams;
use save::SaveParams;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub enum ProcessingStage {
    Open(OpenParams),
    Rotate(RotateParams),
    Save(SaveParams),
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ProcessingPipeline {
    pub steps: Vec<ProcessingStage>,
}
