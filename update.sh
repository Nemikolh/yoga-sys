git clone git@github.com:facebook/yoga.git tmp
cp tmp/yoga/* yoga-sys/c/
rm -rf tmp
bindgen --no-unstable-rust --whitelist-function "^YG.*" yoga-sys/c/wrapper.h -o yoga-sys/src/ffi.rs
