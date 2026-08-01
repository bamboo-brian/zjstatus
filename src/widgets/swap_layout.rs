use std::collections::BTreeMap;
use zellij_tile::shim::next_swap_layout;

use crate::render::FormattedPart;
use crate::{
    config::ZellijState,
    widgets::widget::{Widget, should_hide_when_nested},
};

pub struct SwapLayoutWidget {
    format: Vec<FormattedPart>,
    hide_if_empty: bool,
    nested_show: bool,
}

impl SwapLayoutWidget {
    pub fn new(config: &BTreeMap<String, String>) -> Self {
        let mut format: Vec<FormattedPart> = Vec::new();
        if let Some(form) = config.get("swap_layout_format") {
            format = FormattedPart::multiple_from_format_string(form, config);
        }

        let hide_if_empty = match config.get("swap_layout_hide_if_empty") {
            Some(hide_if_empty) => hide_if_empty == "true",
            None => false,
        };

        let nested_show = config
            .get("swap_layout_nested_show")
            .map(|v| v == "true")
            .unwrap_or(true);

        Self {
            format,
            hide_if_empty,
            nested_show,
        }
    }
}

impl Widget for SwapLayoutWidget {
    fn process(&self, _name: &str, state: &ZellijState) -> String {
        if should_hide_when_nested(self.nested_show, &state.mode) {
            return "".to_owned();
        }

        let active_tab = state.tabs.iter().find(|t| t.active);

        if active_tab.is_none() {
            return "".to_owned();
        }

        let active_tab = active_tab.unwrap();

        let name = match active_tab.active_swap_layout_name.clone() {
            Some(n) => n,
            None => "".to_owned(),
        };

        if name.is_empty() && self.hide_if_empty {
            return "".to_owned();
        }

        if self.format.is_empty() {
            return name;
        }

        let mut output = "".to_owned();

        for f in &self.format {
            let mut content = f.content.clone();

            if content.contains("{name}") {
                content = content.replace("{name}", &name);
            }

            output = format!("{}{}", output, f.format_string(&content));
        }

        output
    }

    fn process_click(&self, _name: &str, _state: &ZellijState, _pos: usize) {
        next_swap_layout()
    }
}

#[cfg(test)]
mod test {
    use zellij_tile::prelude::{ModeInfo, TabInfo};

    use super::*;

    fn nested_state() -> ZellijState {
        ZellijState {
            mode: ModeInfo {
                session_dimmed: Some(true),
                ..ModeInfo::default()
            },
            tabs: vec![TabInfo {
                active: true,
                active_swap_layout_name: Some("compact".to_owned()),
                ..TabInfo::default()
            }],
            ..ZellijState::default()
        }
    }

    #[test]
    fn test_hidden_when_nested_and_nested_show_false() {
        let config = BTreeMap::from([("swap_layout_nested_show".to_owned(), "false".to_owned())]);
        let widget = SwapLayoutWidget::new(&config);

        assert_eq!(widget.process("swap_layout", &nested_state()), "");
    }

    #[test]
    fn test_shown_by_default_when_nested() {
        let widget = SwapLayoutWidget::new(&BTreeMap::new());

        assert_eq!(widget.process("swap_layout", &nested_state()), "compact");
    }
}
