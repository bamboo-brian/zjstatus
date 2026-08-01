use zellij_tile::prelude::ModeInfo;

use crate::config::ZellijState;

pub trait Widget {
    fn process(&self, name: &str, state: &ZellijState) -> String;
    fn process_click(&self, name: &str, state: &ZellijState, pos: usize);
}

/// Whether a widget configured with `nested_show = false` should hide itself, given the
/// current session's nesting state. A host that is fully zoomed into a nested guest
/// (`host_fullscreen`) always shows its widgets regardless of `nested_show`.
pub fn should_hide_when_nested(nested_show: bool, mode: &ModeInfo) -> bool {
    if nested_show || mode.host_fullscreen.unwrap_or(false) {
        return false;
    }

    mode.session_ascended.unwrap_or(false)
        || mode.session_dimmed.unwrap_or(false)
        || !mode.session_ancestry.is_empty()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_not_nested_never_hides() {
        assert!(!should_hide_when_nested(false, &ModeInfo::default()));
        assert!(!should_hide_when_nested(true, &ModeInfo::default()));
    }

    #[test]
    fn test_nested_show_true_never_hides() {
        let mode = ModeInfo {
            session_ascended: Some(true),
            ..ModeInfo::default()
        };

        assert!(!should_hide_when_nested(true, &mode));
    }

    #[test]
    fn test_nested_show_false_hides_when_ascended() {
        let mode = ModeInfo {
            session_ascended: Some(true),
            ..ModeInfo::default()
        };

        assert!(should_hide_when_nested(false, &mode));
    }

    #[test]
    fn test_nested_show_false_hides_when_dimmed() {
        let mode = ModeInfo {
            session_dimmed: Some(true),
            ..ModeInfo::default()
        };

        assert!(should_hide_when_nested(false, &mode));
    }

    #[test]
    fn test_nested_show_false_hides_when_ancestry_present() {
        let mode = ModeInfo {
            session_ancestry: vec!["host-session".to_owned()],
            ..ModeInfo::default()
        };

        assert!(should_hide_when_nested(false, &mode));
    }

    #[test]
    fn test_host_fullscreen_overrides_hiding() {
        let mode = ModeInfo {
            session_dimmed: Some(true),
            host_fullscreen: Some(true),
            ..ModeInfo::default()
        };

        assert!(!should_hide_when_nested(false, &mode));
    }
}
