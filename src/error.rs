use thiserror::Error;

#[derive(Debug, Error)]
pub enum VVSpeechError {
    #[error("invalid id: {0}")]
    InvalidId(u32),
    #[error("invalid speaker name: {0}")]
    InvalidSpeakerName(String),
    #[error("speaker '{0}' does not has style id: {1}")]
    InvalidSpeakerNameIdPair(String, u32),
    #[error("Could not play generated audio")]
    PlayAudioFailed,
    #[error("Could not detect the TTS engine (URL: {0})")]
    DetectEngineFailed(String),
    #[error("Could not get audio query")]
    GetAudioQueryFailed,
    #[error("Could not get audio")]
    GetAudioFailed,
    #[error("Could not get accent phrases")]
    GetAccentPhrasesFailed,
}

pub type Result<T> = ::std::result::Result<T, VVSpeechError>;
