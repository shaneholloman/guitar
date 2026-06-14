#![allow(non_snake_case)]

use ratatui::{
    buffer::Buffer,
    layout::Rect,
    style::{Color, Style},
};
use std::{fs, path::PathBuf};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum ThemeNames {
    Classic,
    Ansi,
    Monochrome,
    DraculaDark,
    DraculaLight,
    MonokaiDark,
    MonokaiLight,
    CatppuccinDark,
    CatppuccinLight,
    AtomDark,
    AtomLight,
    VsCodeDark,
    VsCodeLight,
}

#[derive(Clone, Copy)]
pub struct Theme {
    pub name: ThemeNames,
    pub COLOR_RED: Color,
    pub COLOR_PINK: Color,
    pub COLOR_PURPLE: Color,
    pub COLOR_DURPLE: Color,
    pub COLOR_INDIGO: Color,
    pub COLOR_BLUE: Color,
    pub COLOR_CYAN: Color,
    pub COLOR_TEAL: Color,
    pub COLOR_GREEN: Color,
    pub COLOR_GRASS: Color,
    pub COLOR_LIME: Color,
    pub COLOR_YELLOW: Color,
    pub COLOR_AMBER: Color,
    pub COLOR_ORANGE: Color,
    pub COLOR_GRAPEFRUIT: Color,
    pub COLOR_BROWN: Color,
    pub COLOR_DARK_RED: Color,
    pub COLOR_LIGHT_GREEN_900: Color,
    pub COLOR_GREY_50: Color,
    pub COLOR_GREY_100: Color,
    pub COLOR_GREY_200: Color,
    pub COLOR_GREY_300: Color,
    pub COLOR_GREY_400: Color,
    pub COLOR_GREY_500: Color,
    pub COLOR_GREY_600: Color,
    pub COLOR_GREY_700: Color,
    pub COLOR_GREY_800: Color,
    pub COLOR_GREY_900: Color,
    pub COLOR_GREY_950: Color,
    pub COLOR_BORDER: Color,
    pub COLOR_TEXT: Color,
    pub COLOR_TEXT_SELECTED: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self::classic()
    }
}

