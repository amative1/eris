# eris
LRRbot's Discord part but in Rust because discord.py sucks out loud.

## License
Licensed under Apache-2.0 ([LICENSE](LICENSE) or [https://www.apache.org/licenses/LICENSE-2.0](https://www.apache.org/licenses/LICENSE-2.0)).

## Setup instructions
Needs a [LRRbot](https://github.com/mrphlip/lrrbot) to run. Uses nightly Rust for `async` and `await` without which this would be actually impossible.

Roughly:
```bash
rustup toolchain install nightly
cd /path/to/lrrbot
# in a different terminal: . venv/bin/activate; python3 start_bot.py
cargo +nightly run --manifest-path /path/to/eris/Cargo.toml --release
```

