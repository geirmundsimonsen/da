(cd ../da-host && cargo build --release)

export INCDIR=csound
export DA_CSOUND=$1

if grep -q "use csound as current;" src/lib.rs; then
    cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
else
    sed -i "s/use [a-zA-Z0-9_:]* as current;/use csound as current;/g" src/lib.rs
    cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
fi

