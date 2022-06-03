use lazy_static::lazy_static;
use std::collections::HashMap;
use structopt::StructOpt;

lazy_static! {
    static ref PARSER_OPTIONS: HashMap<&'static str, &'static str> = {
        let mut hash_map = HashMap::new();

        hash_map.insert(
            "parser about",
            "Media tool for clipping videos, extracting\
            frames, and other features.",
        );

        hash_map
    };
}

macro_rules! attribute_parameter {
    ($option:expr) => {
        PARSER_OPTIONS.get($option).unwrap().to_owned()
    };
}

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
