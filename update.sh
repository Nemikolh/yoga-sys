git clone git@github.com:facebook/yoga.git tmp
cp tmp/yoga/* yoga-sys/c/
bindgen --no-unstable-rust --whitelist-function "^YG.*" yoga-sys/c/wrapper.h -o yoga-sys/src/ffi.rs