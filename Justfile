build *args:
    cargo build {{args}}

run *args: build
    cargo run -- {{args}}