impl Theme {
    pub const fn label(&self) -> &'static str {
        self.name.label()
    }

    pub fn from_label(label: &str) -> Option<Self> {
        let normalized = label.trim().to_ascii_lowercase().replace(['-', '_'], " ");
        Self::presets().iter().find(|preset| preset.label == normalized).map(|preset| preset.theme)
    }

    pub const fn background_color(&self) -> Color {
        self.COLOR_GREY_950
    }

    pub const fn background_style(&self) -> Style {
        Style::new().bg(self.background_color())
    }

    pub const fn background_or_default(&self, color: Color) -> Color {
        match color {
            Color::Reset => self.background_color(),
            _ => color,
        }
    }

    pub fn clear_area(&self, area: Rect, buf: &mut Buffer) {
        let area = area.intersection(*buf.area());
        if area.is_empty() {
            return;
        }

        for x in area.left()..area.right() {
            for y in area.top()..area.bottom() {
                let cell = &mut buf[(x, y)];
                cell.reset();
                cell.set_bg(self.background_color());
            }
        }
    }

    pub const fn classic() -> Self {
        Self {
            name: ThemeNames::Classic,
            COLOR_RED: Color::Rgb(239, 83, 80),
            COLOR_PINK: Color::Rgb(236, 64, 122),
            COLOR_PURPLE: Color::Rgb(171, 71, 188),
            COLOR_DURPLE: Color::Rgb(126, 87, 194),
            COLOR_INDIGO: Color::Rgb(92, 107, 192),
            COLOR_BLUE: Color::Rgb(66, 165, 245),
            COLOR_CYAN: Color::Rgb(38, 198, 218),
            COLOR_TEAL: Color::Rgb(38, 166, 154),
            COLOR_GREEN: Color::Rgb(102, 187, 106),
            COLOR_GRASS: Color::Rgb(156, 204, 101),
            COLOR_LIME: Color::Rgb(212, 225, 87),
            COLOR_YELLOW: Color::Rgb(255, 238, 88),
            COLOR_AMBER: Color::Rgb(255, 202, 40),
            COLOR_ORANGE: Color::Rgb(255, 167, 38),
            COLOR_GRAPEFRUIT: Color::Rgb(255, 112, 67),
            COLOR_BROWN: Color::Rgb(141, 110, 99),
            COLOR_DARK_RED: Color::Rgb(82, 31, 31),
            COLOR_LIGHT_GREEN_900: Color::Rgb(34, 57, 37),
            COLOR_GREY_50: Color::Rgb(250, 250, 250),
            COLOR_GREY_100: Color::Rgb(245, 245, 245),
            COLOR_GREY_200: Color::Rgb(238, 238, 238),
            COLOR_GREY_300: Color::Rgb(224, 224, 224),
            COLOR_GREY_400: Color::Rgb(189, 189, 189),
            COLOR_GREY_500: Color::Rgb(158, 158, 158),
            COLOR_GREY_600: Color::Rgb(117, 117, 117),
            COLOR_GREY_700: Color::Rgb(97, 97, 97),
            COLOR_GREY_800: Color::Rgb(66, 66, 66),
            COLOR_GREY_900: Color::Rgb(33, 33, 33),
            COLOR_GREY_950: Color::Rgb(30, 30, 30),
            COLOR_BORDER: Color::Rgb(66, 66, 66),
            COLOR_TEXT: Color::Rgb(97, 97, 97),
            COLOR_TEXT_SELECTED: Color::Rgb(224, 224, 224),
        }
    }
    pub const fn ansi() -> Self {
        Self {
            name: ThemeNames::Ansi,
            COLOR_RED: Color::Red,
            COLOR_PINK: Color::LightRed,
            COLOR_PURPLE: Color::Magenta,
            COLOR_DURPLE: Color::LightMagenta,
            COLOR_INDIGO: Color::Blue,
            COLOR_BLUE: Color::LightBlue,
            COLOR_CYAN: Color::Cyan,
            COLOR_TEAL: Color::LightCyan,
            COLOR_GREEN: Color::Green,
            COLOR_GRASS: Color::LightGreen,
            COLOR_LIME: Color::Yellow,
            COLOR_YELLOW: Color::LightYellow,
            COLOR_AMBER: Color::Red,
            COLOR_ORANGE: Color::LightRed,
            COLOR_GRAPEFRUIT: Color::Magenta,
            COLOR_BROWN: Color::LightMagenta,
            COLOR_DARK_RED: Color::Reset,
            COLOR_LIGHT_GREEN_900: Color::Reset,
            COLOR_GREY_50: Color::Gray,
            COLOR_GREY_100: Color::Gray,
            COLOR_GREY_200: Color::Gray,
            COLOR_GREY_300: Color::Gray,
            COLOR_GREY_400: Color::DarkGray,
            COLOR_GREY_500: Color::DarkGray,
            COLOR_GREY_600: Color::DarkGray,
            COLOR_GREY_700: Color::DarkGray,
            COLOR_GREY_800: Color::DarkGray,
            COLOR_GREY_900: Color::Black,
            COLOR_GREY_950: Color::Rgb(30, 30, 30),
            COLOR_BORDER: Color::DarkGray,
            COLOR_TEXT: Color::White,
            COLOR_TEXT_SELECTED: Color::Reset,
        }
    }
    pub const fn monochrome() -> Self {
        Self {
            name: ThemeNames::Monochrome,
            COLOR_RED: Color::White,
            COLOR_PINK: Color::White,
            COLOR_PURPLE: Color::White,
            COLOR_DURPLE: Color::White,
            COLOR_INDIGO: Color::White,
            COLOR_BLUE: Color::White,
            COLOR_CYAN: Color::White,
            COLOR_TEAL: Color::White,
            COLOR_GREEN: Color::White,
            COLOR_GRASS: Color::White,
            COLOR_LIME: Color::White,
            COLOR_YELLOW: Color::White,
            COLOR_AMBER: Color::White,
            COLOR_ORANGE: Color::White,
            COLOR_GRAPEFRUIT: Color::White,
            COLOR_BROWN: Color::White,
            COLOR_DARK_RED: Color::DarkGray,
            COLOR_LIGHT_GREEN_900: Color::DarkGray,
            COLOR_GREY_50: Color::Gray,
            COLOR_GREY_100: Color::Gray,
            COLOR_GREY_200: Color::Gray,
            COLOR_GREY_300: Color::Gray,
            COLOR_GREY_400: Color::DarkGray,
            COLOR_GREY_500: Color::DarkGray,
            COLOR_GREY_600: Color::DarkGray,
            COLOR_GREY_700: Color::DarkGray,
            COLOR_GREY_800: Color::DarkGray,
            COLOR_GREY_900: Color::Black,
            COLOR_GREY_950: Color::Rgb(30, 30, 30),
            COLOR_BORDER: Color::DarkGray,
            COLOR_TEXT: Color::White,
            COLOR_TEXT_SELECTED: Color::Reset,
        }
    }

    pub const fn dracula_dark() -> Self {
        Self {
            name: ThemeNames::DraculaDark,
            COLOR_RED: Color::Rgb(255, 85, 85),
            COLOR_PINK: Color::Rgb(255, 121, 198),
            COLOR_PURPLE: Color::Rgb(189, 147, 249),
            COLOR_DURPLE: Color::Rgb(139, 133, 255),
            COLOR_INDIGO: Color::Rgb(98, 114, 164),
            COLOR_BLUE: Color::Rgb(139, 233, 253),
            COLOR_CYAN: Color::Rgb(139, 233, 253),
            COLOR_TEAL: Color::Rgb(82, 225, 211),
            COLOR_GREEN: Color::Rgb(80, 250, 123),
            COLOR_GRASS: Color::Rgb(105, 255, 137),
            COLOR_LIME: Color::Rgb(203, 255, 128),
            COLOR_YELLOW: Color::Rgb(241, 250, 140),
            COLOR_AMBER: Color::Rgb(255, 202, 102),
            COLOR_ORANGE: Color::Rgb(255, 184, 108),
            COLOR_GRAPEFRUIT: Color::Rgb(255, 110, 110),
            COLOR_BROWN: Color::Rgb(181, 140, 110),
            COLOR_DARK_RED: Color::Rgb(68, 35, 49),
            COLOR_LIGHT_GREEN_900: Color::Rgb(35, 68, 47),
            COLOR_GREY_50: Color::Rgb(248, 248, 242),
            COLOR_GREY_100: Color::Rgb(225, 225, 218),
            COLOR_GREY_200: Color::Rgb(191, 191, 186),
            COLOR_GREY_300: Color::Rgb(145, 148, 166),
            COLOR_GREY_400: Color::Rgb(98, 114, 164),
            COLOR_GREY_500: Color::Rgb(88, 92, 112),
            COLOR_GREY_600: Color::Rgb(68, 71, 90),
            COLOR_GREY_700: Color::Rgb(52, 55, 70),
            COLOR_GREY_800: Color::Rgb(68, 71, 90),
            COLOR_GREY_900: Color::Rgb(40, 42, 54),
            COLOR_GREY_950: Color::Rgb(40, 42, 54),
            COLOR_BORDER: Color::Rgb(68, 71, 90),
            COLOR_TEXT: Color::Rgb(248, 248, 242),
            COLOR_TEXT_SELECTED: Color::Rgb(248, 248, 242),
        }
    }

    pub const fn dracula_light() -> Self {
        Self {
            name: ThemeNames::DraculaLight,
            COLOR_RED: Color::Rgb(214, 66, 66),
            COLOR_PINK: Color::Rgb(199, 63, 134),
            COLOR_PURPLE: Color::Rgb(124, 91, 191),
            COLOR_DURPLE: Color::Rgb(104, 89, 178),
            COLOR_INDIGO: Color::Rgb(77, 88, 132),
            COLOR_BLUE: Color::Rgb(45, 133, 168),
            COLOR_CYAN: Color::Rgb(8, 127, 149),
            COLOR_TEAL: Color::Rgb(24, 145, 132),
            COLOR_GREEN: Color::Rgb(47, 158, 68),
            COLOR_GRASS: Color::Rgb(64, 180, 77),
            COLOR_LIME: Color::Rgb(105, 166, 32),
            COLOR_YELLOW: Color::Rgb(160, 120, 0),
            COLOR_AMBER: Color::Rgb(184, 107, 19),
            COLOR_ORANGE: Color::Rgb(176, 100, 24),
            COLOR_GRAPEFRUIT: Color::Rgb(197, 79, 68),
            COLOR_BROWN: Color::Rgb(124, 89, 72),
            COLOR_DARK_RED: Color::Rgb(255, 230, 232),
            COLOR_LIGHT_GREEN_900: Color::Rgb(225, 248, 229),
            COLOR_GREY_50: Color::Rgb(40, 42, 54),
            COLOR_GREY_100: Color::Rgb(58, 54, 64),
            COLOR_GREY_200: Color::Rgb(78, 70, 82),
            COLOR_GREY_300: Color::Rgb(103, 94, 105),
            COLOR_GREY_400: Color::Rgb(128, 119, 128),
            COLOR_GREY_500: Color::Rgb(112, 102, 92),
            COLOR_GREY_600: Color::Rgb(151, 141, 126),
            COLOR_GREY_700: Color::Rgb(190, 183, 170),
            COLOR_GREY_800: Color::Rgb(221, 217, 207),
            COLOR_GREY_900: Color::Rgb(239, 238, 232),
            COLOR_GREY_950: Color::Rgb(248, 248, 242),
            COLOR_BORDER: Color::Rgb(214, 208, 198),
            COLOR_TEXT: Color::Rgb(40, 42, 54),
            COLOR_TEXT_SELECTED: Color::Rgb(40, 42, 54),
        }
    }

    pub const fn monokai_dark() -> Self {
        Self {
            name: ThemeNames::MonokaiDark,
            COLOR_RED: Color::Rgb(249, 38, 114),
            COLOR_PINK: Color::Rgb(249, 38, 114),
            COLOR_PURPLE: Color::Rgb(174, 129, 255),
            COLOR_DURPLE: Color::Rgb(146, 118, 218),
            COLOR_INDIGO: Color::Rgb(102, 137, 204),
            COLOR_BLUE: Color::Rgb(102, 217, 239),
            COLOR_CYAN: Color::Rgb(102, 217, 239),
            COLOR_TEAL: Color::Rgb(95, 207, 192),
            COLOR_GREEN: Color::Rgb(166, 226, 46),
            COLOR_GRASS: Color::Rgb(190, 237, 76),
            COLOR_LIME: Color::Rgb(210, 235, 86),
            COLOR_YELLOW: Color::Rgb(230, 219, 116),
            COLOR_AMBER: Color::Rgb(244, 191, 117),
            COLOR_ORANGE: Color::Rgb(253, 151, 31),
            COLOR_GRAPEFRUIT: Color::Rgb(255, 97, 136),
            COLOR_BROWN: Color::Rgb(117, 113, 94),
            COLOR_DARK_RED: Color::Rgb(64, 25, 42),
            COLOR_LIGHT_GREEN_900: Color::Rgb(38, 62, 24),
            COLOR_GREY_50: Color::Rgb(248, 248, 242),
            COLOR_GREY_100: Color::Rgb(220, 220, 214),
            COLOR_GREY_200: Color::Rgb(190, 190, 180),
            COLOR_GREY_300: Color::Rgb(160, 160, 148),
            COLOR_GREY_400: Color::Rgb(117, 113, 94),
            COLOR_GREY_500: Color::Rgb(96, 94, 83),
            COLOR_GREY_600: Color::Rgb(73, 72, 62),
            COLOR_GREY_700: Color::Rgb(56, 56, 48),
            COLOR_GREY_800: Color::Rgb(73, 72, 62),
            COLOR_GREY_900: Color::Rgb(39, 40, 34),
            COLOR_GREY_950: Color::Rgb(39, 40, 34),
            COLOR_BORDER: Color::Rgb(73, 72, 62),
            COLOR_TEXT: Color::Rgb(248, 248, 242),
            COLOR_TEXT_SELECTED: Color::Rgb(248, 248, 242),
        }
    }

    pub const fn monokai_light() -> Self {
        Self {
            name: ThemeNames::MonokaiLight,
            COLOR_RED: Color::Rgb(249, 38, 114),
            COLOR_PINK: Color::Rgb(214, 54, 128),
            COLOR_PURPLE: Color::Rgb(126, 87, 194),
            COLOR_DURPLE: Color::Rgb(113, 83, 180),
            COLOR_INDIGO: Color::Rgb(63, 99, 175),
            COLOR_BLUE: Color::Rgb(0, 132, 181),
            COLOR_CYAN: Color::Rgb(0, 145, 170),
            COLOR_TEAL: Color::Rgb(0, 145, 124),
            COLOR_GREEN: Color::Rgb(80, 150, 31),
            COLOR_GRASS: Color::Rgb(98, 165, 35),
            COLOR_LIME: Color::Rgb(130, 160, 20),
            COLOR_YELLOW: Color::Rgb(151, 128, 24),
            COLOR_AMBER: Color::Rgb(190, 112, 22),
            COLOR_ORANGE: Color::Rgb(210, 92, 20),
            COLOR_GRAPEFRUIT: Color::Rgb(216, 70, 88),
            COLOR_BROWN: Color::Rgb(115, 87, 69),
            COLOR_DARK_RED: Color::Rgb(255, 230, 239),
            COLOR_LIGHT_GREEN_900: Color::Rgb(232, 248, 218),
            COLOR_GREY_50: Color::Rgb(39, 40, 34),
            COLOR_GREY_100: Color::Rgb(65, 65, 58),
            COLOR_GREY_200: Color::Rgb(91, 88, 77),
            COLOR_GREY_300: Color::Rgb(122, 116, 100),
            COLOR_GREY_400: Color::Rgb(150, 145, 130),
            COLOR_GREY_500: Color::Rgb(94, 89, 82),
            COLOR_GREY_600: Color::Rgb(166, 163, 153),
            COLOR_GREY_700: Color::Rgb(208, 205, 194),
            COLOR_GREY_800: Color::Rgb(232, 230, 222),
            COLOR_GREY_900: Color::Rgb(245, 244, 239),
            COLOR_GREY_950: Color::Rgb(250, 250, 247),
            COLOR_BORDER: Color::Rgb(218, 216, 208),
            COLOR_TEXT: Color::Rgb(64, 62, 65),
            COLOR_TEXT_SELECTED: Color::Rgb(64, 62, 65),
        }
    }

    pub const fn catppuccin_dark() -> Self {
        Self {
            name: ThemeNames::CatppuccinDark,
            COLOR_RED: Color::Rgb(243, 139, 168),
            COLOR_PINK: Color::Rgb(245, 194, 231),
            COLOR_PURPLE: Color::Rgb(203, 166, 247),
            COLOR_DURPLE: Color::Rgb(180, 190, 254),
            COLOR_INDIGO: Color::Rgb(137, 180, 250),
            COLOR_BLUE: Color::Rgb(137, 180, 250),
            COLOR_CYAN: Color::Rgb(137, 220, 235),
            COLOR_TEAL: Color::Rgb(148, 226, 213),
            COLOR_GREEN: Color::Rgb(166, 227, 161),
            COLOR_GRASS: Color::Rgb(166, 227, 161),
            COLOR_LIME: Color::Rgb(184, 232, 128),
            COLOR_YELLOW: Color::Rgb(249, 226, 175),
            COLOR_AMBER: Color::Rgb(250, 179, 135),
            COLOR_ORANGE: Color::Rgb(250, 179, 135),
            COLOR_GRAPEFRUIT: Color::Rgb(235, 160, 172),
            COLOR_BROWN: Color::Rgb(166, 148, 129),
            COLOR_DARK_RED: Color::Rgb(63, 39, 53),
            COLOR_LIGHT_GREEN_900: Color::Rgb(42, 61, 48),
            COLOR_GREY_50: Color::Rgb(205, 214, 244),
            COLOR_GREY_100: Color::Rgb(186, 194, 222),
            COLOR_GREY_200: Color::Rgb(166, 173, 200),
            COLOR_GREY_300: Color::Rgb(147, 153, 178),
            COLOR_GREY_400: Color::Rgb(127, 132, 156),
            COLOR_GREY_500: Color::Rgb(108, 112, 134),
            COLOR_GREY_600: Color::Rgb(88, 91, 112),
            COLOR_GREY_700: Color::Rgb(69, 71, 90),
            COLOR_GREY_800: Color::Rgb(49, 50, 68),
            COLOR_GREY_900: Color::Rgb(30, 30, 46),
            COLOR_GREY_950: Color::Rgb(30, 30, 46),
            COLOR_BORDER: Color::Rgb(69, 71, 90),
            COLOR_TEXT: Color::Rgb(205, 214, 244),
            COLOR_TEXT_SELECTED: Color::Rgb(205, 214, 244),
        }
    }

    pub const fn catppuccin_light() -> Self {
        Self {
            name: ThemeNames::CatppuccinLight,
            COLOR_RED: Color::Rgb(210, 15, 57),
            COLOR_PINK: Color::Rgb(234, 118, 203),
            COLOR_PURPLE: Color::Rgb(136, 57, 239),
            COLOR_DURPLE: Color::Rgb(114, 135, 253),
            COLOR_INDIGO: Color::Rgb(30, 102, 245),
            COLOR_BLUE: Color::Rgb(30, 102, 245),
            COLOR_CYAN: Color::Rgb(4, 165, 229),
            COLOR_TEAL: Color::Rgb(23, 146, 153),
            COLOR_GREEN: Color::Rgb(64, 160, 43),
            COLOR_GRASS: Color::Rgb(64, 160, 43),
            COLOR_LIME: Color::Rgb(92, 145, 18),
            COLOR_YELLOW: Color::Rgb(223, 142, 29),
            COLOR_AMBER: Color::Rgb(254, 100, 11),
            COLOR_ORANGE: Color::Rgb(254, 100, 11),
            COLOR_GRAPEFRUIT: Color::Rgb(230, 69, 83),
            COLOR_BROWN: Color::Rgb(124, 90, 76),
            COLOR_DARK_RED: Color::Rgb(252, 226, 231),
            COLOR_LIGHT_GREEN_900: Color::Rgb(230, 246, 224),
            COLOR_GREY_50: Color::Rgb(76, 79, 105),
            COLOR_GREY_100: Color::Rgb(92, 95, 119),
            COLOR_GREY_200: Color::Rgb(108, 111, 133),
            COLOR_GREY_300: Color::Rgb(124, 127, 147),
            COLOR_GREY_400: Color::Rgb(140, 143, 161),
            COLOR_GREY_500: Color::Rgb(108, 111, 133),
            COLOR_GREY_600: Color::Rgb(172, 176, 190),
            COLOR_GREY_700: Color::Rgb(204, 208, 218),
            COLOR_GREY_800: Color::Rgb(220, 224, 232),
            COLOR_GREY_900: Color::Rgb(230, 233, 239),
            COLOR_GREY_950: Color::Rgb(239, 241, 245),
            COLOR_BORDER: Color::Rgb(204, 208, 218),
            COLOR_TEXT: Color::Rgb(76, 79, 105),
            COLOR_TEXT_SELECTED: Color::Rgb(76, 79, 105),
        }
    }

    pub const fn atom_dark() -> Self {
        Self {
            name: ThemeNames::AtomDark,
            COLOR_RED: Color::Rgb(224, 108, 117),
            COLOR_PINK: Color::Rgb(198, 120, 221),
            COLOR_PURPLE: Color::Rgb(198, 120, 221),
            COLOR_DURPLE: Color::Rgb(171, 123, 216),
            COLOR_INDIGO: Color::Rgb(97, 175, 239),
            COLOR_BLUE: Color::Rgb(97, 175, 239),
            COLOR_CYAN: Color::Rgb(86, 182, 194),
            COLOR_TEAL: Color::Rgb(86, 182, 194),
            COLOR_GREEN: Color::Rgb(152, 195, 121),
            COLOR_GRASS: Color::Rgb(152, 195, 121),
            COLOR_LIME: Color::Rgb(183, 205, 129),
            COLOR_YELLOW: Color::Rgb(229, 192, 123),
            COLOR_AMBER: Color::Rgb(209, 154, 102),
            COLOR_ORANGE: Color::Rgb(209, 154, 102),
            COLOR_GRAPEFRUIT: Color::Rgb(224, 132, 122),
            COLOR_BROWN: Color::Rgb(145, 129, 111),
            COLOR_DARK_RED: Color::Rgb(64, 36, 42),
            COLOR_LIGHT_GREEN_900: Color::Rgb(42, 60, 39),
            COLOR_GREY_50: Color::Rgb(171, 178, 191),
            COLOR_GREY_100: Color::Rgb(157, 165, 180),
            COLOR_GREY_200: Color::Rgb(130, 137, 151),
            COLOR_GREY_300: Color::Rgb(112, 119, 133),
            COLOR_GREY_400: Color::Rgb(92, 99, 112),
            COLOR_GREY_500: Color::Rgb(78, 84, 96),
            COLOR_GREY_600: Color::Rgb(62, 68, 81),
            COLOR_GREY_700: Color::Rgb(53, 59, 71),
            COLOR_GREY_800: Color::Rgb(62, 68, 81),
            COLOR_GREY_900: Color::Rgb(40, 44, 52),
            COLOR_GREY_950: Color::Rgb(40, 44, 52),
            COLOR_BORDER: Color::Rgb(62, 68, 81),
            COLOR_TEXT: Color::Rgb(171, 178, 191),
            COLOR_TEXT_SELECTED: Color::Rgb(171, 178, 191),
        }
    }

    pub const fn atom_light() -> Self {
        Self {
            name: ThemeNames::AtomLight,
            COLOR_RED: Color::Rgb(228, 86, 73),
            COLOR_PINK: Color::Rgb(166, 38, 164),
            COLOR_PURPLE: Color::Rgb(166, 38, 164),
            COLOR_DURPLE: Color::Rgb(128, 62, 170),
            COLOR_INDIGO: Color::Rgb(64, 120, 242),
            COLOR_BLUE: Color::Rgb(64, 120, 242),
            COLOR_CYAN: Color::Rgb(1, 132, 188),
            COLOR_TEAL: Color::Rgb(1, 135, 145),
            COLOR_GREEN: Color::Rgb(80, 161, 79),
            COLOR_GRASS: Color::Rgb(80, 161, 79),
            COLOR_LIME: Color::Rgb(110, 155, 42),
            COLOR_YELLOW: Color::Rgb(193, 132, 1),
            COLOR_AMBER: Color::Rgb(202, 118, 0),
            COLOR_ORANGE: Color::Rgb(202, 118, 0),
            COLOR_GRAPEFRUIT: Color::Rgb(211, 82, 70),
            COLOR_BROWN: Color::Rgb(138, 98, 75),
            COLOR_DARK_RED: Color::Rgb(255, 230, 227),
            COLOR_LIGHT_GREEN_900: Color::Rgb(231, 247, 230),
            COLOR_GREY_50: Color::Rgb(56, 58, 66),
            COLOR_GREY_100: Color::Rgb(75, 77, 86),
            COLOR_GREY_200: Color::Rgb(92, 95, 105),
            COLOR_GREY_300: Color::Rgb(122, 126, 138),
            COLOR_GREY_400: Color::Rgb(160, 161, 167),
            COLOR_GREY_500: Color::Rgb(105, 108, 119),
            COLOR_GREY_600: Color::Rgb(186, 188, 195),
            COLOR_GREY_700: Color::Rgb(218, 220, 226),
            COLOR_GREY_800: Color::Rgb(235, 237, 240),
            COLOR_GREY_900: Color::Rgb(245, 246, 247),
            COLOR_GREY_950: Color::Rgb(250, 250, 250),
            COLOR_BORDER: Color::Rgb(218, 220, 226),
            COLOR_TEXT: Color::Rgb(56, 58, 66),
            COLOR_TEXT_SELECTED: Color::Rgb(56, 58, 66),
        }
    }

    pub const fn vscode_dark() -> Self {
        Self {
            name: ThemeNames::VsCodeDark,
            COLOR_RED: Color::Rgb(244, 71, 71),
            COLOR_PINK: Color::Rgb(197, 134, 192),
            COLOR_PURPLE: Color::Rgb(197, 134, 192),
            COLOR_DURPLE: Color::Rgb(156, 139, 207),
            COLOR_INDIGO: Color::Rgb(86, 156, 214),
            COLOR_BLUE: Color::Rgb(86, 156, 214),
            COLOR_CYAN: Color::Rgb(78, 201, 176),
            COLOR_TEAL: Color::Rgb(78, 201, 176),
            COLOR_GREEN: Color::Rgb(106, 153, 85),
            COLOR_GRASS: Color::Rgb(181, 206, 168),
            COLOR_LIME: Color::Rgb(181, 206, 168),
            COLOR_YELLOW: Color::Rgb(220, 220, 170),
            COLOR_AMBER: Color::Rgb(206, 145, 120),
            COLOR_ORANGE: Color::Rgb(206, 145, 120),
            COLOR_GRAPEFRUIT: Color::Rgb(216, 120, 120),
            COLOR_BROWN: Color::Rgb(156, 128, 102),
            COLOR_DARK_RED: Color::Rgb(68, 32, 37),
            COLOR_LIGHT_GREEN_900: Color::Rgb(41, 58, 38),
            COLOR_GREY_50: Color::Rgb(212, 212, 212),
            COLOR_GREY_100: Color::Rgb(190, 190, 190),
            COLOR_GREY_200: Color::Rgb(160, 160, 160),
            COLOR_GREY_300: Color::Rgb(128, 128, 128),
            COLOR_GREY_400: Color::Rgb(106, 106, 106),
            COLOR_GREY_500: Color::Rgb(86, 86, 86),
            COLOR_GREY_600: Color::Rgb(65, 65, 65),
            COLOR_GREY_700: Color::Rgb(51, 51, 51),
            COLOR_GREY_800: Color::Rgb(60, 60, 60),
            COLOR_GREY_900: Color::Rgb(37, 37, 38),
            COLOR_GREY_950: Color::Rgb(30, 30, 30),
            COLOR_BORDER: Color::Rgb(60, 60, 60),
            COLOR_TEXT: Color::Rgb(212, 212, 212),
            COLOR_TEXT_SELECTED: Color::Rgb(212, 212, 212),
        }
    }

    pub const fn vscode_light() -> Self {
        Self {
            name: ThemeNames::VsCodeLight,
            COLOR_RED: Color::Rgb(163, 21, 21),
            COLOR_PINK: Color::Rgb(175, 0, 219),
            COLOR_PURPLE: Color::Rgb(175, 0, 219),
            COLOR_DURPLE: Color::Rgb(121, 94, 38),
            COLOR_INDIGO: Color::Rgb(0, 0, 255),
            COLOR_BLUE: Color::Rgb(0, 0, 255),
            COLOR_CYAN: Color::Rgb(38, 127, 153),
            COLOR_TEAL: Color::Rgb(0, 128, 128),
            COLOR_GREEN: Color::Rgb(0, 128, 0),
            COLOR_GRASS: Color::Rgb(9, 134, 88),
            COLOR_LIME: Color::Rgb(92, 145, 32),
            COLOR_YELLOW: Color::Rgb(121, 94, 38),
            COLOR_AMBER: Color::Rgb(181, 118, 20),
            COLOR_ORANGE: Color::Rgb(181, 118, 20),
            COLOR_GRAPEFRUIT: Color::Rgb(180, 75, 64),
            COLOR_BROWN: Color::Rgb(128, 96, 72),
            COLOR_DARK_RED: Color::Rgb(255, 232, 232),
            COLOR_LIGHT_GREEN_900: Color::Rgb(230, 248, 230),
            COLOR_GREY_50: Color::Rgb(30, 30, 30),
            COLOR_GREY_100: Color::Rgb(51, 51, 51),
            COLOR_GREY_200: Color::Rgb(85, 85, 85),
            COLOR_GREY_300: Color::Rgb(117, 117, 117),
            COLOR_GREY_400: Color::Rgb(150, 150, 150),
            COLOR_GREY_500: Color::Rgb(97, 97, 97),
            COLOR_GREY_600: Color::Rgb(198, 198, 198),
            COLOR_GREY_700: Color::Rgb(229, 229, 229),
            COLOR_GREY_800: Color::Rgb(238, 238, 238),
            COLOR_GREY_900: Color::Rgb(245, 245, 245),
            COLOR_GREY_950: Color::Rgb(255, 255, 255),
            COLOR_BORDER: Color::Rgb(229, 229, 229),
            COLOR_TEXT: Color::Rgb(30, 30, 30),
            COLOR_TEXT_SELECTED: Color::Rgb(30, 30, 30),
        }
    }

    pub fn presets() -> &'static [ThemePreset] {
        &THEME_PRESETS
    }
}

