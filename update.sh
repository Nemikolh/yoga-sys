git clone git@github.com:facebook/yoga.git tmp --depth 1
cp tmp/yoga/* c/
rm -rf tmp
bindgen --whitelist-function "^YG.*" c/wrapper.h -o src/ffi.rs --rustified-enum "(YGAlign|YGDimension|YGDirection|YGEdge|YGDisplay|YGExperimentalFeature|YGFlexDirection|YGLogLevel|YGJustify|YGMeasureMode|YGNodeType|YGOverflow|YGPositionType|YGPrintOptions|YGUnit|YGWrap)"
