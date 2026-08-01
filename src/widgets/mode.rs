use std::collections::BTreeMap;

use zellij_tile::prelude::InputMode;

use crate::{config::ZellijState, render::FormattedPart};

use super::widget::{Widget, should_hide_when_nested};

#[derive(Debug)]
pub struct ModeWidget {
    normal_format: Vec<FormattedPart>,
    locked_format: Vec<FormattedPart>,
    resize_format: Vec<FormattedPart>,
    pane_format: Vec<FormattedPart>,
    tab_format: Vec<FormattedPart>,
    scroll_format: Vec<FormattedPart>,
    enter_search_format: Vec<FormattedPart>,
    search_format: Vec<FormattedPart>,
    rename_tab_format: Vec<FormattedPart>,
    rename_pane_format: Vec<FormattedPart>,
    session_format: Vec<FormattedPart>,
    move_format: Vec<FormattedPart>,
    prompt_format: Vec<FormattedPart>,
    tmux_format: Vec<FormattedPart>,
    default_to_mode: Option<String>,
    nested_ascended_format: Vec<FormattedPart>,
    nested_dimmed_format: Vec<FormattedPart>,
    nested_active_format: Vec<FormattedPart>,
    nested_show: bool,
}

impl ModeWidget {
    pub fn new(config: &BTreeMap<String, String>) -> Self {
        let normal_format = match config.get("mode_normal") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let locked_format = match config.get("mode_locked") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let resize_format = match config.get("mode_resize") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let pane_format = match config.get("mode_pane") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let tab_format = match config.get("mode_tab") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let scroll_format = match config.get("mode_scroll") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let enter_search_format = match config.get("mode_enter_search") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let search_format = match config.get("mode_search") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let rename_tab_format = match config.get("mode_rename_tab") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let rename_pane_format = match config.get("mode_rename_pane") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let session_format = match config.get("mode_session") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let move_format = match config.get("mode_move") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let prompt_format = match config.get("mode_prompt") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let tmux_format = match config.get("mode_tmux") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let default_to_mode = config.get("mode_default_to_mode").map(|s| s.to_string());

        let nested_ascended_format = match config.get("mode_nested_ascended") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let nested_dimmed_format = match config.get("mode_nested_dimmed") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let nested_active_format = match config.get("mode_nested_active") {
            Some(form) => FormattedPart::multiple_from_format_string(form, config),
            None => vec![],
        };

        let nested_show = config
            .get("mode_nested_show")
            .map(|v| v == "true")
            .unwrap_or(true);

        Self {
            normal_format,
            locked_format,
            resize_format,
            pane_format,
            tab_format,
            scroll_format,
            enter_search_format,
            search_format,
            rename_tab_format,
            rename_pane_format,
            session_format,
            move_format,
            prompt_format,
            tmux_format,
            default_to_mode,
            nested_ascended_format,
            nested_dimmed_format,
            nested_active_format,
            nested_show,
        }
    }

    fn nested_format_for(&self, state: &ZellijState) -> Option<&Vec<FormattedPart>> {
        if state.mode.session_ascended.unwrap_or(false) && !self.nested_ascended_format.is_empty()
        {
            Some(&self.nested_ascended_format)
        } else if state.mode.session_dimmed.unwrap_or(false)
            && !self.nested_dimmed_format.is_empty()
        {
            Some(&self.nested_dimmed_format)
        } else if !state.mode.session_ancestry.is_empty() && !self.nested_active_format.is_empty()
        {
            Some(&self.nested_active_format)
        } else {
            None
        }
    }

    fn render_regular_format(&self, state: &ZellijState, mode_name: &str) -> String {
        self.select_format(state.mode.mode)
            .iter()
            .map(|f| {
                let content = if f.content.contains("{name}") {
                    f.content.replace("{name}", mode_name)
                } else {
                    f.content.clone()
                };

                (f, content)
            })
            .fold("".to_owned(), |acc, (f, content)| {
                format!("{acc}{}", f.format_string(&content))
            })
    }
}

