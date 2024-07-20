# TrackersGUI

[![madewith](https://forthebadge.com/images/badges/made-with-rust.svg)](https://forthebadge.com)

A GUI to get public trackers list from [ngosang/trackerslist](https://github.com/ngosang/trackerslist). Port of [PublicTrackersGUI](https://github.com/DarkCeptor44/PublicTrackersGUI) to Rust.

![screen1](assets/screen1.png)

## Installation

You can either get a binary from [Releases](https://github.com/DarkCeptor44/trackers-gui/releases) or setup [Rust](https://rustup.rs/), [Tauri](https://tauri.app/) and build it yourself:

```bash
git clone https://github.com/DarkCeptor44/trackers-gui
cd trackers-gui
cargo tauri build
```

You will find the binary in `src-tauri\target\release` and two bundles in `src-tauri\target\release\bundle`.

## License

This project is licensed under the MIT License, see the [LICENSE](LICENSE) file.
