use std::sync::LazyLock;

use crate::stat::Stat;

pub const STRENGTHS: LazyLock<Vec<Vec<Skill>>> = LazyLock::new(|| {
    vec![
        vec![
            Skill::APPEARANCE,
            Skill::CLIMB,
            Skill::HEALTH,
            Skill::OCCULT,
            Skill::SPRINT,
            Skill::STEALTH,
        ],
        vec![
            Skill::BLADES,
            Skill::ESCAPE,
            Skill::FISTS,
            Skill::GUNS,
            Skill::MELEE,
            Skill::TOXINS,
        ],
        vec![
            Skill::DECEPTION,
            Skill::EAVESDROP,
            Skill::LABOR,
            Skill::REPAIR,
            Skill::SEARCH,
            Skill::STAMINA,
            Skill::TRAVEL,
        ],
        vec![
            Skill::CHEMISTRY,
            Skill::CHRONICLE,
            Skill::COOKING,
            Skill::CREATIVITY,
            Skill::DILIGENCE,
            Skill::INVESTIGATE,
            Skill::MEDICINE,
            Skill::PERFORMANCE,
            Skill::TALENT,
        ],
    ]
});

pub const WEAKNESSES: LazyLock<Vec<Vec<Skill>>> = LazyLock::new(|| {
    vec![
        vec![
            Skill::APPEARANCE,
            Skill::BLADES,
            Skill::DILIGENCE,
            Skill::EAVESDROP,
            Skill::GUNS,
            Skill::HEALTH,
            Skill::LABOR,
            Skill::MELEE,
            Skill::SPRINT,
            Skill::STAMINA,
        ],
        vec![
            Skill::COOKING,
            Skill::CREATIVITY,
            Skill::ESCAPE,
            Skill::FISTS,
            Skill::INVESTIGATE,
            Skill::OCCULT,
            Skill::PERFORMANCE,
            Skill::REPAIR,
            Skill::SEARCH,
            Skill::TALENT,
        ],
        vec![
            Skill::CHEMISTRY,
            Skill::CHRONICLE,
            Skill::CLIMB,
            Skill::DECEPTION,
            Skill::MEDICINE,
            Skill::STEALTH,
            Skill::TRAVEL,
            Skill::TOXINS,
        ],
    ]
});

#[derive(Clone, Copy, PartialEq)]
pub enum Skill {
    HEALTH,
    SEARCH,
    STEALTH,
    MEDICINE,
    DECEPTION,
    CLIMB,
    DILIGENCE,

    STAMINA,
    EAVESDROP,
    SPRINT,
    CHRONICLE,
    APPEARANCE,
    LABOR,
    OCCULT,

    TRAVEL,
    INVESTIGATE,
    REPAIR,
    CHEMISTRY,
    COOKING,
    CREATIVITY,
    TALENT,

    FISTS,
    GUNS,
    BLADES,
    PERFORMANCE,
    TOXINS,
    MELEE,
    ESCAPE,

    NONE,
}

impl Skill {
    pub fn from(number: usize) -> Self {
        match number {
            0 => Self::HEALTH,
            1 => Self::SEARCH,
            2 => Self::STEALTH,
            3 => Self::MEDICINE,
            4 => Self::DECEPTION,
            5 => Self::CLIMB,
            6 => Self::DILIGENCE,

            7 => Self::STAMINA,
            8 => Self::EAVESDROP,
            9 => Self::SPRINT,
            10 => Self::CHRONICLE,
            11 => Self::APPEARANCE,
            12 => Self::LABOR,
            13 => Self::OCCULT,

            14 => Self::TRAVEL,
            15 => Self::INVESTIGATE,
            16 => Self::REPAIR,
            17 => Self::CHEMISTRY,
            18 => Self::COOKING,
            19 => Self::CREATIVITY,
            20 => Self::TALENT,

            21 => Self::FISTS,
            22 => Self::GUNS,
            23 => Self::BLADES,
            24 => Self::PERFORMANCE,
            25 => Self::TOXINS,
            26 => Self::MELEE,
            27 => Self::ESCAPE,

            _ => Self::HEALTH,
        }
    }

    pub const fn length() -> usize { 28 }

