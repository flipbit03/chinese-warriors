dependencies:
	rustup target add wasm32-unknown-unknown
	cargo install basic-http-server cargo-watch

web: dependencies
	rm -rf dist
	mkdir -p dist
	cargo build -r --target wasm32-unknown-unknown
	cp target/wasm32-unknown-unknown/release/chinese-survivors.wasm dist/
	cp assets/index.html dist/
	cp -r assets dist/
	rm dist/assets/index.html # ;-)

serve: web
	basic-http-server dist/

publish: web
	rsync -avz --delete dist/ caddy@new:/var/www/chinese-warriors/

live-reload-local:
	cargo watch -x run