# Getting Started

## Installing Prerequisites

### Install Make Tools

`sudo apt install make`

### Install Build Tools (GCC)

The _build-essential_ package includes the GCC compiler and other necessary tools for building C programs.

`sudo apt install build-essential`

### Install pkg-config and OpenSSL Development Libraries

If you encounter OpenSSL and \`pkg-config\` related issues during compilation:

`sudo apt install pkg-config libssl-dev`

## Setting Up Rust Dependencies

Ensure the following dependencies are specified in your _Cargo.toml_:

```
ndarray = { version = "0.15", features = ["blas", "rayon"] }
blas-src = { version = "0.10", features = ["openblas"] }
openblas-src = { version = "0.10", features = ["cblas", "system"] }
ndarray-linalg = { version = "0.16", features = ["openblas-system"] }
ndarray-rand = { version = "0.14" }
ndarray-stats = { version = "0.5.1" }
```

## Installing OpenBLAS

To use OpenBLAS system-wide, install the \`libopenblas-dev\` package:

`sudo apt install libopenblas-dev`

### Checking OpenBLAS Installation

To verify where OpenBLAS is installed:

`dpkg-query -L libopenblas-dev`

## Additional Tools

Install _\`_cargo-depgraph_\`_ and _\`_graphviz_\`_ for dependency visualization:

```
cargo install cargo-depgraph
sudo apt install graphviz
```

## For git

Since it's a fresh ubuntu build, for git:

`git config --global user.name "Your Name"`
`git config --global user.email "your.email@example.com"`

Then, checkgithub key, if 

`ssh -T git@github.com`

says _git@github.com: Permission denied (publickey)._

Then, probably the key pair was lost, due to new ubuntu fresh install, so do 

`ls -al ~/.ssh`

and see if you indeed have keys stored, if not, then

`ssh-keygen -t ed25519 -C "your_email@example.com"`

`ssh-add`

Then add it to github.com

`cat ~/.ssh/id_ed25519.pub`

Then paste that under Settings, SSH and GPG Keys and that's it.