impl ThemeNames {
    pub const fn label(self) -> &'static str {
        match self {
            Self::Classic => "classic",
            Self::Ansi => "ansi",
            Self::Monochrome => "monochrome",
            Self::DraculaDark => "dracula dark",
            Self::DraculaLight => "dracula light",
            Self::MonokaiDark => "monokai dark",
            Self::MonokaiLight => "monokai light",
            Self::CatppuccinDark => "catppuccin dark",
            Self::CatppuccinLight => "catppuccin light",
            Self::AtomDark => "atom dark",
            Self::AtomLight => "atom light",
            Self::VsCodeDark => "vscode dark",
            Self::VsCodeLight => "vscode light",
        }
    }
}

#[derive(Clone, Copy)]
pub struct ThemePreset {
    pub label: &'static str,
    pub theme: Theme,
}

pub const THEME_PRESETS: [ThemePreset; 13] = [
    ThemePreset { label: "classic", theme: Theme::classic() },
    ThemePreset { label: "ansi", theme: Theme::ansi() },
    ThemePreset { label: "monochrome", theme: Theme::monochrome() },
    ThemePreset { label: "dracula dark", theme: Theme::dracula_dark() },
    ThemePreset { label: "dracula light", theme: Theme::dracula_light() },
    ThemePreset { label: "monokai dark", theme: Theme::monokai_dark() },
    ThemePreset { label: "monokai light", theme: Theme::monokai_light() },
    ThemePreset { label: "catppuccin dark", theme: Theme::catppuccin_dark() },
    ThemePreset { label: "catppuccin light", theme: Theme::catppuccin_light() },
    ThemePreset { label: "atom dark", theme: Theme::atom_dark() },
    ThemePreset { label: "atom light", theme: Theme::atom_light() },
    ThemePreset { label: "vscode dark", theme: Theme::vscode_dark() },
    ThemePreset { label: "vscode light", theme: Theme::vscode_light() },
];

fn theme_path() -> PathBuf {
    let mut path = dirs::config_dir().unwrap();
    path.push("guitar");
    path.push("theme.json");
    path
}

pub fn load_theme() -> Theme {
    let path = theme_path();
    if path.exists() {
        let contents = fs::read_to_string(&path).unwrap_or_default();
        let label = facet_json::from_str::<String>(&contents).unwrap_or_else(|_| contents.trim().trim_matches('"').to_string());
        if let Some(theme) = Theme::from_label(&label) {
            return theme;
        }
    }

    let theme = Theme::default();
    save_theme(&theme);
    theme
}

pub fn save_theme(theme: &Theme) {
    let path = theme_path();
    if let Some(parent) = path.parent()
        && !parent.exists()
    {
        let _ = fs::create_dir_all(parent);
    }

    let theme_string = facet_json::to_string(&theme.label().to_string()).unwrap();
    fs::write(&path, &theme_string).unwrap();
}

#[cfg(test)]
mod tests {
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
}
