default:
    @just -l

build:
    cargo build

publish:
    cargo fmt
    cargo clippy -q -- -D warnings
    cargo test -q

loc:
    find . -name "*.rs" | xargs cat | wc -l
