use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct MoraModel {
    text: String,
    consonant: Option<String>,
    consonant_length: Option<f32>,
    vowel: String,
    vowel_length: f32,
    pitch: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AccentPhraseModel {
    moras: Vec<MoraModel>,
    accent: usize,
    pause_mora: Option<MoraModel>,
    is_interrogative: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub(crate) struct AudioQueryModel {
    pub accent_phrases: Vec<AccentPhraseModel>,
    #[serde(rename = "speedScale")]
    pub speed_scale: f32,
    #[serde(rename = "pitchScale")]
    pub pitch_scale: f32,
    #[serde(rename = "intonationScale")]
    pub intonation_scale: f32,
    #[serde(rename = "volumeScale")]
    pub volume_scale: f32,
    #[serde(rename = "prePhonemeLength")]
    pub pre_phoneme_length: f32,
    #[serde(rename = "postPhonemeLength")]
    pub post_phoneme_length: f32,
    #[serde(rename = "outputSamplingRate")]
    pub output_sampling_rate: u32,
    #[serde(rename = "outputStereo")]
    pub output_stereo: bool,
    pub kana: String,
}
