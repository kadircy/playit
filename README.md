# `playit`

An lightweight and simple AF media player which uses [youtube](https://youtube.com) for sources and plays with [mpv](https://mpv.io).

The goal of this project is playing audio without worry about anything.

Also you can integrate `playit` with almost everything like [dmenu](#integrate#dmenu) / [rofi](#integrate#rofi) / [tofi](#integrate#tofi) and their alternatives.

![Demo Gif](./assets/demo.gif)

## Installation

**Note:** playit requires [mpv](https://mpv.io) and [youtube-dlp](https://github.com/yt-dlp/yt-dlp) for runtime dependencies.

### From Cargo
```bash
cargo install playit
```

### From Script
```bash
curl -sSfL https://raw.githubusercontent.com/kadircy/playit/master/install.sh | sh
```

### From repository

1. **Make sure you have `rust`, `cargo` and `git` installed on your system.**
```bash
cargo --version
rustc --version
git --version
```

2. **Clone the Git repository.**
The compile process takes some time because of using LTO and striping when building release binaries.

```bash
git clone https://github.com/kadircy/playit
```

3. **Compile**
```
cargo build --release
```

Now you can run binary with
```bash
./target/release/playit
```

## Usage
`playit` is like other CLI programs. It accept options and print results.
When you run `playit` without arguments, it will print an small help menu with flags and options.

`playit --play <QUERY>` will play the given query **in background** and will print some information about `mpv` like process id.

`playit --playlist <NAME> --add <QUERY>` will add given query to the playlist with **<NAME>** but will not play it.

`playit --playlist <NAME> --play-playlist` will play the playlist.

You can see other options in details with: `playit --help`

## Integrate
To use `playit` with another programs, you can add keybindings for getting input and using this input to play media.
I prefer using **launchers** for getting user query or selecting playlist.

We will use [Hyprland](https://hyprland.org/) as an example WM for configurations.

### dmenu
You can integrate playit in `dmenu` with something like this:

```config
# Bind a key to launch playit with dmenu input
bind = $mainMod, P, exec, playit --play "$(dmenu < /dev/null)"
```

Also, you can integrate playlists with something like this:

```config
# Bind a key to select and play a playlist using dmenu
bind = $mainMod SHIFT, P, exec, QUERY=$(ls $HOME/.config/playit | sed 's/\..*$//' | dmenu) && playit --playlist "$QUERY" --play-playlist
```

You can add more features with similar commands. Use `playit --help` for all options.

### rofi
If you prefer `rofi` over `dmenu`, you can achieve similar functionality with this configuration:

```config
# Bind a key to launch playit with rofi input
bind = $mainMod, P, exec, playit --play "$(rofi -dmenu < /dev/null)"
```

And for playlists:

```config
# Bind a key to select and play a playlist using rofi
bind = $mainMod SHIFT, P, exec, QUERY=$(ls $HOME/.config/playit | sed 's/\..*$//' | rofi -dmenu) && playit --playlist "$QUERY" --play-playlist
```

### tofi
For tofi, the configuration is quite similar. Here’s how you can use it to play media:

```config
# Bind a key to launch playit with tofi input
bind = $mainMod, P, exec, playit --play "$(tofi --require-match=false < /dev/null)"
```

```config
# Bind a key to select and play a playlist using tofi
bind = $mainMod, P, exec, QUERY=$(ls $HOME/.config/playit | sed 's/\..*$//' | tofi) && playit --playlist "$QUERY" --play-playlist
```

## Contributing

Contributions are welcome! If you’d like to contribute, please follow these steps:

1. Fork the repository.
2. Create a new branch `git checkout -b feature-branch`.
3. Make your changes and commit them `git commit -am 'Add new feature'`.
4. Push to your fork `git push origin feature-branch`.
5. Open a pull request to the main repository.

Please make sure your code follows the style and guidelines of the project. You can format codebase and lint it with this commands:

```bash
cargo fmt    # format
cargo clippy # lint
```

## License
This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details.

## Acknowledgements
- [mpv](https://mpv.io) for media playback.
- [youtube-dlp](https://https://github.com/yt-dlp/yt-dlp) for handling YouTube searchs and more.
- The developers of [dmenu](https://tools.suckless.org/dmenu/), [rofi](https://github.com/davatorium/rofi), and [tofi](https://github.com/philj56/tofi) for their awesome launcher programs.
