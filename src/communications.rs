use serde::Serialize;

use crate::audio_query::{AccentPhraseModel, AudioQueryModel};
use crate::metas::Meta;

pub(crate) async fn get_speakers(base_url: &str) -> surf::Result<Vec<Meta>> {
    surf::get(base_url.to_string() + "/speakers")
        .recv_json()
        .await
}

#[derive(Serialize)]
struct AudioQueryQuery {
    speaker: u32,
    text: String,
}

pub(crate) async fn get_default_audio_query(
    speaker: u32,
    text: &str,
    base_url: &str,
) -> surf::Result<AudioQueryModel> {
    surf::post(base_url.to_string() + "/audio_query")
        .query(&AudioQueryQuery {
            speaker,
            text: text.to_string(),
        })?
        .recv_json()
        .await
}

#[derive(Serialize)]
struct AccentPhrasesQuery {
    speaker: u32,
    text: String,
    is_kana: bool,
}

pub(crate) async fn get_accent_phrases(
    speaker: u32,
    text: &str,
    is_kana: bool,
    base_url: &str,
) -> surf::Result<Vec<AccentPhraseModel>> {
    surf::post(base_url.to_string() + "/accent_phrases")
        .query(&AccentPhrasesQuery {
            speaker,
            text: text.to_string(),
            is_kana,
        })?
        .recv_json()
        .await
}

#[derive(Serialize)]
struct SynthesisQuery {
    speaker: u32,
}

pub(crate) async fn get_audio(
    speaker: u32,
    audio_query: &AudioQueryModel,
    base_url: &str,
) -> surf::Result<Vec<u8>> {
    surf::post(base_url.to_string() + "/synthesis")
        .query(&SynthesisQuery { speaker })?
        .content_type(surf::http::mime::JSON)
        .body_json(audio_query)?
        .recv_bytes()
        .await
}
