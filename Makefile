#################
# Local Development Stuff
#################
run: dependencies
	cargo run --release

debug: dependencies
	cargo run --release -- ./assets/config/debug_world.config.ron

live-reload-local: dependencies
	cargo watch -x run

#################
# Build for Web
#################
web: dependencies
	rm -rf ./web/target/
	cargo build --target-dir target.wasm --release --target wasm32-unknown-unknown
	wasm-bindgen --out-name chinese_warriors --out-dir ./web/target --target web target.wasm/wasm32-unknown-unknown/release/chinese-warriors.wasm

# Run locally
serve: web
	basic-http-server web/

# Publish to Cadu's Test Server
publish: web
	rsync -avz -L --delete web/ caddy@new:/var/www/chinese-warriors/

#################
# Build Dependencies
#################

dependencies:
	rustup target add wasm32-unknown-unknown
	cargo install basic-http-server cargo-watch
