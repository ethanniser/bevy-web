# set shell := ["powershell.exe", "-c"]

run-web:
    cargo run --target wasm32-unknown-unknown

run:
    cargo run --features bevy/dynamic_linking