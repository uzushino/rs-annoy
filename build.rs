extern crate cc;

fn main() {
    cc::Build::new()
        .cpp(true)
        .warnings(false)
        .flag("-std=c++17")
        .file("src/c/binding.cc")
        .compile("binding");
}
