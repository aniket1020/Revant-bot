# Revant - Discord Compiler Bot

Revant bot is a discord compiler bot made in Rust using Serenity discord framework.

## How to run this

Run
```
mkdir Workers/Workerdir{0..7}
```

If you are running this locally, you can do the following
```
cargo run               #To run in developement mode
# OR
cargo run --release     #To run release build
```

Add a `.env` in root folder with following format:
```
DISCORD_TOKEN=<DISCORD_TOKEN>
RUST_LOG=debug
```
