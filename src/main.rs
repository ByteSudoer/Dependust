use std::collections::HashSet;
use std::fs;
use std::path::Path;
use toml::Value;
use walkdir::WalkDir;

fn parse_dependencies(cargo_toml: &str) -> HashSet<String> {
    let cargo_toml: Value = toml::from_str(cargo_toml).expect("Failed to parse Cargo.toml");

    let mut dependencies = HashSet::new();
    if let Some(deps) = cargo_toml["dependencies"].as_table() {
        for (dep_name, _dep_spec) in deps {
            dependencies.insert(dep_name.to_string());
        }
    }

    dependencies
}

fn find_used_dependencies(source_path: &Path) -> HashSet<String> {
    let mut used_dependencies = HashSet::new();

    for entry in WalkDir::new(source_path).into_iter().filter_map(Result::ok) {
        if entry.path().is_file()
            && entry
                .path()
                .extension()
                .map(|ext| ext == "rs")
                .unwrap_or(false)
        {
            if let Ok(contents) = fs::read_to_string(entry.path()) {
                for line in contents.lines() {
                    if let Some(import) = line.trim().strip_prefix("use ") {
                        let dep_name = import.split_once("::").map(|(name, _)| name.trim());
                        if let Some(name) = dep_name {
                            used_dependencies.insert(name.to_string());
                        }
                    }
                }
            }
        }
    }

    used_dependencies
}

fn main() {
    let cargo_toml_path = "Cargo.toml";
    let source_path = Path::new("src");

    let cargo_toml = fs::read_to_string(cargo_toml_path).expect("Failed to read Cargo.toml");
    let dependencies = parse_dependencies(&cargo_toml);
    let used_dependencies = find_used_dependencies(source_path);

    let unused_dependencies: HashSet<_> = dependencies
        .difference(&used_dependencies)
        .cloned()
        .collect();

    println!("Present Dependencies : {:?}", used_dependencies);

    if unused_dependencies.is_empty() {
        println!("No unused dependencies detected.");
    } else {
        println!("Potentially unused dependencies:");
        for dep in unused_dependencies {
            println!("{}", dep);
        }
    }
}
