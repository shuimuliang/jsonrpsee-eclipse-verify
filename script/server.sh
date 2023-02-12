cd ../
export DATABASE_URL="postgresql://solana:jw8s0F4@127.0.0.1/solana"
export HOST="127.0.0.1"
export PORT="3000"
RUST_LOG=info
cargo run --bin jsonrpsee-eclipse-verify
