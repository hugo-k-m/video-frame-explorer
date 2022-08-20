//! Process the parser's inputs.

use super::subcommands::{Clip, Command, ConcatDemuxer, Convert, Frames, Parser};
use crate::clip::ffmpeg_clip;
use std::{env::current_dir, error::Error};

impl Parser {
    pub fn apply_parser_args(self) -> Result<String, Box<dyn Error>> {
        match self.cmd {
            Command::Clip(Clip {
                infile,
                ss,
                t,
                to,
                outfile,
            }) => {
                ffmpeg_clip(infile, ss, t, to, outfile);
            }
            Command::ConcatDemuxer(ConcatDemuxer {
                infile,
                codec,
                outfile,
            }) => {}
            Command::Convert(Convert {
                infile,
                qscale,
                outfile,
            }) => {}
            Command::Frames(Frames {
                infile,
                vsync,
                outfile,
            }) => {}
        }

        Ok("Invoked".to_owned())
    }
}