impl Widget for ModeWidget {
    fn process(&self, _name: &str, state: &ZellijState) -> String {
        if should_hide_when_nested(self.nested_show, &state.mode) {
            return "".to_owned();
        }

        let mode_name = format!("{:?}", state.mode.mode);
        let regular_format = self.render_regular_format(state, &mode_name);

        let Some(format) = self.nested_format_for(state) else {
            return regular_format;
        };

        format.iter().fold("".to_owned(), |acc, f| {
            let mut content = f.content.clone();

            if content.contains("{name}") {
                content = content.replace("{name}", &mode_name);
            }

            if content.contains("{mode}") {
                content = content.replace("{mode}", &regular_format);
            }

            format!("{acc}{}", f.format_string(&content))
        })
    }

    fn process_click(&self, _name: &str, _state: &ZellijState, _pos: usize) {}
}

impl ModeWidget {
    fn get_format_by_mode(&self, mode: InputMode) -> &Vec<FormattedPart> {
        match mode {
            InputMode::Normal => &self.normal_format,
            InputMode::Locked => &self.locked_format,
            InputMode::Resize => &self.resize_format,
            InputMode::Pane => &self.pane_format,
            InputMode::Tab => &self.tab_format,
            InputMode::Scroll => &self.scroll_format,
            InputMode::EnterSearch => &self.enter_search_format,
            InputMode::Search => &self.search_format,
            InputMode::RenameTab => &self.rename_tab_format,
            InputMode::RenamePane => &self.rename_pane_format,
            InputMode::Session => &self.session_format,
            InputMode::Move => &self.move_format,
            InputMode::Prompt => &self.prompt_format,
            InputMode::Tmux => &self.tmux_format,
        }
    }

    fn select_format(&self, mode: InputMode) -> &Vec<FormattedPart> {
        let output = self.get_format_by_mode(mode);

        if output.is_empty() {
            return match self.default_to_mode {
                Some(ref mode) => match map_string_to_mode(mode) {
                    Some(mode) => {
                        let out = self.get_format_by_mode(mode);

                        if out.is_empty() {
                            return &self.normal_format;
                        }

                        return out;
                    }
                    None => &self.normal_format,
                },
                None => &self.normal_format,
            };
        }

        output
    }
}

fn map_string_to_mode(s: &str) -> Option<InputMode> {
    match s {
        "normal" => Some(InputMode::Normal),
        "locked" => Some(InputMode::Locked),
        "resize" => Some(InputMode::Resize),
        "pane" => Some(InputMode::Pane),
        "tab" => Some(InputMode::Tab),
        "scroll" => Some(InputMode::Scroll),
        "enter_search" => Some(InputMode::EnterSearch),
        "search" => Some(InputMode::Search),
        "rename_tab" => Some(InputMode::RenameTab),
        "rename_pane" => Some(InputMode::RenamePane),
        "session" => Some(InputMode::Session),
        "move" => Some(InputMode::Move),
        "prompt" => Some(InputMode::Prompt),
        "tmux" => Some(InputMode::Tmux),
        _ => None,
    }
}

#[cfg(test)]
mod test {
    use std::collections::BTreeMap;

    use zellij_tile::prelude::ModeInfo;

    use crate::{config::ZellijState, widgets::widget::Widget};

    use super::ModeWidget;

    fn state_with_mode(mode: ModeInfo) -> ZellijState {
        ZellijState {
            mode,
            ..ZellijState::default()
        }
    }

    #[test]
    pub fn test_regular_format_used_when_not_nested() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_ascended".to_owned(), "(bg) {mode}".to_owned()),
            ("mode_nested_dimmed".to_owned(), "(dim) {mode}".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo::default());

