# yoga-sys [![Build Status](https://travis-ci.org/Nemikolh/yoga-sys.svg?branch=master)](https://travis-ci.org/Nemikolh/yoga-sys)[![Build Status](https://ci.appveyor.com/api/projects/status/github/Nemikolh/yoga-sys?svg=true)](https://ci.appveyor.com/project/Nemikolh/yoga-sys)

Raw rust bindings for yoga.

> Disclaimer: Those bindings are not provided by any of the facebook
>             maintainers and thus may contains additional bugs.

## Getting started:

Add to your `Cargo.toml`:

```toml
[dependencies]
yoga-sys = "0.2.3"
```

In your `main.rs` or `lib.rs` file add:

```rs
extern crate yoga_sys;
```

## Example

Here is the example that you can find [here](https://facebook.github.io/yoga/)
translated to rust using this crate:

```rust
extern crate yoga_sys;

use yoga_sys::*;

fn main() {
    unsafe {
        let root = YGNodeNew();
        YGNodeStyleSetWidth(root, 500.);
        YGNodeStyleSetHeight(root, 120.);
        YGNodeStyleSetFlexDirection(root, YGFlexDirection::YGFlexDirectionRow);
        YGNodeStyleSetPadding(root, YGEdge::YGEdgeAll, 20.);

        let image = YGNodeNew();
        YGNodeStyleSetWidth(image, 80.);
        YGNodeStyleSetMargin(image, YGEdge::YGEdgeEnd, 20.);

        let text = YGNodeNew();
        YGNodeStyleSetHeight(text, 25.);
        YGNodeStyleSetAlignSelf(text, YGAlign::YGAlignCenter);
        YGNodeStyleSetFlexGrow(text, 1.);

        YGNodeInsertChild(root, image, 0);
        YGNodeInsertChild(root, text, 1);

        YGNodeFreeRecursive(root);
    }
}
```
