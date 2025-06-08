#[cfg(not(target_arch = "wasm32"))]
mod shim_native;
mod star;

#[cfg(not(target_arch = "wasm32"))]
pub use shim_native::*;

use std::collections::HashSet;

use zellij_tile::prelude::*;

use std::collections::BTreeMap;
use std::collections::HashMap;

struct Pane {
    tab_name: String,
    pane_id: PaneId,
    pane_title: String,
}

#[derive(Default)]
struct State {
    tab_infos: Vec<TabInfo>,
    pane_infos: HashMap<usize, Vec<PaneInfo>>,

    panes: Vec<Pane>,
    current_focus: Option<PaneId>,
    previous_focus: Option<PaneId>,
    selected: usize,

    stars: star::Star,

    keybinds: Keybinds,

    plugin_id: Option<u32>,
}

const NAVIGATE_BACK: &str = "navigate_back";
const TOGGLE_STAR: &str = "toggle_star";
const PREV_STAR: &str = "previous_star";
const NEXT_STAR: &str = "next_star";

impl State {
    /// Compute the current state of panes that are visible on the plugin list,
    /// currently and previously focus panes.
    /// We have to call this method in both TabUpdate and PaneUpdate event because
    /// the ordering of the events is unditerministic.
    fn update_state(&mut self) {
        let mut panes: Vec<Pane> = Vec::new();
        let mut current_focus = None;

        for (tab_id, tab_info) in self.tab_infos.iter().enumerate() {
            if let Some(pane_infos) = self.pane_infos.get(&tab_id) {
                pane_infos.iter().for_each(|pane_info| {
                    if pane_info.is_plugin && Some(pane_info.id) == self.plugin_id {
                        return;
                    }

                    if pane_info.is_suppressed || !pane_info.is_selectable {
                        return;
                    }

                    let pane_id = if pane_info.is_plugin {
                        PaneId::Plugin(pane_info.id)
                    } else {
                        PaneId::Terminal(pane_info.id)
                    };

                    panes.push(Pane {
                        tab_name: tab_info.name.clone(),
                        pane_id,
                        pane_title: pane_info.title.clone(),
                    });

                    if pane_info.is_focused && tab_info.active && !pane_info.is_plugin {
                        current_focus = Some(pane_id)
                    }
                });
            }
        }

        // Convert panes to hashset of paneid
        let pane_ids: HashSet<PaneId> = panes.iter().map(|p| p.pane_id).collect();

        if current_focus.is_some() && current_focus != self.current_focus {
            if self
                .current_focus
                .as_ref()
                .is_some_and(|c| pane_ids.contains(c))
            {
                // If the previous current_focus still exists, use that as the
                // next previous_focus.
                self.previous_focus = self.current_focus;
            } else if self
                .previous_focus
                .as_ref()
                .is_some_and(|c| pane_ids.contains(c))
            {
                // Not replacing the old previous_focus because the old
                // current_focus was not found in the current panes (could be already deleted)
                // and the old previous_focus still exists.
            } else {
                self.previous_focus = None;
            }
            self.current_focus = current_focus;
        }

        self.stars.sync(&pane_ids);
        self.panes = panes;
    }

    fn panes_as_nested_list(&self) -> Vec<NestedListItem> {
        let mut items = Vec::new();

        for (i, pane) in self.panes.iter().enumerate() {
            let mut item = NestedListItem::new(format!(
                "{} {}{}",
                pane.tab_name,
                pane.pane_title,
                if self.stars.has(&pane.pane_id) {
                    " *"
                } else {
                    ""
                }
            ));

            if i == self.selected {
                item = item.selected();
            }

            items.push(item);
        }
        items
    }

    fn select_downward(&mut self) {
        if !self.panes.is_empty() {
            self.selected = (self.selected + 1) % self.panes.len();
        }
    }

