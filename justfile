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
  mkdir -p build/x86_64-unknown-linux-gnu/release/rpm && \
  version=$(sed -n 's/^version = "\(.*\)"/\1/p' demo/Cargo.toml | head -n 1) && \
  src_rpm=$(find target/generate-rpm -maxdepth 1 -name 'demo-*.rpm' -print -quit) && \
  test -n "$src_rpm" || { echo 'No RPM artifacts were found in target/generate-rpm/'; exit 1; } && \
  find build/x86_64-unknown-linux-gnu/release/rpm -maxdepth 1 -name 'demo-v*.rpm' -delete && \
  cp "$src_rpm" "build/x86_64-unknown-linux-gnu/release/rpm/demo-v${version}.rpm" && \
  echo "build/x86_64-unknown-linux-gnu/release/rpm/demo-v${version}.rpm"

build-rpm-linux:
  rustup target add x86_64-unknown-linux-gnu
  RUST_FONTCONFIG_DLOPEN=1 cargo zigbuild -p demo --release --target x86_64-unknown-linux-gnu
  cargo generate-rpm -p demo --target x86_64-unknown-linux-gnu --auto-req disabled --metadata-overwrite demo/packaging/linux/generate-rpm-cross.toml
  mkdir -p build/x86_64-unknown-linux-gnu/release/rpm && \
  version=$(sed -n 's/^version = "\(.*\)"/\1/p' demo/Cargo.toml | head -n 1) && \
  src_rpm=$(find target/x86_64-unknown-linux-gnu/generate-rpm -maxdepth 1 -name 'demo-*.rpm' -print -quit) && \
  test -n "$src_rpm" || { echo 'No RPM artifacts were found in target/x86_64-unknown-linux-gnu/generate-rpm/'; exit 1; } && \
  find build/x86_64-unknown-linux-gnu/release/rpm -maxdepth 1 -name 'demo-v*.rpm' -delete && \
  cp "$src_rpm" "build/x86_64-unknown-linux-gnu/release/rpm/demo-v${version}.rpm" && \
  echo "build/x86_64-unknown-linux-gnu/release/rpm/demo-v${version}.rpm"

build-shared-android:
  ./unleash/build_shared_android_so.sh

gen-shared-kotlin:
  ./unleash/build_shared_android_kotlin.sh
