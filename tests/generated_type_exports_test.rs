use std::collections::BTreeSet;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};

#[test]
fn generated_types_include_every_ts_export() {
    let expected = collect_source_ts_exports().expect("collect ts-rs source exports");
    let generated = collect_generated_type_exports().expect("collect generated TypeScript exports");

    assert_eq!(
        expected, generated,
        "frontend/src/types/generated.ts must contain exactly the #[ts(export)] Rust DTOs"
    );
}

#[test]
fn generated_types_are_sorted_and_self_contained() {
    let generated = fs::read_to_string("frontend/src/types/generated.ts")
        .expect("read generated TypeScript types");
    let mut previous = None;

    for line in generated.lines() {
        assert!(
            !line.starts_with("import type "),
            "generated TypeScript file should not import sibling generated files"
        );

        let Some(name) = export_name_from_generated_line(line) else {
            continue;
        };

        if let Some(previous_name) = previous {
            assert!(
                previous_name < name,
                "generated TypeScript exports must be sorted alphabetically"
            );
        }

        previous = Some(name);
    }
}

fn collect_source_ts_exports() -> io::Result<BTreeSet<String>> {
    let mut files = Vec::new();
    collect_rs_files(Path::new("src"), &mut files)?;
    files.sort();

    let mut exports = BTreeSet::new();
    for file in files {
        collect_source_ts_exports_from_file(&file, &mut exports)?;
    }

    Ok(exports)
}

fn collect_rs_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    for entry in fs::read_dir(dir)? {
        let path = entry?.path();

        if path.is_dir() {
            collect_rs_files(&path, files)?;
            continue;
        }

        if path.extension().and_then(|ext| ext.to_str()) == Some("rs") {
            files.push(path);
        }
    }

    Ok(())
}

fn collect_source_ts_exports_from_file(
    path: &Path,
    exports: &mut BTreeSet<String>,
) -> io::Result<()> {
    let contents = fs::read_to_string(path)?;
    let mut pending_export = false;

    for line in contents.lines() {
        let trimmed = line.trim_start();

        if trimmed.starts_with("#[ts(export") {
            pending_export = true;
            continue;
        }

        if !pending_export {
            continue;
        }

        if let Some(name) = rust_type_name(trimmed) {
            exports.insert(name.to_owned());
            pending_export = false;
        }
    }

    Ok(())
}

fn rust_type_name(line: &str) -> Option<&str> {
    let mut parts = line.split_whitespace();

    match (parts.next(), parts.next(), parts.next()) {
        (Some("pub"), Some("struct" | "enum"), Some(name)) => Some(clean_type_name(name)),
        _ => None,
    }
}

fn clean_type_name(name: &str) -> &str {
    name.split(['<', '{', '(']).next().unwrap_or(name)
}

fn collect_generated_type_exports() -> io::Result<BTreeSet<String>> {
    let generated = fs::read_to_string("frontend/src/types/generated.ts")?;

    Ok(generated
        .lines()
        .filter_map(export_name_from_generated_line)
        .map(str::to_owned)
        .collect())
}

fn export_name_from_generated_line(line: &str) -> Option<&str> {
    line.strip_prefix("export type ")
        .and_then(|rest| rest.split_whitespace().next())
}
