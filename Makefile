build_smartcontract:
	cd smartcontract && cargo build --release && cd ..

run: build_smartcontract
	cargo +nightly run
