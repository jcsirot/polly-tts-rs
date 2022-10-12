#![allow(unused)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_polly::model::{OutputFormat, TextType, VoiceId};
use aws_sdk_polly::{Client, Error, Region, PKG_VERSION};
use clap::{Parser, ValueEnum};
use std::fmt;
use std::fs;
use std::path::PathBuf;
use tokio::io::AsyncWriteExt;

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, ValueEnum)]
enum Rate {
    Xslow,
    Slow,
    Medium,
    Fast,
    Xfast,
}

impl fmt::Display for Rate {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Rate::Xslow => write!(f, "x-slow"),
            Rate::Slow => write!(f, "slow"),
            Rate::Medium => write!(f, "medium"),
            Rate::Fast => write!(f, "fast"),
            Rate::Xfast => write!(f, "x-fast"),
        }
    }
}

/// A very simple TTS application using AWS Polly service.
#[derive(Parser)]
#[command(author, version, about, long_about = None)]

struct Cli {
    /// The text to read
    text: String,
    /// The voice ID to use to read the text
    #[arg(short, long)]
    voice: VoiceId,
    /// The reading speed rate
    #[arg(short, long, value_enum, default_value_t = Rate::Medium)]
    rate: Rate,
    /// Path to the output mp3 file
    #[arg(short, long, default_value_t = String::from("output.mp3"))]
    output: String,

    // AWS Access Key ID
    // #[arg(long)]
    // access_key_id: Option<String>,
    // AWS Secret Access Key
    // #[arg(long)]
    // secret_access_key: Option<String>,
    /// AWS Region. If not specified, the AWS_REGION env var is used. If the env var is not defined, it fallbacks to 'eu-west-1'
    #[arg(long)]
    aws_region: Option<String>,
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let args = Cli::parse();

    let region_provider = RegionProviderChain::first_try(args.aws_region.map(Region::new))
        .or_default_provider()
        .or_else(Region::new("eu-west-1"));

    let shared_config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&shared_config);

    let result = synthesize(&client, &args.text, &args.voice, &args.rate, &args.output).await;

    result
}

// Create speech from text.
async fn synthesize(
    client: &Client,
    text: &str,
    voice: &VoiceId,
    rate: &Rate,
    out_file: &str,
) -> Result<(), Error> {
    let content = format!("<speak><prosody rate='{}'>{}</prosody></speak>", rate, text);

    let resp = client
        .synthesize_speech()
        .output_format(OutputFormat::Mp3)
        .text(content)
        .text_type(TextType::Ssml)
        .voice_id(voice.clone())
        .send()
        .await?;

    // Get MP3 data from response and save it
    let mut blob = resp
        .audio_stream
        .collect()
        .await
        .expect("failed to read data");

    let mut file = tokio::fs::File::create(out_file)
        .await
        .expect("failed to create file");

    file.write_all_buf(&mut blob)
        .await
        .expect("failed to write to file");

    Ok(())
}
