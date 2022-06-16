//! Process the parser's inputs.

use super::subcommands::{Clip, Command, ConcatDemuxer, Convert, Frames, Parser};
use std::error::Error;

impl Parser {
    pub fn apply_parser_args(self) -> Result<String, Box<dyn Error>> {
        match self.cmd {
            Command::Clip(Clip { infile, ss, t, to }) => {}
            Command::ConcatDemuxer(ConcatDemuxer { infile, codec }) => {}
            Command::Convert(Convert { infile, qscale }) => {}
            Command::Frames(Frames { infile, vsync }) => {}
        }

        Ok("Invoked".to_owned())
    }
}
