fn main() {
    // Configure

    let sources = [
        "main.cpp"
    ];

    let mut build = cc::Build::new();

    build.cpp(true);

    // Compile

    for source in sources {
        println!("cargo:rerun-if-changed={source}");
        build.file(source);
    }
    let objects = build.compile_intermediates();

    // Link

    for object in objects {
        println!("cargo:rustc-link-arg={}", object.display());
    }
}