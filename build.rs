use std::fs;

fn main() {
    let workspace_cargo_toml = read_cargo_toml("Cargo.toml");
    let frontend_cargo_toml = read_cargo_toml("frontend/Cargo.toml");

    let frontend_version = get_package_version(&frontend_cargo_toml).as_str().unwrap();

    let backend_uses_workspace_version = get_package_version(&workspace_cargo_toml)
        .get("workspace")
        .unwrap()
        .as_bool()
        .unwrap();

    assert!(backend_uses_workspace_version);

    let workspace_version = get_package_version(workspace_cargo_toml.get("workspace").unwrap())
        .as_str()
        .unwrap();

    // Check if versions are the same
    assert_eq!(
        workspace_version, frontend_version,
        "Versions are not the same: workspace = {workspace_version} vs. frontend = {frontend_version}"
    );
}

fn read_cargo_toml(path: &str) -> toml::Value {
    fs::read_to_string(path).unwrap().parse().unwrap()
}

fn get_package_version(cargo_toml: &toml::Value) -> &toml::Value {
    cargo_toml.get("package").unwrap().get("version").unwrap()
}
