# Dependust

A Rust tool to check for unused dependencies included in your crate by parsing your `Cargo.toml` and the source code in `./src`

## Depdendust in action

![2023-08-24_12-43](https://github.com/ByteSudoer/Dependust/assets/88513682/14ec550b-2433-4e3a-b65d-1f0b1cccff06)

## Installation

Clone the repositry and compile it

```bash
#Fetch The Source Code from github
git clone https://github.com/ByteSudoer/Dependust.git
# Compile the crate with cargo
cargo run --release
```

## Usage

You can use the binary globally by running this command :

```bash
# Run this command inside the cloned repo
cargo install --path .
```

To use under Nix/NixOS with direnv enabled you can run the shell:

```bash
#Create a .envrc file
echo "use flake" > .envrc
#And then activate the shell
direnv allow
```
