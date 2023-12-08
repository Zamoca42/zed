// This file was generated by the `theme_importer`.
// Be careful when modifying it by hand.

use gpui::rgba;

#[allow(unused)]
use crate::{
    Appearance, StatusColorsRefinement, ThemeColorsRefinement, UserFontStyle, UserFontWeight,
    UserHighlightStyle, UserSyntaxTheme, UserTheme, UserThemeFamily, UserThemeStylesRefinement,
};

pub fn rose_pine() -> UserThemeFamily {
    UserThemeFamily {
        name: "Rose Pine".into(),
        author: "Rosé Pine".into(),
        themes: vec![
            UserTheme {
                name: "Rose Pine".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x000000ff).into()),
                        border_variant: Some(rgba(0x000000ff).into()),
                        border_focused: Some(rgba(0x6e6a8633).into()),
                        border_selected: Some(rgba(0x000000ff).into()),
                        border_transparent: Some(rgba(0x000000ff).into()),
                        border_disabled: Some(rgba(0x000000ff).into()),
                        elevated_surface_background: Some(rgba(0x1f1d2eff).into()),
                        surface_background: Some(rgba(0x1f1d2eff).into()),
                        background: Some(rgba(0x191724ff).into()),
                        element_background: Some(rgba(0xebbcbaff).into()),
                        element_hover: Some(rgba(0x6e6a861a).into()),
                        element_selected: Some(rgba(0x6e6a8633).into()),
                        drop_target_background: Some(rgba(0x1f1d2eff).into()),
                        ghost_element_hover: Some(rgba(0x6e6a861a).into()),
                        text: Some(rgba(0xe0def4ff).into()),
                        status_bar_background: Some(rgba(0x191724ff).into()),
                        title_bar_background: Some(rgba(0x191724ff).into()),
                        toolbar_background: Some(rgba(0x1f1d2eff).into()),
                        tab_bar_background: Some(rgba(0x1f1d2eff).into()),
                        tab_inactive_background: Some(rgba(0x000000ff).into()),
                        tab_active_background: Some(rgba(0x6e6a861a).into()),
                        editor_background: Some(rgba(0x191724ff).into()),
                        editor_gutter_background: Some(rgba(0x191724ff).into()),
                        editor_line_number: Some(rgba(0x908caaff).into()),
                        editor_active_line_number: Some(rgba(0xe0def4ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x908caaff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xeb6f92ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x31748fff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xf6c177ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x9ccfd8ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc4a7e7ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0xebbcbaff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xe0def4ff).into()),
                        terminal_ansi_black: Some(rgba(0x26233aff).into()),
                        terminal_ansi_red: Some(rgba(0xeb6f92ff).into()),
                        terminal_ansi_green: Some(rgba(0x31748fff).into()),
                        terminal_ansi_yellow: Some(rgba(0xf6c177ff).into()),
                        terminal_ansi_blue: Some(rgba(0x9ccfd8ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc4a7e7ff).into()),
                        terminal_ansi_cyan: Some(rgba(0xebbcbaff).into()),
                        terminal_ansi_white: Some(rgba(0xe0def4ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xeb6f92ff).into()),
                        error: Some(rgba(0xeb6f92ff).into()),
                        hidden: Some(rgba(0x908caaff).into()),
                        hint: Some(rgba(0x908caaff).into()),
                        warning: Some(rgba(0xf6c177ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc4a7e7ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xebbcbaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xebbcbaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9ccfd8ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xebbcbaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x31748fff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "label".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xebbcbaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xebbcbaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9ccfd8ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "title".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xebbcbaff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9ccfd8ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc4a7e7ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xe0def4ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
            UserTheme {
                name: "Rose Pine Moon".into(),
                appearance: Appearance::Dark,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x000000ff).into()),
                        border_variant: Some(rgba(0x000000ff).into()),
                        border_focused: Some(rgba(0x817c9c26).into()),
                        border_selected: Some(rgba(0x000000ff).into()),
                        border_transparent: Some(rgba(0x000000ff).into()),
                        border_disabled: Some(rgba(0x000000ff).into()),
                        elevated_surface_background: Some(rgba(0x2a273fff).into()),
                        surface_background: Some(rgba(0x2a273fff).into()),
                        background: Some(rgba(0x232136ff).into()),
                        element_background: Some(rgba(0xea9a97ff).into()),
                        element_hover: Some(rgba(0x817c9c14).into()),
                        element_selected: Some(rgba(0x817c9c26).into()),
                        drop_target_background: Some(rgba(0x2a273fff).into()),
                        ghost_element_hover: Some(rgba(0x817c9c14).into()),
                        text: Some(rgba(0xe0def4ff).into()),
                        status_bar_background: Some(rgba(0x232136ff).into()),
                        title_bar_background: Some(rgba(0x232136ff).into()),
                        toolbar_background: Some(rgba(0x2a273fff).into()),
                        tab_bar_background: Some(rgba(0x2a273fff).into()),
                        tab_inactive_background: Some(rgba(0x000000ff).into()),
                        tab_active_background: Some(rgba(0x817c9c14).into()),
                        editor_background: Some(rgba(0x232136ff).into()),
                        editor_gutter_background: Some(rgba(0x232136ff).into()),
                        editor_line_number: Some(rgba(0x908caaff).into()),
                        editor_active_line_number: Some(rgba(0xe0def4ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x908caaff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xeb6f92ff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x3e8fb0ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xf6c177ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x9ccfd8ff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0xc4a7e7ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0xea9a97ff).into()),
                        terminal_ansi_bright_white: Some(rgba(0xe0def4ff).into()),
                        terminal_ansi_black: Some(rgba(0x393552ff).into()),
                        terminal_ansi_red: Some(rgba(0xeb6f92ff).into()),
                        terminal_ansi_green: Some(rgba(0x3e8fb0ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xf6c177ff).into()),
                        terminal_ansi_blue: Some(rgba(0x9ccfd8ff).into()),
                        terminal_ansi_magenta: Some(rgba(0xc4a7e7ff).into()),
                        terminal_ansi_cyan: Some(rgba(0xea9a97ff).into()),
                        terminal_ansi_white: Some(rgba(0xe0def4ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xeb6f92ff).into()),
                        error: Some(rgba(0xeb6f92ff).into()),
                        hidden: Some(rgba(0x908caaff).into()),
                        hint: Some(rgba(0x908caaff).into()),
                        warning: Some(rgba(0xf6c177ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc4a7e7ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9a97ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9a97ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9ccfd8ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9a97ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x3e8fb0ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "label".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9a97ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9a97ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x6e6a86ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9ccfd8ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xf6c177ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "title".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9a97ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9ccfd8ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xc4a7e7ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xe0def4ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
            UserTheme {
                name: "Rose Pine Dawn".into(),
                appearance: Appearance::Light,
                styles: UserThemeStylesRefinement {
                    colors: ThemeColorsRefinement {
                        border: Some(rgba(0x000000ff).into()),
                        border_variant: Some(rgba(0x000000ff).into()),
                        border_focused: Some(rgba(0x6e6a8614).into()),
                        border_selected: Some(rgba(0x000000ff).into()),
                        border_transparent: Some(rgba(0x000000ff).into()),
                        border_disabled: Some(rgba(0x000000ff).into()),
                        elevated_surface_background: Some(rgba(0xfffaf3ff).into()),
                        surface_background: Some(rgba(0xfffaf3ff).into()),
                        background: Some(rgba(0xfaf4edff).into()),
                        element_background: Some(rgba(0xd7827eff).into()),
                        element_hover: Some(rgba(0x6e6a860d).into()),
                        element_selected: Some(rgba(0x6e6a8614).into()),
                        drop_target_background: Some(rgba(0xfffaf3ff).into()),
                        ghost_element_hover: Some(rgba(0x6e6a860d).into()),
                        text: Some(rgba(0x575279ff).into()),
                        status_bar_background: Some(rgba(0xfaf4edff).into()),
                        title_bar_background: Some(rgba(0xfaf4edff).into()),
                        toolbar_background: Some(rgba(0xfffaf3ff).into()),
                        tab_bar_background: Some(rgba(0xfffaf3ff).into()),
                        tab_inactive_background: Some(rgba(0x000000ff).into()),
                        tab_active_background: Some(rgba(0x6e6a860d).into()),
                        editor_background: Some(rgba(0xfaf4edff).into()),
                        editor_gutter_background: Some(rgba(0xfaf4edff).into()),
                        editor_line_number: Some(rgba(0x797593ff).into()),
                        editor_active_line_number: Some(rgba(0x575279ff).into()),
                        terminal_ansi_bright_black: Some(rgba(0x797593ff).into()),
                        terminal_ansi_bright_red: Some(rgba(0xb4637aff).into()),
                        terminal_ansi_bright_green: Some(rgba(0x286983ff).into()),
                        terminal_ansi_bright_yellow: Some(rgba(0xea9d34ff).into()),
                        terminal_ansi_bright_blue: Some(rgba(0x56949fff).into()),
                        terminal_ansi_bright_magenta: Some(rgba(0x907aa9ff).into()),
                        terminal_ansi_bright_cyan: Some(rgba(0xd7827eff).into()),
                        terminal_ansi_bright_white: Some(rgba(0x575279ff).into()),
                        terminal_ansi_black: Some(rgba(0xf2e9e1ff).into()),
                        terminal_ansi_red: Some(rgba(0xb4637aff).into()),
                        terminal_ansi_green: Some(rgba(0x286983ff).into()),
                        terminal_ansi_yellow: Some(rgba(0xea9d34ff).into()),
                        terminal_ansi_blue: Some(rgba(0x56949fff).into()),
                        terminal_ansi_magenta: Some(rgba(0x907aa9ff).into()),
                        terminal_ansi_cyan: Some(rgba(0xd7827eff).into()),
                        terminal_ansi_white: Some(rgba(0x575279ff).into()),
                        ..Default::default()
                    },
                    status: StatusColorsRefinement {
                        deleted: Some(rgba(0xb4637aff).into()),
                        error: Some(rgba(0xb4637aff).into()),
                        hidden: Some(rgba(0x797593ff).into()),
                        hint: Some(rgba(0x797593ff).into()),
                        warning: Some(rgba(0xea9d34ff).into()),
                        ..Default::default()
                    },
                    syntax: Some(UserSyntaxTheme {
                        highlights: vec![
                            (
                                "attribute".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x907aa9ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "boolean".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7827eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "comment.doc".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    font_style: Some(UserFontStyle::Italic),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constant".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7827eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "constructor".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x56949fff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "function".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7827eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "keyword".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x286983ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "label".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7827eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "number".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7827eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.bracket".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.delimiter".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.list_marker".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "punctuation.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x9893a5ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9d34ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.escape".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9d34ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.regex".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9d34ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9d34ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "string.special.symbol".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9d34ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "tag".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x56949fff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "text.literal".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xea9d34ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "title".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0xd7827eff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "type".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x56949fff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x907aa9ff).into()),
                                    ..Default::default()
                                },
                            ),
                            (
                                "variable.special".into(),
                                UserHighlightStyle {
                                    color: Some(rgba(0x575279ff).into()),
                                    ..Default::default()
                                },
                            ),
                        ],
                    }),
                },
            },
        ],
    }
}
