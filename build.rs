// For eadkp's own build.rs, we directly include the builder module
// Library users will be able to do: eadkp::builder::setup()
mod builder {
    include!("src/builder.rs");
}

fn main() {
    builder::setup();
}