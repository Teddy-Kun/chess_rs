alias b := build
alias r := run

build:
    cargo +nightly b -r -Z build-std
    cbindgen --crate chess_rs --output target/release/libchess_rs.h --lang c

build-dev:
    cargo +nightly b -Z build-std
    cbindgen --crate chess_rs --output target/debug/libchess_rs.h --lang c

run:
    cargo +nightly r -r -Z build-std
    cbindgen --crate chess_rs --output target/release/libchess_rs.h --lang c

run-dev:
    cargo +nightly r -Z build-std
    cbindgen --crate chess_rs --output target/debug/libchess_rs.h --lang c

test:
    cargo test

clean:
    cargo clean
