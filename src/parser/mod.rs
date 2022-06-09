mod parser_attribute_parameters;

use super::parser::parser_attribute_parameters::*;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    /// Clip the video
    Clip {
        /// specifies the input file
        #[structopt(short, long, parse(from_os_str))]
        infile: PathBuf,

        /// set the start time offset
        ss: Option<String>,

        /// record or transcode "duration" seconds of audio/video
        t: String,

        /// record or transcode stop time
        to: String,
    },

    /// Virtual concatenation script demuxer.
    ConcatDemuxer {
        /// specifies the input list
        #[structopt(short, long)]
        infile: String,

        /// select an encoder or a decoder
        #[structopt(short, long)]
        codec: String,
    },

    /// Extract a video's frames
    Frames {
        /// specifies the input file
        #[structopt(short, long)]
        infile: String,

        /// video sync method
        #[structopt(short, long)]
        vsync: Option<String>,
    },
}

#[derive(StructOpt)]
#[structopt(about = attribute_parameter!("parser about"))]
pub struct Parser {
    #[structopt(subcommand)]
    cmd: Command,
}