    fn select_upward(&mut self) {
        if !self.panes.is_empty() {
            self.selected = (self.selected + self.panes.len() - 1) % self.panes.len();
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
        self.plugin_id = Some(get_plugin_ids().plugin_id);

        request_permission(&[
            PermissionType::ChangeApplicationState,
            PermissionType::ReadApplicationState,
            PermissionType::Reconfigure,
        ]);

        subscribe(&[
            EventType::ModeUpdate,
            EventType::Key,
            EventType::PaneUpdate,
            EventType::TabUpdate,
        ]);
    }

    fn update(&mut self, event: Event) -> bool {
        match event {
            Event::ModeUpdate(mode_info) => {
                if let Some(base_mode) = mode_info.base_mode {
                    if let Some(plugin_id) = self.plugin_id {
                        self.keybinds.bind(base_mode, plugin_id);
                    }
                }
            }
            Event::TabUpdate(tab_infos) => {
                self.tab_infos = tab_infos;
                self.update_state();
            }
            Event::PaneUpdate(PaneManifest { panes }) => {
                self.pane_infos = panes;
                self.update_state();
            }
            Event::Key(key) => match key.bare_key {
                BareKey::Down if key.has_no_modifiers() => self.select_downward(),
                BareKey::Up if key.has_no_modifiers() => self.select_upward(),
                BareKey::Enter if key.has_no_modifiers() => {
                    focus_pane_with_id(self.panes[self.selected].pane_id, true);
                    hide_self();
                }
                BareKey::Esc if key.has_no_modifiers() => {
                    hide_self();
                }
                BareKey::Char(' ') if key.has_no_modifiers() => {
                    let selected_pane_id = self.panes[self.selected].pane_id;
                    self.stars.toggle(selected_pane_id);
                }
                _ => {}
            },
            _ => {}
        }
        true
    }

    fn pipe(&mut self, pipe_message: PipeMessage) -> bool {
        if pipe_message.source == PipeSource::Keybind && pipe_message.is_private {
            if pipe_message.name == NAVIGATE_BACK {
                if let Some(id) = self.previous_focus {
                    focus_pane_with_id(id, true);
                }
            } else if pipe_message.name == TOGGLE_STAR {
                if let Some(pane_id) = self.current_focus {
                    self.stars.toggle(pane_id);
                }
            } else if pipe_message.name == NEXT_STAR {
                if let Some(pane_id) = self.current_focus {
                    if let Some(id) = self.stars.next(&pane_id) {
                        focus_pane_with_id(*id, true);
                    }
                }
            } else if pipe_message.name == PREV_STAR {
                if let Some(pane_id) = self.current_focus {
                    if let Some(id) = self.stars.previous(&pane_id) {
                        focus_pane_with_id(*id, true);
                    }
                }
            }
            return true;
        }
        false
    }

    fn render(&mut self, _rows: usize, _cols: usize) {
        let nested_list = self.panes_as_nested_list();
        print_nested_list(nested_list);
    }
}

struct Keybinds {
    bound_key: bool,
    navigate_back: KeyWithModifier,
    toggle_star: KeyWithModifier,
    next_star: KeyWithModifier,
    previous_star: KeyWithModifier,
}

impl Default for Keybinds {
    fn default() -> Keybinds {
        Keybinds {
            bound_key: Default::default(),
            navigate_back: KeyWithModifier::new(BareKey::Char('o')).with_alt_modifier(),
            toggle_star: KeyWithModifier::new(BareKey::Char('l')).with_alt_modifier(),
            next_star: KeyWithModifier::new(BareKey::Char('i')).with_alt_modifier(),
            previous_star: KeyWithModifier::new(BareKey::Char('u')).with_alt_modifier(),
        }
    }
}

impl Keybinds {
    pub fn bind(&mut self, base_mode: InputMode, plugin_id: u32) {
        if !self.bound_key {
            bind_key(
                base_mode,
                plugin_id,
                &self.navigate_back,
                &self.toggle_star,
                &self.next_star,
                &self.previous_star,
            );
            self.bound_key = true;
        }
    }
}

pub fn bind_key(
    mode: InputMode,
    plugin_id: u32,
    navigate_back: &KeyWithModifier,
    toggle_star: &KeyWithModifier,
    next_star: &KeyWithModifier,
    previous_star: &KeyWithModifier,
) {
    let new_config = format!(
        "
        keybinds {{
            {:?} {{
                bind \"{}\" {{
                    MessagePluginId {} {{
                        name \"{}\"
                    }}
                }}
                bind \"{}\" {{
                    MessagePluginId {} {{
                        name \"{}\"
                    }}
                }}
                bind \"{}\" {{
                    MessagePluginId {} {{
                        name \"{}\"
                    }}
                }}
                bind \"{}\" {{
                    MessagePluginId {} {{
                        name \"{}\"
                    }}
                }}
            }}
        }}
        ",
        format!("{:?}", mode).to_lowercase(),
        navigate_back,
        plugin_id,
        NAVIGATE_BACK,
        toggle_star,
        plugin_id,
        TOGGLE_STAR,
        next_star,
        plugin_id,
        NEXT_STAR,
        previous_star,
        plugin_id,
        PREV_STAR,
    );
    reconfigure(new_config, false);
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::*;

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
            pane_infos: HashMap::from([
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
                        // Hidden because is the plugin
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
            ]),
            selected: 1,
            plugin_id: Some(4),
            ..Default::default()
        };

        state.update_state();

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
                pane_id: PaneId::Terminal(1),
                tab_name: String::from("Tab"),
            },
            Pane {
                pane_title: String::from("Pane 2"),
                pane_id: PaneId::Terminal(2),
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
        assert_eq!(state.selected, 0);
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
        assert_eq!(state.selected, 1);
    }

    #[fixture]
    fn tab(#[default("Tab")] name: &str) -> TabInfo {
        TabInfo {
            name: String::from(name),
            ..Default::default()
        }
    }

    #[fixture]
    fn active_tab(#[default("Tab")] name: &str) -> TabInfo {
        TabInfo {
            name: String::from(name),
            active: true,
            ..Default::default()
        }
    }

    #[fixture]
    fn pane(#[default(0)] id: u32) -> PaneInfo {
        PaneInfo {
            id,
            is_selectable: true,
            ..Default::default()
        }
    }

    #[fixture]
    fn focus_pane(#[default(0)] id: u32) -> PaneInfo {
        PaneInfo {
            id,
            is_focused: true,
            is_selectable: true,
            ..Default::default()
        }
    }

    #[fixture]
    fn focus_plugin_pane(#[default(0)] id: u32) -> PaneInfo {
        PaneInfo {
            id,
            is_focused: true,
            is_selectable: true,
            is_plugin: true,
            ..Default::default()
        }
    }

    #[fixture]
    fn current_focus() -> PaneId {
        PaneId::Terminal(10)
    }

    #[fixture]
    fn previous_focus() -> PaneId {
        PaneId::Terminal(11)
    }

    #[rstest]
    #[case::no_panes_in_tab(vec![ active_tab("Tab") ], HashMap::from([(1, vec![focus_pane(1)])]))]
    #[case::no_tabs(vec![], HashMap::from([(1, vec![focus_pane(1)])]))]
    #[case::focus_pane_is_plugin(vec![ active_tab("Tab") ], HashMap::from([(0, vec![focus_plugin_pane(1)])]))]
    #[case::focus_pane_is_current(vec![ active_tab("Tab") ], HashMap::from([(0, vec![focus_pane(10)])]))]
    fn no_changes_in_focus(
        #[case] tab_infos: Vec<TabInfo>,
        #[case] pane_infos: HashMap<usize, Vec<PaneInfo>>,
        current_focus: PaneId,
        previous_focus: PaneId,
    ) {
        let mut state = State {
            tab_infos,
            pane_infos,
            current_focus: Some(current_focus),
            previous_focus: Some(previous_focus),
            ..Default::default()
        };

        state.update_state();

        assert_eq!(state.current_focus, Some(current_focus));
        assert_eq!(state.previous_focus, Some(previous_focus));
    }

    #[rstest]
    #[case::current_focus_exists(vec![ active_tab("Tab") ], HashMap::from([(0, vec![focus_pane(1), pane(10)])]), PaneId::Terminal(1), Some(PaneId::Terminal(10)))]
    #[case::current_focus_not_exists_previous_exists(vec![ active_tab("Tab") ], HashMap::from([(0, vec![focus_pane(1), pane(11)])]), PaneId::Terminal(1), Some(PaneId::Terminal(11)))]
    #[case::current_and_previous_focus_not_exists(vec![ active_tab("Tab") ], HashMap::from([(0, vec![focus_pane(1)])]), PaneId::Terminal(1), None)]
    fn change_in_focus(
        #[case] tab_infos: Vec<TabInfo>,
        #[case] pane_infos: HashMap<usize, Vec<PaneInfo>>,
        #[case] new_current_focus: PaneId,
        #[case] new_previous_focus: Option<PaneId>,
    ) {
        let mut state = State {
            tab_infos,
            pane_infos,
            current_focus: Some(PaneId::Terminal(10)),
            previous_focus: Some(PaneId::Terminal(11)),
            ..Default::default()
        };

        state.update_state();

        assert_eq!(state.current_focus, Some(new_current_focus));
        assert_eq!(state.previous_focus, new_previous_focus);
    }
}
