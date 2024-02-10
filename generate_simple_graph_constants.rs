use fetch_unroll::Fetch;
use std::path::PathBuf;

const SIMPLE_GRAPH_REMOTE_URL_BASE: &str =
    "https://github.com/dpapathanasiou/simple-graph/archive/refs/tags/";
const SIMPLE_GRAPH_DEFAULT_VERSION: &str = "2.1.0";
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
        .to(temp_simple_graph_dir)
        .unwrap();
}