    pub const fn index(self) -> usize { self as usize }

    pub const fn to_string(&self) -> &'static str {
        match self {
            Self::HEALTH => "Health",
            Self::SEARCH => "Search",
            Self::STEALTH => "Stealth",
            Self::MEDICINE => "Medicine",
            Self::DECEPTION => "Deception",
            Self::CLIMB => "Climb",
            Self::DILIGENCE => "Diligence",

            Self::STAMINA => "Stamina",
            Self::EAVESDROP => "Eavesdrop",
            Self::SPRINT => "Sprint",
            Self::CHRONICLE => "Chronicle",
            Self::APPEARANCE => "Appearance",
            Self::LABOR => "Labor",
            Self::OCCULT => "Occult",

            Self::TRAVEL => "Travel",
            Self::INVESTIGATE => "Investigate",
            Self::REPAIR => "Repair",
            Self::CHEMISTRY => "Chemistry",
            Self::COOKING => "Cooking",
            Self::CREATIVITY => "Creativity",
            Self::TALENT => "Talent",

            Self::FISTS => "Fists",
            Self::GUNS => "Guns",
            Self::BLADES => "Blades",
            Self::PERFORMANCE => "Performance",
            Self::TOXINS => "Toxins",
            Self::MELEE => "Melee",
            Self::ESCAPE => "Escape",

            Self::NONE => "None",
        }
    }

    pub const fn get_stats(&self) -> (Stat, Stat) {
        match self {
            Self::HEALTH => (Stat::CONDITION, Stat::CONDITION),
            Self::SEARCH => (Stat::FORESIGHT, Stat::PRECISION),
            Self::STEALTH => (Stat::EMPATHY, Stat::ATHLETICS),
            Self::MEDICINE => (Stat::PRECISION, Stat::INSIGHT),
            Self::DECEPTION => (Stat::CONDITION, Stat::EMPATHY),
            Self::CLIMB => (Stat::ATHLETICS, Stat::MOXIE),
            Self::DILIGENCE => (Stat::FORESIGHT, Stat::FORESIGHT),

            Self::STAMINA => (Stat::ATHLETICS, Stat::CONDITION),
            Self::EAVESDROP => (Stat::EMPATHY, Stat::PRECISION),
            Self::SPRINT => (Stat::ATHLETICS, Stat::ATHLETICS),
            Self::CHRONICLE => (Stat::EMPATHY, Stat::INSIGHT),
            Self::APPEARANCE => (Stat::FORESIGHT, Stat::EMPATHY),
            Self::LABOR => (Stat::CONDITION, Stat::MOXIE),
            Self::OCCULT => (Stat::FORESIGHT, Stat::FORESIGHT),

            Self::TRAVEL => (Stat::INSIGHT, Stat::CONDITION),
            Self::INVESTIGATE => (Stat::PRECISION, Stat::PRECISION),
            Self::REPAIR => (Stat::INSIGHT, Stat::ATHLETICS),
            Self::CHEMISTRY => (Stat::INSIGHT, Stat::INSIGHT),
            Self::COOKING => (Stat::FORESIGHT, Stat::INSIGHT),
            Self::CREATIVITY => (Stat::EMPATHY, Stat::EMPATHY),
            Self::TALENT => (Stat::FORESIGHT, Stat::FORESIGHT),

            Self::FISTS => (Stat::PRECISION, Stat::CONDITION),
            Self::GUNS => (Stat::MOXIE, Stat::PRECISION),
            Self::BLADES => (Stat::PRECISION, Stat::ATHLETICS),
            Self::PERFORMANCE => (Stat::MOXIE, Stat::EMPATHY),
            Self::TOXINS => (Stat::INSIGHT, Stat::MOXIE),
            Self::MELEE => (Stat::MOXIE, Stat::MOXIE),
            Self::ESCAPE => (Stat::FORESIGHT, Stat::FORESIGHT),

            Self::NONE => (Stat::MOXIE, Stat::MOXIE),
        }
    }

    pub const fn get_pri_stat(&self) -> Stat {
        self.get_stats().1
    }

    pub const fn get_sec_stat(&self) -> Stat {
        self.get_stats().0
    }
}

