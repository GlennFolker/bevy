use bevy::prelude::*;

fn main() {
    AppBuilder::new()
        .add_defaults()
        .load_plugin(concat!(
            env!("CARGO_MANIFEST_DIR"),
            "/examples/plugin_loading/example_plugin/target/release/libexample_plugin.so"
        ))
        .run();
}
