RUST_ARCH="x86_64-unknown-linux-musl"
RUSTUP_SHA256="1455d1df3825c5f24ba06d9dd1c7052908272a2cae9aa749ea49d67acbe22b47"
RUSTUP_URL="https://static.rust-lang.org/rustup/archive/1.27.1/${RUST_ARCH}/rustup-init"
wget "$RUSTUP_URL"
echo "${RUSTUP_SHA256} *rustup-init" | sha256sum -c -
chmod +x rustup-init
./rustup-init -y --no-modify-path --profile minimal --default-toolchain "$RUST_VERSION" --default-host "$RUST_ARCH"
rm rustup-init
chmod -R a+w "$RUSTUP_HOME" "$CARGO_HOME"
