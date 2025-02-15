pub mod log;
pub mod mpv;
pub mod utils;
pub mod youtube;

use crate::log::*;
use clap::Parser;
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[clap(
    name = "playit",
    version,
    about,
    author = "kadircy",
    arg_required_else_help = true
)]
pub struct Cli {
    /// Play an audio with an url or a search query.
    #[clap(long, short = 'p')]
    play: String,

    /// Don't use mpv with `--no-video` flag. Opens an mpv window instead of playing it in
    /// background.
    #[clap(long, short = 'v', default_value_t = false)]
    show_video: bool,
}

fn main() {
    let args = Cli::parse();

    // Check if input is a URL or a search query
    let url: String = if utils::is_url(&args.play) {
        info("Using provided URL directly.");
        args.play
    } else {
        match youtube::search(&args.play) {
            Ok(url) => url,
            Err(e) => {
                error("Error fetching URL from YouTube.");
                error(e);
                std::process::exit(1);
            }
        }
    };

    // Prepare mpv arguments
    let mut mpv_args: mpv::MpvArgs = HashMap::new();
    if !args.show_video {
        mpv_args.insert("--no-video".to_string(), "".to_string());
    }

    // Create and spawn MPV instance
    let mpv = mpv::Mpv::new(url, Some(mpv_args));
    info("Spawning mpv instance.");
    mpv.spawn();
}
