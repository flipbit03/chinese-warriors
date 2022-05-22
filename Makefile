dependencies:
	rustup target add wasm32-unknown-unknown
	cargo install basic-http-server cargo-watch

web: dependencies
	rm -rf ./web/target/
	cargo build --target-dir target.wasm --release --target wasm32-unknown-unknown
	wasm-bindgen --out-name chinese_warriors --out-dir ./web/target --target web target.wasm/wasm32-unknown-unknown/release/chinese-warriors.wasm

serve: web
	basic-http-server web/

publish: web
	rsync -avz -L --delete web/ caddy@new:/var/www/chinese-warriors/

live-reload-local:
	cargo watch -x run