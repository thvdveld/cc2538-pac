#! /usr/bin/sh

svd2rust -i svd/cc2538sf53.svd
rm -rf src

form -i lib.rs -o src/ && rm lib.rs
cargo fmt
