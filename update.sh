git clone git@github.com:facebook/yoga.git tmp --depth 1
cp tmp/yoga/* c/
rm -rf tmp
bindgen --whitelist-function "^YG.*" c/wrapper.h -o src/ffi.rs --rustified-enum "(YGAlign|YGDimension|YGDirection|YGEdge|YGDisplay|YGExperimentalFeature|YGFlexDirection|YGLogLevel|YGJustify|YGMeasureMode|YGNodeType|YGOverflow|YGPositionType|YGPrintOptions|YGUnit|YGWrap)"
sed -i -E "s/(\s+)\.([a-zA-Z]+)(\s+)=(.*)/\1\4/" c/Yoga-internal.h c/Yoga.cpp
sed -i "s/, .unit = /, /" c/Yoga-internal.h c/Yoga.cpp
sed -i "s/{.value = /{ /" c/Yoga-internal.h c/Yoga.cpp
sed -i -E "s/\{\s*\[.+\] =/\{/" c/Yoga-internal.h c/Yoga.cpp
sed -i -E "s/(.+)(\s+)\[.+\] = /\1\2/" c/Yoga-internal.h c/Yoga.cpp
sed -i -E "s/(.+)(\s+)\[.+\] = /\1\2/" c/Yoga-internal.h c/Yoga.cpp
