set shell := ["zsh", "-lc"]

default:
  @just --list

fmt:
  cargo fmt --all

fmt-check:
  cargo fmt --all --check

check:
  cargo xtask check

test:
  cargo xtask test

lint:
  cargo xtask lint

verify:
  cargo xtask verify

run-demo:
  cargo xtask run-demo

build-demo:
  cargo xtask build-demo

build-rpm:
  cargo xtask build-rpm

build-rpm-linux:
  cargo xtask build-rpm-linux
