fn main() {
    let mut cc_build = cc::Build::new();
    let cflags = ""; // TODO
    cc_build.flag(cflags);
    cc_build
        .cpp(true)
        .file("src/manual.cpp")
        .file("src/generated.cpp")
        .include("include")
        .flag_if_supported("-std=c++14")
        // ignore too many warnings with wx3.0
        .flag_if_supported("-Wno-deprecated-copy")
        .flag_if_supported("-Wno-ignored-qualifiers")
        .flag_if_supported("-Wno-unused-parameter")
        .compile("wx");

    let libs = ""; // TODO
    println!("cargo:rustc-flags={}", libs);
}
