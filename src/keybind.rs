use zellij_tile::prelude::BareKey;
use zellij_tile::prelude::InputMode;
use zellij_tile::prelude::KeyWithModifier;

const NAVIGATE_BACK: &str = "navigate_back";
const TOGGLE_STAR: &str = "toggle_star";
const PREV_STAR: &str = "previous_star";
const NEXT_STAR: &str = "next_star";

pub struct Keybinds {
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
}
