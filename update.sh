git clone git@github.com:facebook/yoga.git tmp
cp tmp/yoga/* c/
rm -rf tmp
bindgen --no-unstable-rust --whitelist-function "^YG.*" c/wrapper.h -o src/ffi.rs
