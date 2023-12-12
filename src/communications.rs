use reqwest::blocking::Client;
use serde::Serialize;

use crate::audio_query::{AccentPhraseModel, AudioQueryModel};
use crate::metas::Meta;

pub(crate) fn get_speakers(client: &Client, base_url: &str) -> reqwest::Result<Vec<Meta>> {
    client
        .get(base_url.to_string() + "/speakers")
        .send()?
        .json()
}

#[derive(Serialize)]
struct AudioQueryQuery {
    speaker: u32,
    text: String,
}

pub(crate) fn get_default_audio_query(
    client: &Client,
    speaker: u32,
    text: &str,
    base_url: &str,
) -> reqwest::Result<AudioQueryModel> {
    client
        .post(base_url.to_string() + "/audio_query")
        .query(&AudioQueryQuery {
            speaker,
            text: text.to_string(),
        })
        .send()?
        .json()
}

#[derive(Serialize)]
struct AccentPhrasesQuery {
    speaker: u32,
    text: String,
    is_kana: bool,
}

pub(crate) fn get_accent_phrases(
    client: &Client,
    speaker: u32,
    text: &str,
    is_kana: bool,
    base_url: &str,
) -> reqwest::Result<Vec<AccentPhraseModel>> {
    client
        .post(base_url.to_string() + "/accent_phrases")
        .query(&AccentPhrasesQuery {
            speaker,
            text: text.to_string(),
            is_kana,
        })
        .send()?
        .json()
}

#[derive(Serialize)]
struct SynthesisQuery {
    speaker: u32,
}

pub(crate) fn get_audio(
    client: &Client,
    speaker: u32,
    audio_query: &AudioQueryModel,
    base_url: &str,
) -> reqwest::Result<Vec<u8>> {
    client
        .post(base_url.to_string() + "/synthesis")
        .query(&SynthesisQuery { speaker })
        .json(audio_query)
        .send()?
        .bytes()
        .map(|bytes| bytes.into())
}
