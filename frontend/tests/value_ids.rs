use std::{collections::HashSet, fs, path::Path};

use colored::*;
use strum::IntoEnumIterator;
use walkdir::WalkDir;

use klick_domain::InputValueId as In;

#[test]
fn input_value_ids_are_used_at_least_once() {
    let subdirs = &[
        "src/pages/tool/plant_profile",
        "src/pages/tool/sensitivity_parameters",
        "src/pages/tool/recommendations",
    ];

    for id in In::iter() {
        let all_occurrences = find_variant(subdirs, id);
        if !all_occurrences.is_empty() {
            continue;
        }
        let variant_name = format!("{id:?}").yellow().bold();
        let subdirs = subdirs
            .into_iter()
            .map(|dir| format!("  - {}", dir.blue()))
            .collect::<Vec<_>>()
            .join("\n");
        let not_used_in_any = "not used in any".red().bold();
        panic!("The variant {variant_name} was {not_used_in_any} of these directories:\n{subdirs}");
    }
}

#[ignore] // FIXME
#[test]
fn input_value_ids_are_used_at_most_once() {
    let subdirs = &[
        "src/pages/tool/plant_profile",
        "src/pages/tool/sensitivity_parameters",
        "src/pages/tool/recommendations",
    ];

    // TODO:
    let skip = [In::ProfileSewageGasProduced]
        .into_iter()
        .collect::<HashSet<In>>();

    for id in In::iter().filter(|v| !skip.contains(v)) {
        let all_occurrences = find_variant(subdirs, id);
        let variant_name = format!("{id:?}").yellow().bold();

        if all_occurrences.len() < 2 {
            continue;
        }

        let all_occurrences = all_occurrences
            .into_iter()
            .map(|o| {
                let line_number = format!("{}", o.line_number).yellow();
                let contents = o.line_content.trim().bright_black();
                let file = o.file.blue();
                (file, line_number, contents)
            })
            .collect::<Vec<_>>();
        let max_file_name_len = all_occurrences
            .iter()
            .map(|(f, _, _)| f.len())
            .max()
            .unwrap_or(0);
        let max_line_number_len = all_occurrences
            .iter()
            .map(|(_, l, _)| l.len())
            .max()
            .unwrap_or(0);
        let occurrences = all_occurrences
            .into_iter()
            .map(|(file, line, contents)| {
                format!(
                    "  - {file:max_file_name_len$}  line {line:max_line_number_len$} : {contents}"
                )
            })
            .collect::<Vec<_>>()
            .join("\n");

        let more_then_once = "more then once".red().bold();
        panic!("The variant {variant_name} was used {more_then_once}:\n{occurrences}");
    }
}

fn find_variant(directories: &[&str], variant: In) -> Vec<Occurrence> {
    let variant_name = format!("{variant:?}");
    let mut all_occurrences = Vec::new();

    for src_path in directories {
        for entry in WalkDir::new(src_path).into_iter().filter_map(|e| e.ok()) {
            let path = entry.path();

            if path.is_file() && path.extension().and_then(|s| s.to_str()) == Some("rs") {
                let occurrences = find_variant_in_file(&variant_name, &path);
                all_occurrences.extend(occurrences);
            }
        }
    }
    all_occurrences
}

fn find_variant_in_file(variant: &str, path: &Path) -> Vec<Occurrence> {
    let content = fs::read_to_string(path).expect("Could not read file");
    let mut occurrences = Vec::new();

    for (line_number, line) in content.lines().enumerate() {
        if line.contains(variant) {
            occurrences.push(Occurrence {
                file: path.to_string_lossy().to_string(),
                line_number: line_number + 1,
                line_content: line.to_string(),
            });
        }
    }
    occurrences
}

#[derive(Debug)]
struct Occurrence {
    file: String,
    line_number: usize,
    line_content: String,
}
