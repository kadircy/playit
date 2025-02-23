pub mod cache;
pub mod log;
pub mod mpv;
pub mod notification;
pub mod playlist;
pub mod utils;
pub mod youtube;
use crate::log::*;
use crate::notification::send_notification;
use crate::playlist::Playlist;
use clap::{ArgGroup, Parser};
use rand::seq::SliceRandom;
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
    /// The URL or search query to play.
    ///
    /// If a valid URL is provided, it will play the media from that URL.
    /// If a search query is provided, it will attempt to find the media using YouTube search.
    #[clap(long, short = 'p')]
    play: Option<String>,

    /// Flag to specify whether the video should be played in an MPV window.
    ///
    /// Set to `true` (pass this option to arguments) to open the video in a window.
    /// Set to `false` (default) to play in the background without opening a window.
    #[clap(long, short = 'w', default_value_t = false)]
    show_video: bool,

    /// Flag to play only the video (without audio).
    ///
    /// Set to `true` (pass this option to arguments) to play only the video, and the audio will be muted.
    /// Set to `false` (default) to play both audio and video.
    #[clap(long, default_value_t = false)]
    only_video: bool,

    /// The name of a playlist to play or modify.
    ///
    /// If specified, it will attempt to load the playlist and play it.
    /// Playlists can be used to manage multiple media files.
    #[clap(long, short = 'l')]
    playlist: Option<String>,

    /// (PLAYLIST ONLY) Change the playlist directory to custom one.
    ///
    /// This option will change the playlist path from `$XDG_CONFIG_HOME/playit/<playlist>` to
    /// `<prefix>/<playlist>`.
    /// Also note that, all the playlist files should end with '.pl' extension and should be in
    /// JSON format.
    #[clap(long)]
    prefix: Option<String>,

    /// (PLAYLIST ONLY) Add a new media item to the selected playlist.
    ///
    /// This option accepts a query (e.g., a song name or URL) to add a new media item to the playlist.
    #[clap(long, short = 'a')]
    add: Option<String>,

    /// (PLAYLIST ONLY) Remove a media item from the selected playlist based on a query.
    ///
    /// This option accepts a query to find and remove a specific media item from the playlist.
    #[clap(long, short = 'r')]
    remove: Option<String>,

    /// (PLAYLIST ONLY) Play the selected playlist using MPV.
    ///
    /// This option will play all the media in the specified playlist using MPV.
    #[clap(long, default_value_t = false)]
    play_playlist: bool,

    /// (PLAYLIST ONLY) Shuffle the playlist items.
    ///
    /// This option will randomize the order of the media items in the playlist.
    #[clap(long, short = 's', default_value_t = false)]
    shuffle: bool,

    /// Set the volume for MPV playback.
    ///
    /// This option accepts an integer value to set the volume level (0-100). Default is 100.
    #[clap(long, short = 'v', default_value_t = 100)]
    volume: u8,

    /// Loop the audio or playlist when it finishes.
    ///
    /// This option will repeat the audio once it is finished.
    #[clap(long = "loop", default_value_t = false)]
    loop_audio: bool,

    /// Mute the audio during playback.
    ///
    /// This option will mute the audio while the media is playing.
    #[clap(long, short = 'm', default_value_t = false)]
    mute: bool,

    /// Send a notification when audio starts playing.
    ///
    /// This option will send a notification when the media started playing.
    /// You can pass "" to not use notifications.
    /// The '{}' will be replaced with media url.
    #[clap(long, short = 'n', default_value_t = String::from("Now playing: {}"))]
    notification: String,
}

