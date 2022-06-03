mod parser_attribute_parameters;

use super::parser::parser_attribute_parameters::{attribute_parameter, PARSER_OPTIONS};
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(about = attribute_parameter!("parser about"))]
pub enum Parser {
    /// Clip the video
    Clip {
        /// Specifies the input file
        #[structopt(short, long)]
        infile: String,
    },

    /// Extract a video's frames
    Frames {
        /// Specifies the input file
        #[structopt(short, long)]
        infile: String,
    },
}
