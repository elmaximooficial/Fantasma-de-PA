$env:RUSTFLAGS = "-C link-arg=--import-memory -C link-arg=--initial-memory=65536 -C link-arg=--max-memory=65536 -C link-arg=-zstack-size=14752"
cargo build --release
cp .\target\wasm32-unknown-unknown\release\luccas.wasm .\
w4 run .\luccas.wasm