fn main() {
    let args = Cli::parse();

    // Determine the URL to be played based on the provided arguments
    let url: String = if let Some(ref playlist_name) = args.playlist {
        playlist_name.to_string()
    } else if let Some(play) = args.play {
        if utils::is_url(&play) {
            info("Using provided URL directly.");
            play
        } else {
            // Create an cache object
            let mut cache = cache::Cache::new();
            if let Err(e) = cache.read() {
                warning("An error occured while reading cache file.");
                error(&e);
                warning("The caching will be not used.");
                // Get the search result of url to use it without caching.
                match youtube::search(&play) {
                    Ok(search_url) => {
                          search_url
                    }
                    Err(e) => {
                        error("Error fetching URL from YouTube.");
                        error(&e);
                        std::process::exit(1);
                    }
                }
            } else if let Some(cached_url) = cache.items.get(&play) {
                // If the query is already cached, use it.
                info("Using cached URL.");
                cached_url.to_string()
            } else {
                // If the query is not cached, get search result of query and save it to the cache
                // And use the result of search
                match youtube::search(&play) {
                    Ok(search_url) => {
                        cache.add(&play, &search_url);
                        cache.write().unwrap();
                        search_url
                    }
                    Err(e) => {
                        error("Error fetching URL from YouTube.");
                        error(&e);
                        std::process::exit(1);
                    }
                }
            }
        }
    } else {
        error("Either --playlist or --play must be provided.");
        std::process::exit(1);
    };

    // Prepare MPV arguments based on user preferences
    let mut mpv_args: mpv::MpvArgs = HashMap::new();

    // Handle the video options
    if !args.show_video && !args.only_video {
        mpv_args.insert("--no-video".to_string(), None); // Play without video
    }
    if args.only_video {
        mpv_args.insert("--no-audio".to_string(), None); // Play only video
    }

    // Set volume
    mpv_args.insert("--volume".to_string(), Some(args.volume.to_string()));

    // Mute the audio if specified
    if args.mute {
        mpv_args.insert("--mute".to_string(), None); // Mute the audio
    }

    // Loop the audio if specified
    if args.loop_audio {
        mpv_args.insert("--loop".to_string(), None); // Loop the audio
    }

    // Handle playlist-related logic if specified
    if let Some(ref playlist_name) = args.playlist {
        let mut playlist: Playlist = playlist::Playlist::new(playlist_name, args.prefix.as_deref());

        if std::fs::exists(&playlist.path).unwrap_or(false) {
            // Try reading the playlist if it exists
            if let Err(e) = playlist.read() {
                error("Error reading playlist.");
                error(&e); // Log detailed error message
                std::process::exit(1);
            }
        }

        // Add media to the playlist if the `--add` option is specified
        if let Some(ref add_query) = args.add {
            playlist.add(add_query);
        }

        // Remove media from the playlist if the `--remove` option is specified
        if let Some(ref remove_query) = args.remove {
            playlist.remove(remove_query);
        }

        // Shuffle playlist if the `--shuffle` option is specified
        if args.shuffle {
            playlist.items.shuffle(&mut rand::rng()); // Shuffle playlist
            info("Playlist items shuffled.");
        }

        if args.add.is_some() || args.remove.is_some() || args.shuffle {
            // Write the updated playlist back to disk
            playlist = playlist
                .write()
                .expect("error: Unable to write changes to playlist file.");
        }

        // Play the playlist if the `--play_playlist` option is specified
        if args.play_playlist {
            // If the playlist is empty, give an error message
            if playlist.items.is_empty() {
                error("The playlist is empty. Add some querys with `--add` flag.");
                std::process::exit(1);
            }

            let first_audio = playlist
                .items
                .first()
                .expect("Playlist should not be empty.");
            for media in &playlist.items[1..] {
                mpv_args.insert(media.to_string(), None);
            }
            start_instance(first_audio, mpv_args, &args.notification);
        }
    } else {
        // Play a single media URL (either from --play or search)
        start_instance(&url, mpv_args, &args.notification);
    }
}

fn start_instance(url: &str, mpv_args: mpv::MpvArgs, notification: &str) {
    let mpv = mpv::Mpv::new(url.to_string(), Some(mpv_args));
    info("Spawning mpv instance.");
    let id = mpv.spawn();
    if !notification.is_empty() {
        send_notification(&notification.replace("{}", url));
    }
    info("Process id:");
    println!("  {}", id);
}
