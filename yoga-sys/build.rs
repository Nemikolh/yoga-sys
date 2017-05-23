extern crate gcc;
extern crate skeptic;

fn main() {
    skeptic::generate_doc_tests(&["README.md"]);
    gcc::compile_library("libyoga.a", &[
        "c/YGEnums.c",
        "c/YGNodeList.c",
        "c/Yoga.c",
    ]);
}
