(cd ../da-host && cargo build --release)
# replace "use <some-id> as current;" with "use <arg> as current;" in src/lib.rs if arg is given, unless arg is equal to <some-id>
if [ -z "$1" ]; then
  cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
else
  # check if "use <arg> as current;" is already in src/lib.rs
    if grep -q "use $1 as current;" src/lib.rs; then
        cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
    else
        # replace "use <some-id> as current;" with "use <arg> as current;" in src/lib.rs
        sed -i "s/use [a-zA-Z0-9_]* as current;/use $1 as current;/g" src/lib.rs
        cargo build --release && ../da-host/target/release/da-host target/release/libpatch.so
    fi
fi
