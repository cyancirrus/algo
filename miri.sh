# rustup toolchain install nightly
# rustup component add miri --toolchain nightly
# cargo +nightly miri setup
# cargo +nightly miri test

# enforces the borrow stack
MIRIFLAGS="-Zmiri-tag-raw-pointers" cargo +nightly miri test
