(cd ../da-host && cargo build --release)

export INCDIR=csound

if [ -z "$1" ]; then
  cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
else
    if grep -q "use $1 as current;" src/lib.rs; then
        cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
    else
        sed -i "s/use [a-zA-Z0-9_:]* as current;/use $1 as current;/g" src/lib.rs
        cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
    fi
fi
