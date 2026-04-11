set shell := ["zsh", "-lc"]

default:
  @just --list

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all --check

check:
  cargo check --workspace

test:
  cargo test --workspace

lint:
  cargo clippy --workspace --all-targets -- -D warnings

verify: fmt-check check test lint

run-demo:
  cargo run -p demo

build-demo:
  cargo build -p demo --release

build-rpm:
  cargo build -p demo --release
  cargo generate-rpm -p demo

build-rpm-linux:
  rustup target add x86_64-unknown-linux-gnu
  RUST_FONTCONFIG_DLOPEN=1 cargo zigbuild -p demo --release --target x86_64-unknown-linux-gnu
  cargo generate-rpm -p demo --target x86_64-unknown-linux-gnu --auto-req disabled --metadata-overwrite demo/packaging/linux/generate-rpm-cross.toml

build-shared-android:
  ./unleash/build_shared_android_so.sh

gen-shared-kotlin:
  ./unleash/build_shared_android_kotlin.sh
