use std::collections::BTreeMap;

use crate::render::FormattedPart;
use crate::{
    config::ZellijState,
    widgets::widget::{Widget, should_hide_when_nested},
};

pub struct AncestryWidget {
    format: Vec<FormattedPart>,
    separator: Option<FormattedPart>,
    nested_show: bool,
}

impl AncestryWidget {
    pub fn new(config: &BTreeMap<String, String>) -> Self {
        let format = FormattedPart::multiple_from_format_string(
            config
                .get("ancestry_format")
                .map(|s| s.as_str())
                .unwrap_or("{name}"),
            config,
        );

        let separator = config
            .get("ancestry_separator")
            .map(|s| FormattedPart::from_format_string(s, config));

        let nested_show = config
            .get("ancestry_nested_show")
            .map(|v| v == "true")
            .unwrap_or(true);

        Self {
            format,
            separator,
            nested_show,
        }
    }
}

impl Widget for AncestryWidget {
    fn process(&self, _name: &str, state: &ZellijState) -> String {
        if should_hide_when_nested(self.nested_show, &state.mode) {
            return "".to_owned();
        }

        let ancestry = &state.mode.session_ancestry;

        ancestry
            .iter()
            .enumerate()
            .fold("".to_owned(), |acc, (i, name)| {
                let rendered = self.format.iter().fold("".to_owned(), |acc, f| {
                    let content = if f.content.contains("{name}") {
                        f.content.replace("{name}", name)
                    } else {
                        f.content.clone()
                    };

                    format!("{acc}{}", f.format_string(&content))
                });

                let separator = match (i + 1 < ancestry.len(), &self.separator) {
                    (true, Some(sep)) => sep.format_string(&sep.content),
                    _ => "".to_owned(),
                };

                format!("{acc}{rendered}{separator}")
            })
    }

    fn process_click(&self, _name: &str, _state: &ZellijState, _pos: usize) {}
}

#[cfg(test)]
mod test {
    use zellij_tile::prelude::ModeInfo;

    use super::*;

    fn state_with_ancestry(ancestry: Vec<&str>) -> ZellijState {
        ZellijState {
            mode: ModeInfo {
                session_ancestry: ancestry.into_iter().map(|s| s.to_owned()).collect(),
                ..ModeInfo::default()
            },
            ..ZellijState::default()
        }
    }

    #[test]
    fn test_empty_when_not_nested() {
        let widget = AncestryWidget::new(&BTreeMap::new());

        let state = state_with_ancestry(vec![]);

        assert_eq!(widget.process("ancestry", &state), "");
    }

    #[test]
    fn test_defaults_to_plain_name_per_item() {
        let widget = AncestryWidget::new(&BTreeMap::new());

        let state = state_with_ancestry(vec!["host", "middle"]);

        assert_eq!(widget.process("ancestry", &state), "hostmiddle");
    }

    #[test]
    fn test_ancestry_format_applies_to_each_item() {
        let config = BTreeMap::from([("ancestry_format".to_owned(), "{name} > ".to_owned())]);
        let widget = AncestryWidget::new(&config);

        let state = state_with_ancestry(vec!["host", "middle"]);

        assert_eq!(widget.process("ancestry", &state), "host > middle > ");
    }

    #[test]
    fn test_ancestry_format_styling_applies_per_item() {
        let config = BTreeMap::from([("ancestry_format".to_owned(), "#[fg=3]{name}".to_owned())]);
        let widget = AncestryWidget::new(&config);

        let state = state_with_ancestry(vec!["host"]);

        let output = widget.process("ancestry", &state);

        assert!(output.contains("host"));
        assert_ne!(output, "host", "expected ANSI styling to wrap {{name}}");
    }

    #[test]
    fn test_separator_rendered_between_items_only() {
        let config = BTreeMap::from([("ancestry_separator".to_owned(), " > ".to_owned())]);
        let widget = AncestryWidget::new(&config);

        let state = state_with_ancestry(vec!["host", "middle", "guest"]);

        assert_eq!(
            widget.process("ancestry", &state),
            "host > middle > guest"
        );
    }

    #[test]
    fn test_no_separator_for_single_item() {
        let config = BTreeMap::from([("ancestry_separator".to_owned(), " > ".to_owned())]);
        let widget = AncestryWidget::new(&config);

        let state = state_with_ancestry(vec!["host"]);

        assert_eq!(widget.process("ancestry", &state), "host");
    }

    #[test]
    fn test_no_separator_when_unconfigured() {
        let widget = AncestryWidget::new(&BTreeMap::new());

        let state = state_with_ancestry(vec!["host", "middle"]);

        assert_eq!(widget.process("ancestry", &state), "hostmiddle");
    }

    #[test]
    fn test_hidden_when_nested_and_nested_show_false() {
        let config = BTreeMap::from([("ancestry_nested_show".to_owned(), "false".to_owned())]);
        let widget = AncestryWidget::new(&config);

        let state = state_with_ancestry(vec!["host"]);

        assert_eq!(widget.process("ancestry", &state), "");
    }

    #[test]
    fn test_shown_by_default_when_nested() {
        let widget = AncestryWidget::new(&BTreeMap::new());

        let state = state_with_ancestry(vec!["host"]);

        assert_eq!(widget.process("ancestry", &state), "host");
    }

    #[test]
    fn test_shown_when_host_fullscreen_despite_nested_show_false() {
        let config = BTreeMap::from([("ancestry_nested_show".to_owned(), "false".to_owned())]);
        let widget = AncestryWidget::new(&config);

        let mut state = state_with_ancestry(vec!["host"]);
        state.mode.host_fullscreen = Some(true);

        assert_eq!(widget.process("ancestry", &state), "host");
    }
}
