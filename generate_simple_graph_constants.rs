use fetch_unroll::Fetch;
use glob::glob;
use std::{
    fs::OpenOptions,
    io::Write,
    path::{Path, PathBuf},
};

const SIMPLE_GRAPH_REMOTE_URL_BASE: &str =
    "https://github.com/dpapathanasiou/simple-graph/archive/refs/tags/";
const SIMPLE_GRAPH_DEFAULT_VERSION: &str = "2.1.1";
const TEMP_DIR: &str = "temp";
const TEMP_ARCHIVE_DIR: &str = "simple_graph";

// Macro from this advice: https://stackoverflow.com/a/40567215/22130324
// Used this answer because it supports constants
macro_rules! build_from_paths {
    ($base:expr, $($segment:expr),+) => {{
        let mut base: ::std::path::PathBuf = $base.into();
        $(
            base.push($segment);
        )*
        base
    }}
}

fn main() {
    println!("cargo:rerun-if-changed=build.rs");

    let simple_graph_url = format!(
        "{}v{}.tar.gz",
        SIMPLE_GRAPH_REMOTE_URL_BASE,
        option_env!("SIMPLE_GRAPH_PKG_VERSION").unwrap_or(SIMPLE_GRAPH_DEFAULT_VERSION)
    );
    let temp_simple_graph_dir: PathBuf = build_from_paths!(TEMP_DIR, TEMP_ARCHIVE_DIR);

    // Fetching and unrolling archive
    Fetch::from(simple_graph_url)
        .unroll()
        .strip_components(1)
        .to(&temp_simple_graph_dir)
        .unwrap();

    // Clear any stale constants.rs file
    let constants_file_path = Path::new("src").join("constants.rs");
    if std::fs::metadata(&constants_file_path).is_ok() {
        std::fs::remove_file(&constants_file_path).unwrap();
    }
    // Create constants.rs file
    let mut constants_file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(&constants_file_path)
        .unwrap();

    // Write simple-graph SQL files as constants
    for entry in glob(temp_simple_graph_dir.join("sql/*.sql").to_str().unwrap()).unwrap() {
        match entry {
            Ok(path) => {
                let filename_screaming_snake_case = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_uppercase()
                    .replace("-", "_");
                let file_contents = std::fs::read_to_string(&path).unwrap();

                // Write a docstring
                constants_file.write(b"\n/// Generated SQL string from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)\n",).unwrap();

                // Then write the constant
                constants_file
                    .write(
                        format!(
                            "pub const {}: &str = r###\"{}\"###;\n",
                            filename_screaming_snake_case, file_contents
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
            Err(e) => println!("{:?}", e),
        }
    }

    // Write simple-graph .template files to constants
    for entry in glob(
        temp_simple_graph_dir
            .join("sql/*.template")
            .to_str()
            .unwrap(),
    )
    .unwrap()
    {
        match entry {
            Ok(path) => {
                let filename_screaming_snake_case = path
                    .file_stem()
                    .unwrap()
                    .to_str()
                    .unwrap()
                    .to_uppercase()
                    .replace("-", "_");
                let file_contents = std::fs::read_to_string(&path).unwrap();

                // Write a docstring
                constants_file.write(b"\n/// Generated Jinja2 template strings that can create SQL function, from the [simple-graph library](https://github.com/dpapathanasiou/simple-graph/tree/main/sql)\n").unwrap();

                // Then write the constant
                constants_file
                    .write(
                        format!(
                            "pub const {}: &str = r###\"{}\"###;\n",
                            filename_screaming_snake_case + "_TEMPLATE",
                            file_contents
                        )
                        .as_bytes(),
                    )
                    .unwrap();
            }
            Err(e) => println!("{:?}", e),
        }
    }

    // Colocate a test *within* our generated file 😎
    constants_file
        .write(
            b"\n\n\
            #[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn imported_constants() {
        assert_eq!(
            DELETE_EDGE,
            \"DELETE FROM edges WHERE source = ? AND target = ?\"
        );
    }
}",
        )
        .unwrap();
}
