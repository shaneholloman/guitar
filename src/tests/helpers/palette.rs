use super::*;

#[test]
fn background_or_default_preserves_real_colors() {
    let theme = Theme::classic();

    assert_eq!(theme.COLOR_GREY_950, Color::Rgb(30, 30, 30));
    assert_eq!(theme.background_or_default(Color::Red), Color::Red);
    assert_eq!(theme.background_or_default(theme.COLOR_GREY_900), theme.COLOR_GREY_900);
}

#[test]
fn background_or_default_replaces_reset() {
    let theme = Theme::ansi();

    assert_eq!(theme.background_or_default(Color::Reset), theme.COLOR_GREY_950);
}

#[test]
fn clear_area_erases_symbols_and_paints_theme_background() {
    let theme = Theme::classic();
    let mut buffer = Buffer::with_lines(["abcde", "fghij", "klmno"]);
    let area = Rect::new(1, 1, 3, 1);

    theme.clear_area(area, &mut buffer);

    for x in 1..4 {
        let cell = &buffer[(x, 1)];
        assert_eq!(cell.symbol(), " ");
        assert_eq!(cell.bg, theme.COLOR_GREY_950);
    }

    assert_eq!(buffer[(0, 1)].symbol(), "f");
    assert_eq!(buffer[(4, 1)].symbol(), "j");
}

#[test]
fn presets_include_editor_theme_variants() {
    let expected = ["dracula dark", "dracula light", "monokai dark", "monokai light", "catppuccin dark", "catppuccin light", "atom dark", "atom light", "vscode dark", "vscode light"];

    for label in expected {
        assert!(Theme::presets().iter().any(|preset| preset.label == label), "missing theme preset {label}");
    }

    assert!(Theme::presets().iter().all(|preset| preset.theme.background_color() != Color::Reset));
}

#[test]
fn preset_labels_resolve_to_their_themes() {
    for preset in Theme::presets() {
        assert_eq!(preset.theme.label(), preset.label);
        let resolved = Theme::from_label(preset.label).unwrap();
        assert!(resolved.name == preset.theme.name);
    }

    assert!(Theme::from_label("dracula-dark").unwrap().name == ThemeNames::DraculaDark);
    assert!(Theme::from_label("vscode_light").unwrap().name == ThemeNames::VsCodeLight);
}
