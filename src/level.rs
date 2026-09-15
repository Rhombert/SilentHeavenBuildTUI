enum Level {
    KLUTZ,
    JUVENILE,
    INITIATE,
    HOPEFUL,
    GOOD,
    FOCUSED,
    EXPERIENCED,
    DEDICATED,
    COMPETITIVE,
    BRILLIANT,
    ASTONISHING,
}

impl Level {
    const fn to_string(&self) -> &'static str {
        match self {
            Level::KLUTZ => "Klutz",
            Level::JUVENILE => "Juvenile",
            Level::INITIATE => "Initiate",
            Level::HOPEFUL => "Hopeful",
            Level::GOOD => "Good",
            Level::FOCUSED => "Focused",
            Level::EXPERIENCED => "Experienced",
            Level::DEDICATED => "Dedicated",
            Level::COMPETITIVE => "Competitive",
            Level::BRILLIANT => "Brilliant",
            Level::ASTONISHING => "Astonishing",
        }
    }
}
