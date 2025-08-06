use super::*;
use serde::{Deserialize, Serialize};
use ts_rs::TS;
use uuid::Uuid;
// use uuid::Uuid;

#[derive(Default, Debug, Clone, Serialize, Deserialize, TS)]
pub struct OpenParams {
    pub file_path: String,
}

pub type OpenStage = StageData<OpenParams>;

impl OpenStage {
    pub fn add_image(&mut self, path: String) {
        let image_info = ImageInfo {
            id: Uuid::new_v4(),
            parent_id: None,
            params: OpenParams { file_path: path },
            mode: ImageMode::Auto,
        };
        let image_id = image_info.id;
        self.images.push(image_info);
        self.images_lookup.insert(image_id, self.images.len() - 1);
    }

    pub fn add_image_at(&mut self, path: String, index: usize) {
        let image_info = ImageInfo {
            id: Uuid::new_v4(),
            parent_id: None,
            params: OpenParams { file_path: path },
            mode: ImageMode::Auto,
        };
        let image_id = image_info.id;
        self.images.insert(index, image_info);
        self.images_lookup.insert(image_id, index);
    }
}
