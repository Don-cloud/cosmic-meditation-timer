# Meditation Countdown Timer (Rust)

Simple desktop countdown timer for Pop!_OS / COSMIC.
Happy Meditation!
- Sajal

## Features
- Enter `Hours`, `Minutes`, and/or `Seconds`
- Start, pause, and reset countdown
- Live `HH:MM:SS` display
- Plays a synthesized meditation-style bell when timer reaches zero

## Prerequisites (Pop!_OS 24.04)
Install Rust toolchain:

```bash
sudo apt update
sudo apt install -y curl build-essential pkg-config libasound2-dev
curl https://sh.rustup.rs -sSf | sh
source "$HOME/.cargo/env"
```

## Run

```bash
cd cosmic-meditation-timer
cargo run
```

## Build release

```bash
cargo build --release
```

Binary will be at:

```text
target/release/cosmic-meditation-timer
```

## Add to COSMIC app menu
After building release, install binary, icon, and desktop launcher:

```bash
mkdir -p ~/.local/bin
cp target/release/cosmic-meditation-timer ~/.local/bin/
chmod +x ~/.local/bin/cosmic-meditation-timer

mkdir -p ~/.local/share/icons/hicolor/scalable/apps
cp assets/buddha-meditation.svg ~/.local/share/icons/hicolor/scalable/apps/cosmic-meditation-timer.svg

mkdir -p ~/.local/share/applications
cp cosmic-meditation-timer.desktop ~/.local/share/applications/
```

## Custom icon
This project includes a Buddha meditation icon at:

```text
assets/buddha-meditation.svg
```

After editing the `.desktop` file, recopy it:

```bash
cp cosmic-meditation-timer.desktop ~/.local/share/applications/
```

## License
This project is licensed under the MIT License.

See [LICENSE](LICENSE) for full text.
