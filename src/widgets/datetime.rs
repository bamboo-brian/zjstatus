use std::{collections::BTreeMap, str::FromStr};

use chrono::Local;
use chrono_tz::Tz;

use crate::render::FormattedPart;

use crate::{
    config::ZellijState,
    widgets::widget::{Widget, should_hide_when_nested},
};

pub struct DateTimeWidget {
    format: String,
    time_format: String,
    date_format: String,
    color_format: Vec<FormattedPart>,
    time_zone: Option<Tz>,
    nested_show: bool,
}

impl DateTimeWidget {
    pub fn new(config: &BTreeMap<String, String>) -> Self {
        let mut format = "%H:%M";
        if let Some(form) = config.get("datetime_format") {
            format = form;
        }

        let mut time_format = "%H:%M";
        if let Some(form) = config.get("datetime_time_format") {
            time_format = form;
        }

        let mut date_format = "%Y-%m-%d";
        if let Some(form) = config.get("datetime_date_format") {
            date_format = form;
        }

        let mut time_zone_string = "Etc/UTC";
        if let Some(tz_string) = config.get("datetime_timezone") {
            time_zone_string = tz_string;
        }

        let time_zone = Tz::from_str(time_zone_string).ok();

        let mut color_format = "";
        if let Some(form) = config.get("datetime") {
            color_format = form;
        }

        let nested_show = config
            .get("datetime_nested_show")
            .map(|v| v == "true")
            .unwrap_or(true);

        Self {
            format: format.to_owned(),
            date_format: date_format.to_owned(),
            time_format: time_format.to_owned(),
            color_format: FormattedPart::multiple_from_format_string(color_format, config),
            time_zone,
            nested_show,
        }
    }
}

impl Widget for DateTimeWidget {
    fn process(&self, _name: &str, state: &ZellijState) -> String {
        if should_hide_when_nested(self.nested_show, &state.mode) {
            return "".to_owned();
        }

        let date = Local::now();

        let mut tz = Tz::UTC;
        if let Some(t) = self.time_zone {
            tz = t;
        }

        self.color_format
            .iter()
            .map(|f| {
                let mut content = f.content.clone();

                if content.contains("{format}") {
                    content = content.replace(
                        "{format}",
                        format!("{}", date.with_timezone(&tz).format(self.format.as_str()))
                            .as_str(),
                    );
                }

                if content.contains("{date}") {
                    content = content.replace(
                        "{date}",
                        format!(
                            "{}",
                            date.with_timezone(&tz).format(self.date_format.as_str())
                        )
                        .as_str(),
                    );
                }

                if content.contains("{time}") {
                    content = content.replace(
                        "{time}",
                        format!(
                            "{}",
                            date.with_timezone(&tz).format(self.time_format.as_str())
                        )
                        .as_str(),
                    );
                }

                (f, content)
            })
            .fold("".to_owned(), |acc, (f, content)| {
                format!("{acc}{}", f.format_string(&content))
            })
    }

    fn process_click(&self, _name: &str, _state: &ZellijState, _pos: usize) {}
}

#[cfg(test)]
mod test {
    use zellij_tile::prelude::ModeInfo;

    use super::*;

    #[test]
    fn test_hidden_when_nested_and_nested_show_false() {
        let config = BTreeMap::from([
            ("datetime".to_owned(), "{time}".to_owned()),
            ("datetime_nested_show".to_owned(), "false".to_owned()),
        ]);
        let widget = DateTimeWidget::new(&config);

        let state = ZellijState {
            mode: ModeInfo {
                session_dimmed: Some(true),
                ..ModeInfo::default()
            },
            ..ZellijState::default()
        };

        assert_eq!(widget.process("datetime", &state), "");
    }

    #[test]
    fn test_shown_by_default_when_nested() {
        let config = BTreeMap::from([("datetime".to_owned(), "{time}".to_owned())]);
        let widget = DateTimeWidget::new(&config);

        let state = ZellijState {
            mode: ModeInfo {
                session_dimmed: Some(true),
                ..ModeInfo::default()
            },
            ..ZellijState::default()
        };

        assert_ne!(widget.process("datetime", &state), "");
    }

    #[test]
    fn test_shown_when_host_fullscreen_despite_nested_show_false() {
        let config = BTreeMap::from([
            ("datetime".to_owned(), "{time}".to_owned()),
            ("datetime_nested_show".to_owned(), "false".to_owned()),
        ]);
        let widget = DateTimeWidget::new(&config);

        let state = ZellijState {
            mode: ModeInfo {
                session_dimmed: Some(true),
                host_fullscreen: Some(true),
                ..ModeInfo::default()
            },
            ..ZellijState::default()
        };

        assert_ne!(widget.process("datetime", &state), "");
    }
}
