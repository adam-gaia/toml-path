default:
    @just --list

check:
    cargo lclippy

build:
    cargo lbuild

run:
    RUST_LOG=debug cargo lrun

test:
    cargo lbuild --tests
    cargo nextest run

fmt:
    treefmt

docs:
    oranda build
    oranda serve

cov:
    nix build .#packages.x86_64-linux.llm-coverage
