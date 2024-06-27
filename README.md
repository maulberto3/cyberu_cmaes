# 🚀 Getting Started

## Installing Prerequisites

### Install Make Tools

\`\`\`bash
sudo apt install make
\`\`\`

### Install Build Tools (GCC)

The \`build-essential\` package includes the GCC compiler and other necessary tools for building C programs.

\`\`\`bash
sudo apt install build-essential
\`\`\`

### Install pkg-config and OpenSSL Development Libraries

If you encounter OpenSSL and \`pkg-config\` related issues during compilation:

\`\`\`bash
sudo apt install pkg-config libssl-dev
\`\`\`

## Setting Up Rust Dependencies

Ensure the following dependencies are specified in your \`Cargo.toml\`:

\`\`\`toml
ndarray = { version = "0.15", features = ["blas", "rayon"] }
blas-src = { version = "0.10", features = ["openblas"] }
openblas-src = { version = "0.10", features = ["cblas", "system"] }
ndarray-linalg = { version = "0.16", features = ["openblas-system"] }
ndarray-rand = { version = "0.14" }
ndarray-stats = { version = "0.5.1" }
\`\`\`

## Installing OpenBLAS

To use OpenBLAS system-wide, install the \`libopenblas-dev\` package:

\`\`\`bash
sudo apt install libopenblas-dev
\`\`\`

### Checking OpenBLAS Installation

To verify where OpenBLAS is installed:

\`\`\`bash
dpkg-query -L libopenblas-dev
\`\`\`

## Additional Tools

Install \`cargo-depgraph\` and \`graphviz\` for dependency visualization:

\`\`\`bash
cargo install cargo-depgraph
sudo apt install graphviz
\`\`\`
