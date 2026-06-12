use ratatui::style::Color;

pub mod neon {
    use ratatui::style::Color;

    pub const GREEN: Color = Color::Rgb(0x00, 0xFF, 0x41);
    pub const PINK: Color = Color::Rgb(0xFF, 0x10, 0xF0);
    pub const CYAN: Color = Color::Rgb(0x04, 0xD9, 0xFF);
    pub const AMBER: Color = Color::Rgb(0xFF, 0xB0, 0x00);
    pub const PURPLE: Color = Color::Rgb(0xBC, 0x13, 0xFE);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Theme {
    pub name: &'static str,
    pub bg: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub accent: Color,
    pub selected: Color,
    pub suit_red: Color,
    pub suit_black: Color,
}

impl Theme {
    pub const MATRIX: Theme = Theme {
        name: "Matrix",
        bg: Color::Rgb(0x00, 0x00, 0x00),
        fg: neon::GREEN,
        fg_dim: Color::Rgb(0x00, 0x8F, 0x11),
        accent: neon::GREEN,
        selected: neon::AMBER,
        suit_red: neon::PINK,
        suit_black: neon::GREEN,
    };

    pub const AMBER: Theme = Theme {
        name: "Amber",
        bg: Color::Rgb(0x0A, 0x06, 0x00),
        fg: Color::Rgb(0xFF, 0xB0, 0x00),
        fg_dim: Color::Rgb(0x7A, 0x52, 0x00),
        accent: Color::Rgb(0xFF, 0xCC, 0x33),
        selected: Color::Rgb(0xFF, 0xFF, 0xFF),
        suit_red: Color::Rgb(0xFF, 0x5A, 0x3C),
        suit_black: Color::Rgb(0xFF, 0xB0, 0x00),
    };

    pub const ICE: Theme = Theme {
        name: "Ice",
        bg: Color::Rgb(0x00, 0x10, 0x1A),
        fg: Color::Rgb(0x9F, 0xE8, 0xFF),
        fg_dim: Color::Rgb(0x2A, 0x6A, 0x82),
        accent: neon::CYAN,
        selected: Color::Rgb(0xFF, 0xFF, 0xFF),
        suit_red: Color::Rgb(0xFF, 0x6E, 0xC7),
        suit_black: Color::Rgb(0x9F, 0xE8, 0xFF),
    };

    pub const SYNTHWAVE: Theme = Theme {
        name: "Synthwave",
        bg: Color::Rgb(0x16, 0x08, 0x22),
        fg: Color::Rgb(0xF8, 0x6E, 0xE0),
        fg_dim: Color::Rgb(0x6A, 0x2C, 0x7A),
        accent: neon::PURPLE,
        selected: neon::CYAN,
        suit_red: Color::Rgb(0xFF, 0x3E, 0x6E),
        suit_black: Color::Rgb(0xB8, 0x8C, 0xFF),
    };

    pub const PAPER: Theme = Theme {
        name: "Paper",
        bg: Color::Rgb(0xEC, 0xE6, 0xD6),
        fg: Color::Rgb(0x22, 0x22, 0x22),
        fg_dim: Color::Rgb(0x9A, 0x90, 0x80),
        accent: Color::Rgb(0x1A, 0x6F, 0xB0),
        selected: Color::Rgb(0xC0, 0x39, 0x2B),
        suit_red: Color::Rgb(0xC0, 0x39, 0x2B),
        suit_black: Color::Rgb(0x22, 0x22, 0x22),
    };

    pub const ALL: [Theme; 5] = [
        Self::MATRIX,
        Self::AMBER,
        Self::ICE,
        Self::SYNTHWAVE,
        Self::PAPER,
    ];
}

impl Default for Theme {
    fn default() -> Self {
        Self::MATRIX
    }
}
