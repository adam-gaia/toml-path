default:
    @just --list

check:
    cargo lclippy

build:
    cargo lbuild

run:
    RUST_LOG=debug cargo lrun

doctest:
    # cargo-nextest doesn't yet support doctests
    # https://github.com/nextest-rs/nextest/issues/16
    cargo ltest --doc

test:
    cargo lbuild --tests
    cargo nextest run --all-targets

fmt:
    treefmt

docs:
    oranda build
    oranda serve

cov:
    nix build .#packages.x86_64-linux.llm-coverage
