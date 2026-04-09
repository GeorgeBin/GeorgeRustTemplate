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

run-dashboard:
  cargo run -p dashboard

run-demo:
  cargo run -p demo

build-dashboard:
  cargo build -p dashboard --release

build-demo:
  cargo build -p demo --release

build-shared-android:
  ./unleash/build_shared_android_so.sh

gen-shared-kotlin:
  ./unleash/build_shared_android_kotlin.sh
