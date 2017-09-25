extern crate cc;
extern crate skeptic;

fn main() {
    skeptic::generate_doc_tests(&["README.md"]);
    cc::Build::new()
        .file("c/YGEnums.c")
        .file("c/YGNodeList.c")
        .file("c/Yoga.c")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("libyoga.a");
}
