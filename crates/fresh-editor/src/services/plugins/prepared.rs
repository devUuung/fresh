//! Build-time prepared plugins.
//!
//! `build.rs` runs the plugin preparation pipeline (bundle, strip, transpile,
//! emit declarations) over every plugin shipped with the editor and writes the
//! results here as a static table. Registering it lets the plugin runtime look
//! a plugin up instead of calling into oxc for an answer that was already
//! settled at compile time.
//!
//! Only the bundled plugins are in the table. Anything the user installs
//! misses the lookup and takes the runtime pipeline, unchanged.

include!(concat!(env!("OUT_DIR"), "/prepared_plugins.rs"));

/// Hand the table to the plugin runtime. Idempotent: later calls are ignored,
/// which matters because tests build many editors in one process.
pub fn register() {
    fresh_plugin_runtime::register_prepared_plugins(PREPARED_PLUGINS);
}
