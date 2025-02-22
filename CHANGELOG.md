# Changelog
## [Unreleased]

## [1.1.0] - 2025-02-22
### Added
- Added notification support when audio starts playing.
- Added `--notification` option to CLI for changing notification text.
- Added support for changing playlist directory to custom one.

### Changed
- The option `--loop-playlist` changed to `--loop` because it can be flagged in both `playlist` and `play` mode.
- Replaced some codes into a function for not repeating them.

### Fix
- Fixed an performance issue which causes to read playlist twice.
- Fixed some unnecessary `.clone()` calls and more.
- Fixed an bug where the playlist item will not remove unless the exact url given.

## [1.0.0] - 2025-02-17
### Added
- The whole project
