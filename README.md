# Youtube Music Discord RPC

Discord Rich Presence for Youtube Music activity in any browser

## Installation

### Install it via cargo

```bash
cargo install youtube-music-discord-rpc
```

### Manual installation

```bash
cargo build --release --locked
```

Then copy the binary from `target/release/youtube-music-discord-rpc` to a `bin`
folder on the path and make it executable.

```bash
cp target/release/youtube-music-discord-rpc ~/.local/bin
chmod +x ~/.local/bin/youtube-music-discord-rpc
```

Assuming your bin folder is on the path:

```bash
youtube-music-discord-rpc
```