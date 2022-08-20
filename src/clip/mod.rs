//! Clip subcommand services.

use std::path::PathBuf;

pub fn ffmpeg_clip(
    infile: PathBuf,
    ss: String,
    t: String,
    to: Option<String>,
    output_file: PathBuf,
) -> String {
    let mut ffmpeg_command = format!("ffmpeg -ss {} -i {:?}", ss, infile);

    if let Some(to_var) = to {
        ffmpeg_command.push_str(&format!(" -to {:?}", to_var));
    } else {
        ffmpeg_command.push_str(&format!(" -t {}", t));
    };

    ffmpeg_command.push_str(&format!(" {:?}", output_file));

    ffmpeg_command
}
