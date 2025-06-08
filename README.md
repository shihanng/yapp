# YAPP

Yet Another (Zellij) Panes Picker

## Installation

Put the following in your Zellij's configuration.
Replace `<VERSION>` with the appropriate version from the release page, e.g., `v0.1.0`.

```kdl
shared_except "locked" {
    bind "Alt y" {
        LaunchOrFocusPlugin "yapp" {
            floating true; move_to_focused_tab true;
        }
    }
}

load_plugins {
    yapp
}

plugins {
    yapp location="https://github.com/shihanng/yapp/releases/download/<VERSION>/yapp.wasm"
}
```

## Usage

- **Alt Y** to open plugin pane and list all panes.
- **Alt O** to toggle between two panes.
- **Alt l** to toggle star/unstar the focused pane.
- **Alt i** navigate to next starred pane.
- **Alt u** navigate to previous starred pane.

### In the plugin pane

- **Up/Down** to move the selection in the list of panes.
- **Enter** to navigate to the selected pane.
- **Esc** to close the plugin without navigating to a pane.
- **Space** to toggle star/unstar the selected pane.

## Development

### Pre-commit and Testing

- Install pre-commit hooks with `pre-commit install`.
- Run linters with `just lint`.
- Run tests with `just test`.

### Run GitHub Actions locally

Use [`act`](https://github.com/nektos/act) to run GitHub Actions locally.

We use GitHub token in order to avoid hitting the rate limit
when installing the toolings.

```shell
export GITHUB_TOKEN=$(gh auth token)
just run-ci-local
```

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
