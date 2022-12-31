use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct MoraModel {
    text: String,
    consonant: Option<String>,
    consonant_length: Option<f32>,
    vowel: String,
    vowel_length: f32,
    pitch: f32,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AccentPhraseModel {
    moras: Vec<MoraModel>,
    accent: usize,
    pause_mora: Option<MoraModel>,
    is_interrogative: bool,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct AudioQueryModel {
    accent_phrases: Vec<AccentPhraseModel>,
    #[serde(rename = "speedScale")]
    speed_scale: f32,
    #[serde(rename = "pitchScale")]
    pitch_scale: f32,
    #[serde(rename = "intonationScale")]
    intonation_scale: f32,
    #[serde(rename = "volumeScale")]
    volume_scale: f32,
    #[serde(rename = "prePhonemeLength")]
    pre_phoneme_length: f32,
    #[serde(rename = "postPhonemeLength")]
    post_phoneme_length: f32,
    #[serde(rename = "outputSamplingRate")]
    output_sampling_rate: u32,
    #[serde(rename = "outputStereo")]
    output_stereo: bool,
    kana: String,
}
