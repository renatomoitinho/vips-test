## vips-test

export export RUSTFLAGS="$(pkg-config --libs vips)"

cargo build
cargo build --release

./vips-test picture.jpg 300

