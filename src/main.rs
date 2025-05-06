use zellij_tile::prelude::*;

use std::collections::BTreeMap;

#[derive(Default)]
struct State {
    // the state of the plugin
}

register_plugin!(State);

// NOTE: you can start a development environment inside Zellij by running `zellij -l zellij.kdl` in
// this plugin's folder
//
// More info on plugins: https://zellij.dev/documentation/plugins

impl ZellijPlugin for State {
    fn load(&mut self, _configuration: BTreeMap<String, String>) {
        // runs once on plugin load, provides the configuration with which this plugin was loaded
        // (if any)
        //
        // this is a good place to `subscribe` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.subscribe.html)
        // to `Event`s (https://docs.rs/zellij-tile/latest/zellij_tile/prelude/enum.Event.html)
        // and `request_permissions` (https://docs.rs/zellij-tile/latest/zellij_tile/shim/fn.request_permission.html)
    }
    fn update(&mut self, _event: Event) -> bool {
        // react to `Event`s that have been subscribed to (and the plugin has permissions for)
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        false
    }
    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        // react to data piped to this plugin from the CLI, a keybinding or another plugin
        // read more about pipes: https://zellij.dev/documentation/plugin-pipes
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        false
    }
    fn render(&mut self, rows: usize, cols: usize) {
        println!("Hi there! I have {rows} rows and {cols} columns");
    }
}
