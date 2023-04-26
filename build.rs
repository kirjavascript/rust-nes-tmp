fn main() {
    cc::Build::new()
        .compiler("clang")
        .target("mos-nes")
        .file("src/nmi.c")
        .compile("nmi");
}
