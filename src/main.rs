pub mod log;
pub mod mpv;
pub mod playlist;
pub mod utils;
pub mod youtube;

use crate::log::*;
use clap::{ArgGroup, Parser};
use std::collections::HashMap;

#[derive(Debug, Parser)]
#[clap(
    name = "playit",
    version,
    about,
    author = "kadircy",
    arg_required_else_help = true,
    group = ArgGroup::new("play_options").required(false).args(&["play", "playlist"])
)]
pub struct Cli {
    /// Play an audio with an url or a search query.
    #[clap(long, short = 'p')]
    play: Option<String>,

    /// Opens an mpv window instead of playing it in background.
    #[clap(long, short = 'v', default_value_t = false)]
    show_video: bool,

    /// Play video without audio. Opens an mpv window.
    #[clap(long, short = 'w', default_value_t = false)]
    only_video: bool,

    /// Select an playlist to play and modify.
    #[clap(long, short = 'l')]
    playlist: Option<String>,

    /// PLAYLIST ONLY: Add a new media to selected playlist.
    #[clap(long, short = 'a')]
    add: Option<String>,

    /// PLAYLIST ONLY: Remove a media from selected playlist with given query.
    #[clap(long, short = 'r')]
    remove: Option<String>,

    /// PLAYLIST ONLY: Play the selected playlist with mpv.
    #[clap(long, default_value_t = false)]
    play_playlist: bool,
}

fn main() {
    let args = Cli::parse();

    // If there is a playlist, we don't require --play argument
    let url: String = if let Some(ref playlist_name) = args.playlist {
        playlist_name.to_string()
    } else if let Some(play) = args.play {
        // Otherwise, use the --play argument
        if utils::is_url(&play) {
            info("Using provided URL directly.");
            play
        } else {
            match youtube::search(&play) {
                Ok(url) => url,
                Err(e) => {
                    error("Error fetching URL from YouTube.");
                    error(e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // If neither `--playlist` nor `--play` are provided, exit with an error
        error("Either --playlist or --play must be provided.");
        std::process::exit(1);
    };

    // Prepare mpv arguments
    let mut mpv_args: mpv::MpvArgs = HashMap::new();
    if !args.show_video && !args.only_video {
        mpv_args.insert("--no-video".to_string(), None);
    }
    if args.only_video {
        mpv_args.insert("--no-audio".to_string(), None);
    }

    // If a playlist is specified, handle the playlist logic
    if let Some(ref playlist_name) = args.playlist {
        let mut playlist = playlist::Playlist::new(&playlist_name);

        if std::fs::exists(&playlist.path).unwrap_or(false) {
            playlist.read().unwrap();
        }

        if args.add.is_some() {
            playlist.add(args.add.unwrap());
        }

        if args.remove.is_some() {
            playlist.remove(args.remove.unwrap());
        }

        match playlist.write() {
            Ok(_) => (),
            Err(e) => {
                error("Unable to write playlist.");
                error(e);
                std::process::exit(1);
            }
        };

        if args.play_playlist {
            // Read and play playlist
            match playlist.read() {
                Ok(_) => {
                    let first_audio = &playlist.items.get(0).unwrap();
                    for media in &playlist.items[1..] {
                        mpv_args.insert(media.to_string(), None);
                    }
                    let mpv = mpv::Mpv::new(first_audio.to_string(), Some(mpv_args.clone()));
                    info("Spawning mpv instance.");
                    let id = mpv.spawn();
                    info("Process id:");
                    println!("{}", id);
                }
                Err(e) => {
                    error("Error reading playlist.");
                    error(e);
                    std::process::exit(1);
                }
            }
        }
    } else {
        // Otherwise, handle the provided URL (either from --play or search)
        let mpv = mpv::Mpv::new(url, Some(mpv_args));
        info("Spawning mpv instance.");
        let id = mpv.spawn();
        info("Process id:");
        println!("{}", id);
    }
}
