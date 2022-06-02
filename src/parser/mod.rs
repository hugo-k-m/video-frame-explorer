use structopt::StructOpt;

#[derive(Debug, StructOpt)]
pub enum Parser {
    /// Clip the video
    Clip {
        /// Specifies the input file
        infile: String,
    },

    /// Extract a video's frames
    Frames {
        /// Specifies the input file
        infile: String,
    },
}
