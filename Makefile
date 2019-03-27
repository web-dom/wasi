build:
	node tools/generate.js
	cargo fmt
	cargo build;
