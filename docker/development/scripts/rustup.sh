RUST_ARCH="x86_64-unknown-linux-musl"
RUSTUP_SHA256="8e60c9157b7aa2bf32baab5c124b80a31dd24ba6c41b39b50645d354d381f831"
RUSTUP_URL="https://static.rust-lang.org/rustup/archive/1.28.1/${RUST_ARCH}/rustup-init"
wget "$RUSTUP_URL"
echo "${RUSTUP_SHA256} *rustup-init" | sha256sum -c -
chmod +x rustup-init
./rustup-init -y --no-modify-path --profile minimal --default-toolchain "$RUST_VERSION" --default-host "$RUST_ARCH"
rm rustup-init
chmod -R a+w "$RUSTUP_HOME" "$CARGO_HOME"
