dev-size:
	clear && du ./target/debug/cyberu_cmaes -h

prod-size:
	clear && du ./target/release/cyberu_cmaes -h

ex:
	#  --features=openblas
	clear && cargo run --example ex1 

clean:
	clear && cargo cache --autoclean && cargo clean

prep:
	clear && cargo fmt && cargo check && cargo clippy && cargo build && cargo tree && make graph-dep

graph-dep:
	# graphviz must be installed: sudo apt install graphviz
	cargo depgraph --all-deps | dot -Tpng > dependencies_graph_of_current_cargo_toml.png

test:
	clear && cargo test --lib

run:
	clear && cargo run
	
rel:
	clear && cargo run --release
