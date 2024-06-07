dev-size:
	du ./target/debug/rust_openes -h

prod-size:
	du ./target/release/rust_openes -h

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy --no-default-features 

clean:
	clear && cargo clean

build:
	clear && cargo build

test:
	clear && cargo test --tests

prep:
	clear && make fmt && make lint

run:
	clear && make build && cargo run

rel:
	clear && make build && cargo run --release
