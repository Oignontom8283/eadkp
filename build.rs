// Pour le build.rs d'eadkp lui-même, on inclut directement le module builder
// Les utilisateurs de la lib pourront faire : eadkp::builder::setup()
mod builder {
    include!("src/builder.rs");
}

fn main() {
    builder::setup();
}