use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Style {
    pub name: String,
    pub id: u32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct Meta {
    pub name: String,
    pub styles: Vec<Style>,
    pub speaker_uuid: String,
    pub version: String,
}

impl Meta {
    pub fn has_id(&self, speaker_id: u32) -> bool {
        self.styles.iter().any(|style| style.id == speaker_id)
    }

    pub fn first_style_id(&self) -> u32 {
        self.styles[0].id
    }
}
