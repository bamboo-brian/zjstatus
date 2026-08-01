use std::collections::BTreeMap;

use crate::{
    config::ZellijState,
    widgets::widget::{Widget, should_hide_when_nested},
};

pub struct SessionWidget {
    nested_show: bool,
}

impl SessionWidget {
    pub fn new(config: &BTreeMap<String, String>) -> Self {
        let nested_show = config
            .get("session_nested_show")
            .map(|v| v == "true")
            .unwrap_or(true);

        Self { nested_show }
    }
}

impl Widget for SessionWidget {
    fn process(&self, _name: &str, state: &ZellijState) -> String {
        if should_hide_when_nested(self.nested_show, &state.mode) {
            return "".to_owned();
        }

        match &state.mode.session_name {
            Some(name) => name.to_owned(),
            None => "".to_owned(),
        }
    }

    fn process_click(&self, _name: &str, _state: &ZellijState, _pos: usize) {}
}

#[cfg(test)]
mod test {
    use zellij_tile::prelude::ModeInfo;

    use super::*;

    fn nested_state() -> ZellijState {
        ZellijState {
            mode: ModeInfo {
                session_name: Some("my-session".to_owned()),
                session_dimmed: Some(true),
                ..ModeInfo::default()
            },
            ..ZellijState::default()
        }
    }

    #[test]
    fn test_hidden_when_nested_and_nested_show_false() {
        let config = BTreeMap::from([("session_nested_show".to_owned(), "false".to_owned())]);
        let widget = SessionWidget::new(&config);

        assert_eq!(widget.process("session", &nested_state()), "");
    }

    #[test]
    fn test_shown_by_default_when_nested() {
        let widget = SessionWidget::new(&BTreeMap::new());

        assert_eq!(widget.process("session", &nested_state()), "my-session");
    }
}
