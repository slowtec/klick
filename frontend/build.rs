use pulldown_cmark::{html, Options, Parser};
use std::{fs, path::Path};
use walkdir::WalkDir;

const MARKDOWN_CONTENT_DIR: &str = "content";
const HTML_DIST_DIR: &str = "target/content";

fn main() {
    let workspace_cargo_toml = read_cargo_toml("../Cargo.toml");
    let frontend_cargo_toml = read_cargo_toml("Cargo.toml");

    let frontend_version = get_package_version(&frontend_cargo_toml).as_str().unwrap();
    let workspace_version = get_package_version(workspace_cargo_toml.get("workspace").unwrap())
        .as_str()
        .unwrap();

    // Check if versions are the same
    assert_eq!(
        workspace_version, frontend_version,
        "Versions are not the same: workspace = {workspace_version} vs frontend = {frontend_version}"
    );

    compile_markdown_files();
}

fn compile_markdown_files() {
    let content_dir = Path::new(MARKDOWN_CONTENT_DIR);
    let dist_dir = Path::new(HTML_DIST_DIR);

    fs::create_dir_all(dist_dir).unwrap();

    for entry in WalkDir::new(content_dir) {
        let entry = entry.unwrap();
        if entry.file_type().is_file() && entry.path().extension().is_some_and(|s| s == "md") {
            let markdown_path = entry.path();
            let html_path = dist_dir.join(
                markdown_path
                    .strip_prefix(content_dir)
                    .unwrap()
                    .with_extension("html"),
            );

            if let Some(parent) = html_path.parent() {
                fs::create_dir_all(parent).unwrap();
            }

            let content = fs::read_to_string(markdown_path).unwrap();

            let html_content = markdown_to_html(&content);
            fs::write(html_path, html_content).unwrap();
        }
    }
}

fn read_cargo_toml(path: &str) -> toml::Value {
    fs::read_to_string(path).unwrap().parse().unwrap()
}

fn get_package_version(cargo_toml: &toml::Value) -> &toml::Value {
    cargo_toml.get("package").unwrap().get("version").unwrap()
}

fn markdown_to_html(content: &str) -> String {
    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);
    html_output
}
