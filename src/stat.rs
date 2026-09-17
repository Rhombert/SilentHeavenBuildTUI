use ratatui::{style::{Color, Style}, text::Text};

#[derive(Clone, Copy)]
pub enum Stat {
    CONDITION,
    MOXIE,
    PRECISION,
    ATHLETICS,
    EMPATHY,
    INSIGHT,
    FORESIGHT,
}

pub const fn stat_to_pri_string(stat: &Stat) -> &'static str {
    match stat {
        Stat::CONDITION => "CON",
        Stat::MOXIE => "MOX",
        Stat::PRECISION => "PRE",
        Stat::ATHLETICS => "ATH",
        Stat::EMPATHY => "EMP",
        Stat::INSIGHT => "INS",
        Stat::FORESIGHT => "FOR",
    }
}

pub const fn stat_to_sec_string(stat: &Stat) -> &'static str {
    match stat {
        Stat::CONDITION => "Con",
        Stat::MOXIE => "Mox",
        Stat::PRECISION => "Pre",
        Stat::ATHLETICS => "Ath",
        Stat::EMPATHY => "Emp",
        Stat::INSIGHT => "Ins",
        Stat::FORESIGHT => "For",
    }
}

impl Stat {
    pub const fn from(value: usize) -> Self {
        match value {
            0 => Self::CONDITION,
            1 => Self::MOXIE,
            2 => Self::PRECISION,
            3 => Self::ATHLETICS,
            4 => Self::EMPATHY,
            5 => Self::INSIGHT,
            6 => Self::FORESIGHT,
            _ => Self::CONDITION,
        }
    }

    pub const fn index(self) -> usize { self as usize }

    pub const fn to_pri_string(&self) -> &'static str {
        stat_to_pri_string(self)
    }

    pub const fn to_sec_string(&self) -> &'static str {
        stat_to_sec_string(self)
    }

    pub fn to_pri_text(&self) -> Text {
        Text::from(self.to_pri_string())
            .style(Style::default().fg(self.to_color()))
    }

    pub fn to_sec_text(&self) -> Text {
        Text::from(String::from(" ") + self.to_sec_string())
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