        assert_eq!(widget.process("mode", &state), "Normal");
    }

    #[test]
    pub fn test_nested_ascended_format_replaces_regular_format() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_ascended".to_owned(), "(bg) {mode}".to_owned()),
            ("mode_nested_dimmed".to_owned(), "(dim) {mode}".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_ascended: Some(true),
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "(bg) Normal");
    }

    #[test]
    pub fn test_nested_dimmed_format_replaces_regular_format() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_ascended".to_owned(), "(bg) {mode}".to_owned()),
            ("mode_nested_dimmed".to_owned(), "(dim) {mode}".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "(dim) Normal");
    }

    #[test]
    pub fn test_nested_ascended_takes_precedence_over_dimmed() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_ascended".to_owned(), "bg".to_owned()),
            ("mode_nested_dimmed".to_owned(), "dim".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_ascended: Some(true),
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "bg");
    }

    #[test]
    pub fn test_nested_falls_back_to_regular_format_when_unconfigured() {
        let config = BTreeMap::from([("mode_normal".to_owned(), "{name}".to_owned())]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "Normal");
    }

    #[test]
    pub fn test_nested_name_placeholder_takes_wrapper_styling() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "#[fg=1]{name}".to_owned()),
            (
                "mode_nested_dimmed".to_owned(),
                "#[fg=3,bold]{name}".to_owned(),
            ),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        let output = widget.process("mode", &state);
        let regular_output = widget.render_regular_format(&state, "Normal");

        assert!(output.contains("Normal"));
        assert_ne!(
            output, regular_output,
            "{{name}} should be styled by the nested wrapper, not by mode_normal"
        );
    }

    #[test]
    pub fn test_nested_mode_placeholder_embeds_whole_regular_format() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "#[fg=1]{name}".to_owned()),
            (
                "mode_nested_dimmed".to_owned(),
                "(dim) {mode}".to_owned(),
            ),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        let output = widget.process("mode", &state);
        let regular_output = widget.render_regular_format(&state, "Normal");

        assert_eq!(output, format!("(dim) {regular_output}"));
    }

    #[test]
    pub fn test_nested_format_can_use_both_name_and_mode_placeholders() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "#[fg=1]{name}".to_owned()),
            (
                "mode_nested_dimmed".to_owned(),
                "{name}: {mode}".to_owned(),
            ),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        let output = widget.process("mode", &state);
        let regular_output = widget.render_regular_format(&state, "Normal");

        assert_eq!(output, format!("Normal: {regular_output}"));
    }

    #[test]
    pub fn test_nested_active_used_when_ancestry_present_and_not_ascended_or_dimmed() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_active".to_owned(), "(active) {mode}".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_ancestry: vec!["host-session".to_owned()],
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "(active) Normal");
    }

    #[test]
    pub fn test_nested_active_not_used_without_ancestry() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_active".to_owned(), "(active) {mode}".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo::default());

        assert_eq!(widget.process("mode", &state), "Normal");
    }

    #[test]
    pub fn test_nested_ascended_and_dimmed_take_precedence_over_active() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_ascended".to_owned(), "bg".to_owned()),
            ("mode_nested_active".to_owned(), "active".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_ascended: Some(true),
            session_ancestry: vec!["host-session".to_owned()],
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "bg");
    }

    #[test]
    pub fn test_hidden_when_nested_and_nested_show_false() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_show".to_owned(), "false".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "");
    }

    #[test]
    pub fn test_shown_when_host_fullscreen_despite_nested_show_false() {
        let config = BTreeMap::from([
            ("mode_normal".to_owned(), "{name}".to_owned()),
            ("mode_nested_show".to_owned(), "false".to_owned()),
        ]);
        let widget = ModeWidget::new(&config);

        let state = state_with_mode(ModeInfo {
            session_dimmed: Some(true),
            host_fullscreen: Some(true),
            ..ModeInfo::default()
        });

        assert_eq!(widget.process("mode", &state), "Normal");
    }
}
