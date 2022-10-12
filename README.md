# polly-tts-rs

A very simple TTS application using [AWS Polly](https://aws.amazon.com/polly/) service

## Build

    cargo build

## Usage

``` plain
A very simple TTS application using AWS Polly service

Usage: polly-tts-rs [OPTIONS] --voice <VOICE> <TEXT>

Arguments:
  <TEXT>  The text to read

Options:
  -v, --voice <VOICE>            The voice ID to use to read the text
  -r, --rate <RATE>              The reading speed rate [default: medium] [possible values: xslow, slow, medium, fast, xfast]
  -o, --output <OUTPUT>          Path to the output mp3 file [default: output.mp3]
      --aws-region <AWS_REGION>  AWS Region. If not specified, the AWS_REGION env var is used. If the env var is not defined, it fallbacks to 'eu-west-1'
  -h, --help                     Print help information
  -V, --version                  Print version information
  ```

### Authentication and credentials

Using Amazon Polly requires authentication (for billing). The credentials are read from environment variables `AWS_ACCESS_KEY` and `AWS_SECRET_ACCESS_KEY`.

## Usage

    polly-tts-rs -v Mizuki -r slow -o message.mp3 -- お客様は神様です
