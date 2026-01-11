// For eadkp's own build.rs, we directly include the builder module
// Library users will be able to do: eadkp::builder::setup()
mod builder {
    include!("src/builder.rs");
}

fn main() {
    // Si le projet est utilisé comme dépendance, ne rien faire
    if std::env::var_os("CARGO_PRIMARY_PACKAGE").is_none() {
        return;
    }
    builder::setup();
}