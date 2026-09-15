use crate::stat::Stat;

#[derive(Clone)]
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
}

impl Skill {
    pub const fn length() -> usize { 28 }

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
        }
    }

    pub const fn get_pri_stat(&self) -> Stat {
        self.get_stats().1
    }

    pub const fn get_sec_stat(&self) -> Stat {
        self.get_stats().0
    }
}

