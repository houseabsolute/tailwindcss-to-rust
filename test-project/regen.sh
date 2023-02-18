#!/bin/bash

set -e
set -x

pushd ..
cargo build -p tailwindcss-to-rust
popd
../target/debug/tailwindcss-to-rust \
    --tailwind-config tailwind.config.js \
    --input ./css/tailwind.css \
    --output ./src/generated.rs \
    --rustfmt
tailwindcss --input ./css/tailwind.css --output ./assets/tailwind_compiled.css

