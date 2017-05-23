extern crate gcc;

fn main() {
    gcc::compile_library("libyoga.a", &[
        "c/YGEnums.c",
        "c/YGNodeList.c",
        "c/Yoga.c",
    ]);
}
