fn main() {
    let lib = pkg_config::Config::new()
        .atleast_version("0.24")
        .probe("lilv-0")
        .expect("Did not find 'lilv-0' the C dependency required for this crate. Please install the 'lilv-dev' package.");

    cc::Build::new()
        .include("src-c")
        .file("src-c/main.c")
        .includes(&lib.include_paths)
        .compile("lilv-test");

    println!("cargo:rerun-if-changed=src-c/main.c");
}
