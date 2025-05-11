use zellij_tile::prelude::*;

use std::collections::BTreeMap;
use std::collections::HashMap;

#[derive(Default)]
struct Pane {
    tab_name: String,
    pane_title: String,
}

#[derive(Default)]
struct State {
    tab_infos: Vec<TabInfo>,
    panes: Vec<Pane>,
    selected: usize,
}

impl State {
    fn save_panes(&mut self, pane_infos: HashMap<usize, Vec<PaneInfo>>) {
        let mut panes: Vec<Pane> = Vec::new();

        for (tab_id, tab_info) in self.tab_infos.iter().enumerate() {
            if let Some(pane_infos) = pane_infos.get(&tab_id) {
                pane_infos.iter().for_each(|pane_info| {
                    if !pane_info.is_plugin && !pane_info.is_suppressed && pane_info.is_selectable {
                        panes.push(Pane {
                            tab_name: tab_info.name.clone(),
                            pane_title: pane_info.title.clone(),
                        });
                    }
                });
            }
        }

        self.panes = panes;
    }

    fn panes_as_nested_list(&self) -> Vec<NestedListItem> {
        let mut items = Vec::new();

        for (i, pane) in self.panes.iter().enumerate() {
            let mut item = NestedListItem::new(format!("{} {}", pane.tab_name, pane.pane_title));

            if i == self.selected {
                item = item.selected();
            }

            items.push(item);
        }
        items
    }

    fn select_downward(&mut self) {
        if self.selected + 1 < self.panes.len() {
            self.selected += 1;
        }
    }

    fn select_upward(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
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
        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
        ]);

        subscribe(&[EventType::Key, EventType::PaneUpdate, EventType::TabUpdate]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::TabUpdate(tab_infos) => {
                self.tab_infos = tab_infos;
            }
            Event::PaneUpdate(PaneManifest { panes }) => {
                self.save_panes(panes);
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Down if key.has_no_modifiers() => self.select_downward(),
                BareKey::Up if key.has_no_modifiers() => self.select_upward(),
                BareKey::Enter if key.has_no_modifiers() => {
                    focus_pane_with_id(PaneId::Terminal(self.selected as u32), true);
                    hide_self();
                }
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn pipe(&mut self, _pipe_message: PipeMessage) -> bool {
        // react to data piped to this plugin from the CLI, a keybinding or another plugin
        // read more about pipes: https://zellij.dev/documentation/plugin-pipes
        // return true if this plugin's `render` function should be called for the plugin to render
        // itself
        false
    }
    fn render(&mut self, _rows: usize, _cols: usize) {
        let nested_list = self.panes_as_nested_list();
        print_nested_list(nested_list);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn panes_as_nested_list() {
        let mut state = State {
            tab_infos: vec![
                TabInfo {
                    name: String::from("Tab 1"),
                    ..Default::default()
                },
                TabInfo {
                    name: String::from("Tab 2"),
                    ..Default::default()
                },
            ],
            selected: 1,
            ..Default::default()
        };

        let panes = HashMap::from([
            (
                // This will be the last because it is the last tab.
                1,
                vec![PaneInfo {
                    id: 55,
                    title: String::from("Pane 55"),
                    is_selectable: true,
                    ..Default::default()
                }],
            ),
            (
                0,
                vec![
                    PaneInfo {
                        id: 1,
                        title: String::from("Pane 1"),
                        is_selectable: true,
                        ..Default::default()
                    },
                    PaneInfo {
                        id: 2,
                        title: String::from("Pane 2"),
                        is_selectable: true,
                        ..Default::default()
                    },
                    // Hidden because is not selectable
                    PaneInfo {
                        id: 3,
                        title: String::from("Pane 3 (not selectable)"),
                        ..Default::default()
                    },
                    // Hidden because is suppressed
                    PaneInfo {
                        id: 4,
                        title: String::from("Pane 4 (suppressed)"),
                        is_selectable: true,
                        is_suppressed: true,
                        ..Default::default()
                    },
                    // Hidden because is a plugin
                    PaneInfo {
                        id: 4,
                        title: String::from("Pane 4 (suppressed)"),
                        is_selectable: true,
                        is_plugin: true,
                        ..Default::default()
                    },
                ],
            ),
            (
                // The following tab does not exist.
                2,
                vec![PaneInfo {
                    id: 99,
                    title: String::from("Pane 99 on non existing tab"),
                    is_selectable: true,
                    ..Default::default()
                }],
            ),
        ]);

        state.save_panes(panes);

        let items = state.panes_as_nested_list();

        assert_eq!(items.len(), 3);
        assert_eq!(items[0].serialize(), Text::new("Tab 1 Pane 1").serialize());
        assert_eq!(
            items[1].serialize(),
            Text::new("Tab 1 Pane 2").selected().serialize()
        );
        assert_eq!(items[2].serialize(), Text::new("Tab 2 Pane 55").serialize());
    }

    #[test]
    fn select_downward_without_panes() {
        let mut state = State::default();
        state.select_downward();
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn select_upward_without_panes() {
        let mut state = State::default();
        state.select_upward();
        assert_eq!(state.selected, 0);
    }

    fn setup_panes() -> Vec<Pane> {
        vec![
            Pane {
                pane_title: String::from("Pane 1"),
                tab_name: String::from("Tab"),
            },
            Pane {
                pane_title: String::from("Pane 2"),
                tab_name: String::from("Tab"),
            },
        ]
    }

    #[test]
    fn select_downward() {
        let mut state = State {
            panes: setup_panes(),
            selected: 0,
            ..Default::default()
        };

        state.select_downward();
        assert_eq!(state.selected, 1);
    }

    #[test]
    fn select_upward() {
        let mut state = State {
            panes: setup_panes(),
            selected: 1,
            ..Default::default()
        };

        state.select_upward();
        assert_eq!(state.selected, 0);
    }

    #[test]
    fn select_downward_overflow() {
        let mut state = State {
            panes: setup_panes(),
            selected: 0,
            ..Default::default()
        };

        state.select_downward();
        state.select_downward();
        assert_eq!(state.selected, 1);
    }

    #[test]
    fn select_upward_overflow() {
        let mut state = State {
            panes: setup_panes(),
            selected: 1,
            ..Default::default()
        };

        state.select_upward();
        state.select_upward();
        assert_eq!(state.selected, 0);
    }
}
