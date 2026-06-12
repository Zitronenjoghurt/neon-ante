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
    pub bg: Color,
    pub fg: Color,
    pub fg_dim: Color,
    pub accent: Color,
}

impl Theme {
    pub const MATRIX: Theme = Theme {
        bg: Color::Rgb(0x00, 0x00, 0x00),
        fg: neon::GREEN,
        fg_dim: Color::Rgb(0x00, 0x8F, 0x11),
        accent: neon::GREEN,
    };
}

impl Default for Theme {
    fn default() -> Self {
        Self::MATRIX
    }
}
