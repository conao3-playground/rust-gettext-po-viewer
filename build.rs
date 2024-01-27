fn main() {
    cc::Build::new()
        .file("src/gettext.c")
        .compile("gettext");
}
