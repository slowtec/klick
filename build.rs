use std::fs;

fn main() {
    let backend = read_cargo_toml("Cargo.toml");
    let frontend = read_cargo_toml("frontend/Cargo.toml");

    let backend_version = get_version(&backend);
    let frontend_version = get_version(&frontend);

    // Check if versions are the same
    assert_eq!(
        backend_version, frontend_version,
        "Versions are not the same: backend = {backend_version} vs frontend = {frontend_version}"
    );
}

fn read_cargo_toml(path: &str) -> toml::Value {
    fs::read_to_string(path).unwrap().parse().unwrap()
}

fn get_version(cargo_toml: &toml::Value) -> &str {
    cargo_toml
        .get("package")
        .unwrap()
        .get("version")
        .unwrap()
        .as_str()
        .unwrap()
}
