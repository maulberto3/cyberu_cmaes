dev-size:
	du ./target/debug/cyberu_cmaes -h

prod-size:
	du ./target/release/cyberu_cmaes -h

check:
	cargo check

fmt:
	cargo fmt

lint:
	cargo clippy --no-default-features 

build:
	clear && cargo build

clean:
	clear && cargo cache --autoclean && cargo clean

prep:
	clear && make fmt && make lint

run:
	clear && make build && cargo run

test:
	clear && cargo test --lib
	
rel:
	clear && make build && cargo run --release
