# YAPP

![GitHub branch check runs](https://img.shields.io/github/check-runs/shihanng/yapp/main)
![GitHub Release](https://img.shields.io/github/v/release/shihanng/yapp)
![GitHub License](https://img.shields.io/github/license/shihanng/yapp)

Yet Another (Zellij) Panes Picker.
With YAPP, you can quickly switch, star, and jump to panes
using customizable keyboard shortcuts.

## Installation

Put the following in your
[Zellij configuration](https://zellij.dev/documentation/configuration.html)
`config.kdl`.

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
    yapp location="https://github.com/shihanng/yapp/releases/download/v0.3.0/yapp.wasm"
}
```

## Usage

### Global Keybindings

| Keybinding | Description                                         |
| ---------- | --------------------------------------------------- |
| Alt y      | Open plugin pane and lists all available panes      |
| Alt o      | Toggle between two panes (`navigate_back`)          |
| Alt l      | Star/unstar the focused pane (`toggle_star`)        |
| Alt i      | Navigate to next starred pane (`next_star`)         |
| Alt u      | Navigate to previous starred pane (`previous_star`) |

<!-- markdownlint-disable MD013 -->

### Plugin Keybindings

| Keybinding | Description                                                                       |
| ---------- | --------------------------------------------------------------------------------- |
| Up/Down    | Move the selection in the list of panes (`plugin_select_up`/`plugin_select_down`) |
| Enter      | Navigate to the selected pane (`plugin_navigate_to`)                              |
| Esc        | Close the plugin without navigating to a pane (`plugin_hide`)                     |
| Space      | Toggle star/unstar the selected pane (`plugin_toggle_star`)                       |

### Customize Keybindings

Use the plugin configuration to customize the keybindings, e.g.,
the following allows us to use **Ctrl p/n** to move the selection
in the list of panes.

```kdl
plugins {
    yapp location="https://github.com/shihanng/yapp/releases/download/v0.3.0/yapp.wasm" {
        plugin_select_down "Ctrl n"
        plugin_select_up "Ctrl p"
    }
}
```

<!-- markdownlint-enable MD013 -->

## Development

### Linters and Testing

- Install pre-commit hooks with `pre-commit install`.
- I use [just](https://just.systems/) (command runner) to execute linters
  and run tests but it is optional. See [`justfile`](./justfile) for
  what they actually do.
  - Run linters with `just lint`.
  - Run tests with `just test`.

### (Optional) Run GitHub Actions locally

Use [`act`](https://github.com/nektos/act) to run GitHub Actions locally.

GitHub token in order to avoid hitting the rate limit
when installing the toolings.

```shell
export GITHUB_TOKEN=$(gh auth token)
just run-ci-local
```
