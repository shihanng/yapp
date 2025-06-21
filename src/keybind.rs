use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::str::FromStr;
use thiserror::Error;
use zellij_tile::prelude::BareKey;
use zellij_tile::prelude::InputMode;
use zellij_tile::prelude::KeyWithModifier;

pub const LIST_PANES: &str = "list_panes";
pub const NAVIGATE_BACK: &str = "navigate_back";
pub const TOGGLE_STAR: &str = "toggle_star";
pub const PREV_STAR: &str = "previous_star";
pub const NEXT_STAR: &str = "next_star";

const PLUGIN_SELECT_DOWN: &str = "plugin_select_down";
const PLUGIN_SELECT_UP: &str = "plugin_select_up";
const PLUGIN_NAVIGATE_TO: &str = "plugin_navigate_to";
const PLUGIN_HIDE: &str = "plugin_hide";
const PLUGIN_TOGGLE_STAR: &str = "plugin_toggle_star";

pub struct Keybinds {
    list_panes: Option<KeyWithModifier>,
    navigate_back: Option<KeyWithModifier>,
    toggle_star: Option<KeyWithModifier>,
    next_star: Option<KeyWithModifier>,
    previous_star: Option<KeyWithModifier>,

    // These are key bindings while inside the plugin pane.
    pub plugin_select_down: Option<KeyWithModifier>,
    pub plugin_select_up: Option<KeyWithModifier>,
    pub plugin_navigate_to: Option<KeyWithModifier>,
    pub plugin_hide: Option<KeyWithModifier>,
    pub plugin_toggle_star: Option<KeyWithModifier>,
}

impl Default for Keybinds {
    fn default() -> Keybinds {
        Keybinds {
            list_panes: Some(KeyWithModifier::new(BareKey::Char('y')).with_alt_modifier()),
            navigate_back: Some(KeyWithModifier::new(BareKey::Char('o')).with_alt_modifier()),
            toggle_star: Some(KeyWithModifier::new(BareKey::Char('l')).with_alt_modifier()),
            next_star: Some(KeyWithModifier::new(BareKey::Char('i')).with_alt_modifier()),
            previous_star: Some(KeyWithModifier::new(BareKey::Char('u')).with_alt_modifier()),

            plugin_select_down: Some(KeyWithModifier::new(BareKey::Down)),
            plugin_select_up: Some(KeyWithModifier::new(BareKey::Up)),
            plugin_navigate_to: Some(KeyWithModifier::new(BareKey::Enter)),
            plugin_hide: Some(KeyWithModifier::new(BareKey::Esc)),
            plugin_toggle_star: Some(KeyWithModifier::new(BareKey::Char(' '))),
        }
    }
}

impl Keybinds {
    pub fn bind_global_keys<F>(&mut self, base_mode: InputMode, plugin_id: u32, mut configure: F)
    where
        F: FnMut(String, bool),
    {
        let key_actions = [
            (&self.list_panes, LIST_PANES),
            (&self.navigate_back, NAVIGATE_BACK),
            (&self.toggle_star, TOGGLE_STAR),
            (&self.next_star, NEXT_STAR),
            (&self.previous_star, PREV_STAR),
        ];

        for (key, action) in key_actions {
            if let Some(key) = key {
                configure(
                    create_keybind_config(base_mode, plugin_id, key, action),
                    false,
                );
            }
        }
    }
}

#[derive(Error, Debug)]
pub enum KeybindError {
    #[error(transparent)]
    FromStr(#[from] Box<dyn std::error::Error>),
}

impl TryFrom<BTreeMap<String, String>> for Keybinds {
    type Error = KeybindError;
    fn try_from(map: BTreeMap<String, String>) -> Result<Self, Self::Error> {
        let mut keybinds = Keybinds::default();

        let key_mappings = [
            (PLUGIN_SELECT_DOWN, &mut keybinds.plugin_select_down),
            (PLUGIN_SELECT_UP, &mut keybinds.plugin_select_up),
            (PLUGIN_NAVIGATE_TO, &mut keybinds.plugin_navigate_to),
            (PLUGIN_HIDE, &mut keybinds.plugin_hide),
            (PLUGIN_TOGGLE_STAR, &mut keybinds.plugin_toggle_star),
            (LIST_PANES, &mut keybinds.list_panes),
            (NAVIGATE_BACK, &mut keybinds.navigate_back),
            (TOGGLE_STAR, &mut keybinds.toggle_star),
            (PREV_STAR, &mut keybinds.previous_star),
            (NEXT_STAR, &mut keybinds.next_star),
        ];

        for (key_name, keybind_field) in key_mappings {
            if let Some(key_str) = map.get(key_name) {
                if !key_str.is_empty() {
                    *keybind_field = Some(KeyWithModifier::from_str(key_str)?);
                } else {
                    *keybind_field = None;
                }
            }
        }

        Ok(keybinds)
    }
}

pub fn create_keybind_config(
    mode: InputMode,
    plugin_id: u32,
    key: &KeyWithModifier,
    message_plugin_id: &str,
) -> String {
    format!(
        "
        keybinds {{
            {:?} {{
                bind \"{}\" {{
                    MessagePluginId {} {{
                        name \"{}\"
                    }}
                }}
            }}
        }}
        ",
        format!("{:?}", mode).to_lowercase(),
        key,
        plugin_id,
        message_plugin_id
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bind_global_keys() {
        let mut keybinds = Keybinds {
            toggle_star: None,
            ..Default::default()
        };
        let base_mode = InputMode::Normal;
        let plugin_id = 42;
        let mut got_configs = Vec::new();

        keybinds.bind_global_keys(base_mode, plugin_id, |key, _| {
            got_configs.push(key);
        });

        insta::assert_snapshot!(got_configs.join(""));
    }

    #[test]
    fn test_try_from() {
        let map = BTreeMap::from([
            (PLUGIN_SELECT_DOWN.to_string(), String::from("Ctrl Down")),
            (PLUGIN_HIDE.to_string(), String::from("")),
            (String::from("unknown_key"), String::from("Invalid")),
        ]);

        let keybinds = Keybinds::try_from(map).unwrap();

        assert_eq!(
            keybinds.plugin_select_down,
            Some(KeyWithModifier::new(BareKey::Down).with_ctrl_modifier()),
        );
        assert_eq!(
            keybinds.plugin_select_up,
            Some(KeyWithModifier::new(BareKey::Up),)
        );
        assert_eq!(keybinds.plugin_hide, None);
    }
}
