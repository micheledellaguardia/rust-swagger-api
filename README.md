# build con profilo release (dopo aver aggiornato Cargo.toml)
cargo build --release

# rimuovere simboli di debug (se non già strip)
strip target/release/rust-swagger-api

# opzionale: compressione con upx
upx --best --lzma target/release/rust-swagger-api

# avviare il docker
# build dell'immagine (esegui nella cartella del progetto)
docker build -t rust-swagger-api:latest .

# esegui il container mappando la porta 3000
docker run --rm -p 3000:3000 rust-swagger-api:latest
