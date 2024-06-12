dev-size:
	clear && du ./target/debug/cyberu_cmaes -h

prod-size:
	clear && du ./target/release/cyberu_cmaes -h

check:
	clear && cargo check

fmt:
	clear && cargo fmt

lint:
	clear && cargo clippy --no-default-features 

build:
	clear && cargo check && cargo build

clean:
	clear && cargo cache --autoclean && cargo clean

prep:
	clear && make fmt && make lint

run:
	clear && cargo run

test:
	clear && cargo test --lib
	
rel:
	clear && cargo run --release

graph-dep:
	# graphviz must be installed: sudo apt install graphviz
	clear && cargo depgraph --all-deps | dot -Tpng > graph.png
