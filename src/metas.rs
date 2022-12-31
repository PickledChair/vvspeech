use serde::{Deserialize, Serialize};

use crate::error::*;

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

fn is_valid_id(metas: &[Meta], speaker_id: u32) -> bool {
    metas.iter().any(|meta| meta.has_id(speaker_id))
}

fn get_first_speaker_id(metas: &[Meta], speaker_name: &str) -> Result<u32> {
    Ok(metas
        .iter()
        .find(|meta| meta.name == speaker_name)
        .ok_or(VVSpeechError::InvalidSpeakerName(speaker_name.to_string()))?
        .first_style_id())
}

pub(crate) fn get_appropriate_id(
    metas: &[Meta],
    speaker_id: &Option<u32>,
    speaker_name: &Option<String>,
) -> Result<u32> {
    match (speaker_id, speaker_name) {
        (None, None) => Ok(0),
        (Some(speaker_id), None) => {
            if is_valid_id(metas, *speaker_id) {
                Ok(*speaker_id)
            } else {
                Err(VVSpeechError::InvalidId(*speaker_id))
            }
        }
        (None, Some(speaker_name)) => get_first_speaker_id(metas, speaker_name),
        (Some(speaker_id), Some(speaker_name)) => {
            let speaker = metas
                .iter()
                .find(|meta| &meta.name == speaker_name)
                .ok_or_else(|| VVSpeechError::InvalidSpeakerName(speaker_name.to_string()))?;
            if !speaker.has_id(*speaker_id) {
                return Err(VVSpeechError::InvalidSpeakerNameIdPair(
                    speaker.name.to_string(),
                    *speaker_id,
                ));
            }
            Ok(*speaker_id)
        }
    }
}
