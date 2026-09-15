use ratatui::{style::{Color, Style}, text::Text};

pub enum Stat {
    CONDITION,
    MOXIE,
    PRECISION,
    ATHLETICS,
    EMPATHY,
    INSIGHT,
    FORESIGHT,
}

impl Stat {
    pub const fn to_pri_string(&self) -> &'static str {
        match self {
            Self::CONDITION => "CON",
            Self::MOXIE => "MOX",
            Self::PRECISION => "PRE",
            Self::ATHLETICS => "ATH",
            Self::EMPATHY => "EMP",
            Self::INSIGHT => "INS",
            Self::FORESIGHT => "FOR",
        }
    }

    pub const fn to_sec_string(&self) -> &'static str {
        match self {
            Self::CONDITION => "Con",
            Self::MOXIE => "Mox",
            Self::PRECISION => "Pre",
            Self::ATHLETICS => "Ath",
            Self::EMPATHY => "Emp",
            Self::INSIGHT => "Ins",
            Self::FORESIGHT => "For",
        }
    }

    pub fn to_pri_text(&self) -> Text {
        Text::from(self.to_pri_string())
            .style(Style::default().fg(self.to_color()))
    }

    pub fn to_sec_text(&self) -> Text {
        Text::from(self.to_sec_string())
            .style(Style::default().fg(self.to_color()))
    }

    pub fn to_color(&self) -> Color {
        match self {
            Self::CONDITION => Color::Rgb(130, 254, 131),
            Self::MOXIE => Color::Rgb(255, 135, 175),
            Self::PRECISION => Color::Rgb(135, 215, 255),
            Self::ATHLETICS => Color::Rgb(255, 255, 0),
            Self::EMPATHY => Color::Rgb(175, 95, 255),
            Self::INSIGHT => Color::Rgb(255, 175, 135),
            Self::FORESIGHT => Color::Rgb(255, 175, 255),
        }
    }
}

