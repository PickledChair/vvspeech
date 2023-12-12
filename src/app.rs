use clap::{Parser, Subcommand};
use reqwest::blocking::Client;

use std::fs::File;
use std::io::{Cursor, Write};

use crate::communications::*;
use crate::error::*;
use crate::metas::*;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:50021";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    /// Set the TTS engine URL (default = "http://127.0.0.1:50021".
    /// shorthands: ["voicevox", "coeiroink", "sharevox", "lmroid", "itvoice"])
    #[arg(short, long, value_name = "URL")]
    engine_url: Option<String>,
}

#[derive(Subcommand)]
enum Commands {
    /// Show the speakers information
    Info {
        /// speaker name
        #[arg(short, long, value_name = "SPEAKER NAME")]
        name: Option<String>,
        /// JSON output
        #[arg(short, long)]
        json: bool,
        /// pretty JSON output
        #[arg(short, long)]
        pretty_json: bool,
    },
    /// Convert the given text to AquesTalk-like notation
    Kana {
        /// input text
        text: String,
    },
    /// Speak the given text
    Play {
        /// input text (if '--kana' option is specified, AquesTalk-like notation required)
        text: String,
        /// speaker ID (default = 0)
        #[arg(short, long, value_name = "SPEAKER ID")]
        id: Option<u32>,
        /// speaker name
        #[arg(short, long, value_name = "SPEAKER NAME")]
        name: Option<String>,
        /// AquesTalk-like notation flag
        #[arg(short, long)]
        kana: bool,
        /// speed of speech
        #[arg(long, default_value_t = 1.0)]
        speed: f32,
        /// pitch of speech
        #[arg(long, default_value_t = 0.0)]
        pitch: f32,
        /// intonation of speech
        #[arg(long, default_value_t = 1.0)]
        intonation: f32,
        /// volume of speech
        #[arg(long, default_value_t = 1.0)]
        volume: f32,
        /// pre phoneme length
        #[arg(long, default_value_t = 0.1)]
        pre_phoneme: f32,
        /// post phoneme length
        #[arg(long, default_value_t = 0.1)]
        post_phoneme: f32,
    },
    /// Generate an audio file from the given text and save it
    Save {
        /// input text (if '--kana' option is specified, AquesTalk-like notation required)
        text: String,
        /// speaker ID (default = 0)
        #[arg(short, long, value_name = "SPEAKER ID")]
        id: Option<u32>,
        /// speaker name
        #[arg(short, long, value_name = "SPEAKER NAME")]
        name: Option<String>,
        /// output file name
        #[arg(short, long, value_name = "OUTPUT FILE")]
        output: String,
        /// AquesTalk-like notation flag
        #[arg(short, long)]
        kana: bool,
        /// speed of speech
        #[arg(long, default_value_t = 1.0)]
        speed: f32,
        /// pitch of speech
        #[arg(long, default_value_t = 0.0)]
        pitch: f32,
        /// intonation of speech
        #[arg(long, default_value_t = 1.0)]
        intonation: f32,
        /// volume of speech
        #[arg(long, default_value_t = 1.0)]
        volume: f32,
        /// pre phoneme length
        #[arg(long, default_value_t = 0.1)]
        pre_phoneme: f32,
        /// post phoneme length
        #[arg(long, default_value_t = 0.1)]
        post_phoneme: f32,
    },
}

#[allow(clippy::too_many_arguments)]
fn get_audio_from_engine(
    client: &Client,
    metas: &[Meta],
    speaker_id: &Option<u32>,
    speaker_name: &Option<String>,
    text: &str,
    speed: f32,
    pitch: f32,
    intonation: f32,
    volume: f32,
    pre_phoneme: f32,
    post_phoneme: f32,
    is_kana: bool,
    base_url: &str,
) -> Result<Vec<u8>> {
    let speaker_id = get_appropriate_id(metas, speaker_id, speaker_name)?;
    let mut audio_query = get_default_audio_query(
        client,
        speaker_id,
        if is_kana { "" } else { text },
        base_url,
    )
    .map_err(|_| VVSpeechError::GetAudioQueryFailed)?;
    if is_kana {
        audio_query.accent_phrases =
            get_accent_phrases(client, speaker_id, text, is_kana, base_url)
                .map_err(|_| VVSpeechError::GetAccentPhrasesFailed)?;
    }
    audio_query.speed_scale = speed;
    audio_query.pitch_scale = pitch;
    audio_query.intonation_scale = intonation;
    audio_query.volume_scale = volume;
    audio_query.pre_phoneme_length = pre_phoneme;
    audio_query.post_phoneme_length = post_phoneme;

    get_audio(client, speaker_id, &audio_query, base_url).map_err(|_| VVSpeechError::GetAudioFailed)
}

