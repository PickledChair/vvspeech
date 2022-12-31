use clap::{Parser, Subcommand};

use std::fs::File;
use std::io::{BufReader, Cursor, Write};

use super::communications::*;
use super::error::*;
use super::metas::*;

const DEFAULT_BASE_URL: &str = "http://127.0.0.1:50021";

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Commands,
    /// Set the TTS engine URL (default = "http://127.0.0.1:50021".
    /// shorthands: ["voicevox", "coeiroink", "sharevox"])
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
    /// Speak the given text
    Play {
        /// input text
        text: String,
        /// speaker ID (default = 0)
        #[arg(short, long, value_name = "SPEAKER ID")]
        id: Option<u32>,
        /// speaker name
        #[arg(short, long, value_name = "SPEAKER NAME")]
        name: Option<String>,
    },
    /// Generate an audio file from the given text and save it
    Save {
        /// input text
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
    },
}

async fn get_audio_from_engine(
    metas: &[Meta],
    speaker_id: &Option<u32>,
    speaker_name: &Option<String>,
    text: &str,
    base_url: &str,
) -> Result<Vec<u8>> {
    let speaker_id = get_appropriate_id(metas, speaker_id, speaker_name)?;
    let audio_query = get_default_audio_query(speaker_id, text, base_url)
        .await
        .map_err(|_| VVSpeechError::GetAudioQueryFailed)?;

    get_audio(speaker_id, &audio_query, base_url)
        .await
        .map_err(|_| VVSpeechError::GetAudioFailed)
}

fn play_audio(audio_data: Vec<u8>) -> Result<()> {
    let audio_buf = Cursor::new(audio_data);

    let (_stream, handle) =
        rodio::OutputStream::try_default().map_err(|_| VVSpeechError::PlayAudioFailed)?;
    let sink = rodio::Sink::try_new(&handle).map_err(|_| VVSpeechError::PlayAudioFailed)?;
    sink.append(
        rodio::Decoder::new(BufReader::new(audio_buf))
            .map_err(|_| VVSpeechError::PlayAudioFailed)?,
    );

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

pub(crate) async fn app_run() -> anyhow::Result<()> {
    let args = Args::parse();
    let base_url = args
        .engine_url
        .unwrap_or_else(|| DEFAULT_BASE_URL.to_string());
    let base_url = if ["voicevox", "coeiroink", "sharevox"].contains(&base_url.as_str()) {
        "http://127.0.0.1:".to_string()
            + match base_url.as_str() {
                "voicevox" => "50021",
                "coeiroink" => "50031",
                "sharevox" => "50025",
                _ => unreachable!(),
            }
    } else {
        base_url
    };

    let speakers = get_speakers(&base_url)
        .await
        .map_err(|_| VVSpeechError::DetectEngineFailed(base_url.clone()))?;

    match &args.command {
        Commands::Info {
            name,
            json,
            pretty_json,
        } => {
            show_info(&speakers, name, *json, *pretty_json)?;
        }
        Commands::Play {
            text,
            id: speaker_id,
            name: speaker_name,
        } => {
            let audio =
                get_audio_from_engine(&speakers, speaker_id, speaker_name, text, &base_url).await?;
            play_audio(audio)?;
        }
        Commands::Save {
            text,
            id: speaker_id,
            name: speaker_name,
            output,
        } => {
            let audio =
                get_audio_from_engine(&speakers, speaker_id, speaker_name, text, &base_url).await?;
            let mut file = File::create(output)?;
            file.write_all(&audio)?;
        }
    };

    Ok(())
}
