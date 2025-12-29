Basic Rust API + swagger

Just for study purpose, not a production-ready code
Feel free to share

--- build with release profile (after updating Cargo.toml)
cargo build --release

--- remove debug symbols (if not stripped)
strip target/release/rust-swagger-api

--- optional: upx compression
upx --best --lzma target/release/rust-swagger-api

--- Dockerize

--- build docker image (execute inside project's folder)
docker build -t rust-swagger-api:latest .

--- execute container mapping port 3000
docker run --rm -p 3000:3000 rust-swagger-api:latest
