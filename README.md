# YAPP

Yet Another (Zellij) Panes Picker

## Installation

You have to build this plugin before proceeding with the setup.

```shell
git clone https://github.com/shihanng/yapp.git
cd yapp
cargo build --release
```

```kdl
shared_except "locked" {
    bind "Alt y" {
        LaunchOrFocusPlugin "file:/<PATH_TO>/yapp.wasm" {
            floating true; move_to_focused_tab true;
        }
    }
}
```

## Usage

**Alt Y** to open plugin pane and list all panes.

### In the plugin pane

- **Up/Down** to move the selection in the list of panes.
- **Enter** to navigate to the selected pane.
- **Esc** to close the plugin without navigating to a pane.

## Development

### Pre-commit and Testing

- Install pre-commit hooks with `pre-commit install`.
- Run tests with `just test`.

### With the Provided Layout

![img-2024-11-14-100111](https://github.com/user-attachments/assets/e3bae15c-1f94-4d4a-acea-a036f8afdf67)

Run `zellij -l zellij.kdl` at the root of this repository.
This will open a development environment that
will help you develop the plugin inside Zellij.

It can also be used if you prefer developing outside
of the terminal - in this case you should
ignore the `$EDITOR` pane and use your IDE instead.

### Otherwise

1. Build the project: `cargo build`
2. Load it inside a running Zellij session:
   `zellij action start-or-reload-plugin file:target/wasm32-wasi/debug/rust-plugin-example.wasm`
3. Repeat on changes (perhaps with a `watchexec` or
   similar command to run on fs changes).
