mod frames;
mod parser_args;
mod parser_attribute_parameters;

use super::parser::parser_attribute_parameters::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt)]
struct Clip {
    /// Specifies the input file.
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,

    /// Set the start time offset.
    ss: Option<String>,

    /// Record or transcode "duration" seconds of audio/video.
    t: String,

    /// Record or transcode stop time.
    to: String,
}

#[derive(StructOpt)]
struct ConcatDemuxer {
    /// Specifies the input list.
    #[structopt(short, long)]
    infile: String,

    /// Select an encoder or a decoder.
    #[structopt(short, long)]
    codec: String,
}

#[derive(StructOpt)]
struct Convert {
    /// Specifies the input file.
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,

    /// Use fixed quality scale (VBR).
    #[structopt(short, long)]
    qscale: String,
}

#[derive(StructOpt)]
struct Frames {
    /// Specifies the input file.
    #[structopt(short, long, parse(from_os_str))]
    infile: PathBuf,

    /// Video sync method.
    #[structopt(short, long)]
    vsync: Option<String>,
}

#[derive(StructOpt)]
enum Command {
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
    cmd: Command,
}
