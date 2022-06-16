//! Parser subcommands

use super::parser_attribute_parameters::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
pub struct Clip {
    /// Specifies the input file.
    #[structopt(short, long, parse(from_os_str))]
    pub infile: PathBuf,

    /// Set the start time offset.
    pub ss: Option<String>,

    /// Record or transcode "duration" seconds of audio/video.
    pub t: String,

    /// Record or transcode stop time.
    pub to: String,

    /// Specifies the output file.
    pub outfile: PathBuf,
}

#[derive(StructOpt)]
pub struct ConcatDemuxer {
    /// Specifies the input list.
    #[structopt(short, long)]
    pub infile: String,

    /// Select an encoder or a decoder.
    #[structopt(short, long)]
    pub codec: String,

    /// Specifies the output file.
    pub outfile: PathBuf,
}

#[derive(StructOpt)]
pub struct Convert {
    /// Specifies the input file.
    #[structopt(short, long, parse(from_os_str))]
    pub infile: PathBuf,

    /// Use fixed quality scale (VBR).
    #[structopt(short, long)]
    pub qscale: String,

    /// Specifies the output file.
    pub outfile: PathBuf,
}

#[derive(StructOpt)]
pub struct Frames {
    /// Specifies the input file.
    #[structopt(short, long, parse(from_os_str))]
    pub infile: PathBuf,

    /// Video sync method.
    #[structopt(short, long)]
    pub vsync: Option<String>,

    /// Specifies the output file.
    pub outfile: PathBuf,
}

#[derive(StructOpt)]
pub enum Command {
    /// Clip the video file.
    Clip(Clip),

    /// Virtual concatenation script demuxer.
    ConcatDemuxer(ConcatDemuxer),

    /// Convert video formats.
    Convert(Convert),

    /// Extract a video's frames
    Frames(Frames),
}

#[derive(StructOpt)]
#[structopt(about = attribute_parameter!("parser about"))]
pub struct Parser {
    #[structopt(subcommand)]
    pub cmd: Command,
}
