use std::collections::BTreeMap;
use std::convert::TryFrom;
use std::str::FromStr;
use thiserror::Error;
use zellij_tile::prelude::BareKey;
use zellij_tile::prelude::InputMode;
use zellij_tile::prelude::KeyWithModifier;

pub const LIST_PANES: &str = "list_panes";
const NAVIGATE_BACK: &str = "navigate_back";
const TOGGLE_STAR: &str = "toggle_star";
const PREV_STAR: &str = "previous_star";
const NEXT_STAR: &str = "next_star";

const PLUGIN_SELECT_DOWN: &str = "plugin_select_down";
const PLUGIN_SELECT_UP: &str = "plugin_select_up";
const PLUGIN_NAVIGATE_TO: &str = "plugin_navigate_to";
const PLUGIN_HIDE: &str = "plugin_hide";
const PLUGIN_TOGGLE_STAR: &str = "plugin_toggle_star";

pub struct Keybinds {
    list_panes: KeyWithModifier,
    navigate_back: KeyWithModifier,
    toggle_star: KeyWithModifier,
    next_star: KeyWithModifier,
    previous_star: KeyWithModifier,

    // These are key bindings while inside the plugin pane.
    pub plugin_select_down: KeyWithModifier,
    pub plugin_select_up: KeyWithModifier,
    pub plugin_navigate_to: KeyWithModifier,
    pub plugin_hide: KeyWithModifier,
    pub plugin_toggle_star: KeyWithModifier,
}

impl Default for Keybinds {
    fn default() -> Keybinds {
        Keybinds {
            list_panes: KeyWithModifier::new(BareKey::Char('y')).with_alt_modifier(),
            navigate_back: KeyWithModifier::new(BareKey::Char('o')).with_alt_modifier(),
            toggle_star: KeyWithModifier::new(BareKey::Char('l')).with_alt_modifier(),
            next_star: KeyWithModifier::new(BareKey::Char('i')).with_alt_modifier(),
            previous_star: KeyWithModifier::new(BareKey::Char('u')).with_alt_modifier(),

            plugin_select_down: KeyWithModifier::new(BareKey::Down),
            plugin_select_up: KeyWithModifier::new(BareKey::Up),
            plugin_navigate_to: KeyWithModifier::new(BareKey::Enter),
            plugin_hide: KeyWithModifier::new(BareKey::Esc),
            plugin_toggle_star: KeyWithModifier::new(BareKey::Char(' ')),
        }
    }
}

impl Keybinds {
    pub fn bind_global_keys<F>(&mut self, base_mode: InputMode, plugin_id: u32, mut configure: F)
    where
        F: FnMut(String, bool),
    {
        configure(
            create_keybind_config(base_mode, plugin_id, &self.list_panes, LIST_PANES),
            false,
        );
        configure(
            create_keybind_config(base_mode, plugin_id, &self.navigate_back, NAVIGATE_BACK),
            false,
        );
        configure(
            create_keybind_config(base_mode, plugin_id, &self.toggle_star, TOGGLE_STAR),
            false,
        );
        configure(
            create_keybind_config(base_mode, plugin_id, &self.next_star, NEXT_STAR),
            false,
        );
        configure(
            create_keybind_config(base_mode, plugin_id, &self.previous_star, PREV_STAR),
            false,
        );
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

        if let Some(key) = map.get(PLUGIN_SELECT_DOWN) {
            keybinds.plugin_select_down = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(PLUGIN_SELECT_UP) {
            keybinds.plugin_select_up = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(PLUGIN_NAVIGATE_TO) {
            keybinds.plugin_navigate_to = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(PLUGIN_HIDE) {
            keybinds.plugin_hide = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(PLUGIN_TOGGLE_STAR) {
            keybinds.plugin_toggle_star = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(LIST_PANES) {
            keybinds.list_panes = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(NAVIGATE_BACK) {
            keybinds.navigate_back = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(TOGGLE_STAR) {
            keybinds.toggle_star = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(PREV_STAR) {
            keybinds.previous_star = KeyWithModifier::from_str(key)?
        }
        if let Some(key) = map.get(NEXT_STAR) {
            keybinds.next_star = KeyWithModifier::from_str(key)?
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
        let mut keybinds = Keybinds::default();
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
            (String::from("unknown_key"), String::from("Invalid")),
        ]);

        let keybinds = Keybinds::try_from(map).unwrap();

        assert_eq!(
            keybinds.plugin_select_down,
            KeyWithModifier::new(BareKey::Down).with_ctrl_modifier(),
        );
        assert_eq!(keybinds.plugin_select_up, KeyWithModifier::new(BareKey::Up),);
    }
}
