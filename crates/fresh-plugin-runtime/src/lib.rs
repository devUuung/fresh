pub mod backend;
pub mod process;
pub mod thread;
pub mod ts_export;

pub use thread::{PluginConfig, PluginThreadHandle};

/// A plugin whose preparation was done at build time.
///
/// The editor ships ~50 plugins whose sources are fixed at compile time, so
/// bundling, transpiling and emitting their declarations is work with a known
/// answer. `build.rs` runs the pipeline once and emits a table of these; the
/// runtime then does a lookup instead of calling into oxc. Plugins the user
/// installs are not in the table and take the live path, unchanged.
pub struct PreparedPluginEntry {
    /// File name as it appears on disk, e.g. `vi_mode.ts`.
    pub file_name: &'static str,
    /// Length and fingerprint of the source this entry was built from. Both
    /// are checked before the entry is used: a user plugin that happens to
    /// share a file name with a bundled one must not be served the bundled
    /// plugin's code.
    pub source_len: usize,
    pub source_fingerprint: u64,
    pub js_code: &'static str,
    pub declarations: Option<&'static str>,
    pub dependencies: &'static [&'static str],
}

static PREPARED_PLUGINS: std::sync::OnceLock<&'static [PreparedPluginEntry]> =
    std::sync::OnceLock::new();

/// Hand the runtime the build-time table. Called once by the editor during
/// startup; later calls are ignored, so tests that build several editors in
/// one process are fine.
pub fn register_prepared_plugins(table: &'static [PreparedPluginEntry]) {
    let _ = PREPARED_PLUGINS.set(table);
}

/// The build-time entry for `file_name`, if there is one and it was built
/// from exactly this source.
pub(crate) fn lookup_prepared_plugin(
    file_name: &str,
    source: &str,
) -> Option<&'static PreparedPluginEntry> {
    let table = PREPARED_PLUGINS.get()?;
    let entry = table.iter().find(|e| e.file_name == file_name)?;
    (entry.source_len == source.len()
        && entry.source_fingerprint == fresh_parser_js::source_fingerprint(source))
    .then_some(entry)
}
