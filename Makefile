.PHONY: run debug web serve publish dependencies

#################
# Local Development Stuff
#################
run:
	cargo run --release

debug:
	cargo run --release -- ./assets/config/debug_world.config.ron

#################
# Build for Web
#################
web:
	rm -rf ./web/target/
	cargo build --target-dir target.wasm --release --target wasm32-unknown-unknown
	wasm-bindgen --out-name chinese_warriors --out-dir ./web/target --target web target.wasm/wasm32-unknown-unknown/release/chinese-warriors.wasm

# Run locally (python3 http server)
serve: web
	@echo "Serving at http://localhost:8080"
	cd web && python3 -m http.server 8080

# Publish to Cadu's Test Server
publish: web
	rsync -avz -L --delete web/ caddy@new:/var/www/chinese-warriors/

#################
# Build Dependencies
#################

dependencies:
	rustup target add wasm32-unknown-unknown
	cargo install wasm-bindgen-cli