fn play_audio(audio_data: Vec<u8>) -> Result<()> {
    let audio_buf = Cursor::new(audio_data);

    let (_stream, handle) =
        rodio::OutputStream::try_default().map_err(|_| VVSpeechError::PlayAudioFailed)?;
    let sink = rodio::Sink::try_new(&handle).map_err(|_| VVSpeechError::PlayAudioFailed)?;
    sink.append(rodio::Decoder::new(audio_buf).map_err(|_| VVSpeechError::PlayAudioFailed)?);

    sink.sleep_until_end();
    Ok(())
}

fn show_non_json_speaker_info(meta: &Meta) {
    println!(
        "{}\n\tstyles: {}\n\tspeaker_uuid: {}\n\tversion: {}",
        meta.name.as_str(),
        meta.styles
            .iter()
            .map(|style| format!("{} (id: {})", style.name.as_str(), style.id))
            .collect::<Vec<_>>()
            .join(", ")
            .as_str(),
        meta.speaker_uuid.as_str(),
        meta.version.as_str(),
    );
}

fn show_info(
    metas: &[Meta],
    name: &Option<String>,
    json: bool,
    pretty_json: bool,
) -> anyhow::Result<()> {
    #[allow(clippy::collapsible_else_if)]
    if let Some(name) = name {
        let meta = metas
            .iter()
            .find(|meta| &meta.name == name)
            .ok_or_else(|| VVSpeechError::InvalidSpeakerName(name.clone()))?;
        if json && !pretty_json {
            println!("{}", serde_json::to_string(meta)?);
        } else if pretty_json {
            println!("{}", serde_json::to_string_pretty(meta)?);
        } else {
            show_non_json_speaker_info(meta);
        }
    } else {
        if json && !pretty_json {
            println!("{}", serde_json::to_string(metas)?);
        } else if pretty_json {
            println!("{}", serde_json::to_string_pretty(metas)?);
        } else {
            for meta in metas {
                show_non_json_speaker_info(meta);
            }
        }
    }
    Ok(())
}

fn get_kana(client: &Client, text: &str, base_url: &str) -> Result<String> {
    let audio_query = get_default_audio_query(client, 0, text, base_url)
        .map_err(|_| VVSpeechError::GetAudioQueryFailed)?;

    Ok(audio_query.kana)
}

pub(crate) fn app_run() -> anyhow::Result<()> {
    let args = Args::parse();
    let base_url = if let Some(base_url) = args.engine_url {
        match base_url.to_lowercase().as_str() {
            "voicevox" => "http://127.0.0.1:50021".to_string(),
            "coeiroink" => "http://127.0.0.1:50031".to_string(),
            "sharevox" => "http://127.0.0.1:50025".to_string(),
            "lmroid" => "http://127.0.0.1:50073".to_string(),
            "itvoice" => "http://127.0.0.1:49540".to_string(),
            _ => base_url,
        }
    } else {
        DEFAULT_BASE_URL.to_string()
    };

    let client = Client::new();

    let speakers = get_speakers(&client, &base_url)
        .map_err(|_| VVSpeechError::DetectEngineFailed(base_url.clone()))?;

    match &args.command {
        Commands::Info {
            name,
            json,
            pretty_json,
        } => {
            show_info(&speakers, name, *json, *pretty_json)?;
        }
        Commands::Kana { text } => {
            println!("\"{}\"", get_kana(&client, text, &base_url)?);
        }
        Commands::Play {
            text,
            id: speaker_id,
            name: speaker_name,
            kana,
            speed,
            pitch,
            intonation,
            volume,
            pre_phoneme,
            post_phoneme,
        } => {
            let audio = get_audio_from_engine(
                &client,
                &speakers,
                speaker_id,
                speaker_name,
                text,
                *speed,
                *pitch,
                *intonation,
                *volume,
                *pre_phoneme,
                *post_phoneme,
                *kana,
                &base_url,
            )?;
            play_audio(audio)?;
        }
        Commands::Save {
            text,
            id: speaker_id,
            name: speaker_name,
            output,
            kana,
            speed,
            pitch,
            intonation,
            volume,
            pre_phoneme,
            post_phoneme,
        } => {
            let audio = get_audio_from_engine(
                &client,
                &speakers,
                speaker_id,
                speaker_name,
                text,
                *speed,
                *pitch,
                *intonation,
                *volume,
                *pre_phoneme,
                *post_phoneme,
                *kana,
                &base_url,
            )?;
            let mut file = File::create(output)?;
            file.write_all(&audio)?;
        }
    };

    Ok(())
}
