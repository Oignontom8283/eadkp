// For eadkp's own build.rs, we directly include the builder module
// Library users will be able to do: eadkp::builder::setup()
mod builder {
    include!("src/builder.rs");
}

fn main() {
    // Ne s'exécute que si la variable EADKP_PRIMARY_BUILD est définie
    // Cette variable est définie dans docker-compose.yml lors de la compilation du projet eadkp
    // Quand eadkp est utilisé comme dépendance, cette variable ne sera pas définie
    if std::env::var("EADKP_PRIMARY_BUILD").is_ok() {
        builder::setup();
    }
}