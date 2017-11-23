extern crate cc;
extern crate skeptic;

fn main() {
    skeptic::generate_doc_tests(&["README.md"]);
    cc::Build::new()
        .file("c/YGEnums.cpp")
        .file("c/YGNodeList.cpp")
        .file("c/Yoga.cpp")
        .cpp(true)
        .flag_if_supported("-std=c++14")
        .flag_if_supported("/std:c++14")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("libyoga.a");
}
