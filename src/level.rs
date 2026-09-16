use std::convert::TryFrom;

use crate::skill::Skill;

pub enum SkillCategory {
    SURVIVAL,
    GENERAL,
    PROFESSIONAL,
    CONFLICT,
}

pub struct LevelPlan {
    steps: Vec<Skill>,
    levels: [Level; 28],
    aspects: [u32; 7],
    costs: [u32; 28],
    xp_total: u32,
    xp_locked: u32,
    xp_remaining: u32,
}

// Skil cost is
// current_rank * 10 BEFORE Good
// current_rank * 10 + 10 AT OR ABOVE Good

impl LevelPlan {
    pub fn new() -> Self {
        Self {
            steps: vec!(),
            levels: core::array::from_fn(|_| Level::JUVENILE),
            aspects: core::array::from_fn(|_| 0),
            costs: core::array::from_fn(|_| 10),
            xp_total: 2000,
            xp_locked: 0,
            xp_remaining: 2000,
        }
    }

    pub fn calculate(&mut self) {
        self.calculate_aspects();
        self.calculate_costs();
    }

    fn calculate_aspects(&mut self) {
        self.aspects.fill(0);

        for (i, level) in self.levels.iter().enumerate() {
            let skill = Skill::from(i);

            let primary = skill.get_pri_stat();
            let secondary = skill.get_sec_stat();

            self.aspects[primary.index()] += level.index() as u32;
            self.aspects[secondary.index()] += level.index() as u32;
        }
    }

    fn calculate_costs(&mut self) {
        self.costs.fill(0);

        for i in 0..self.costs.len() {
            let level = self.levels[i];
            let skill = Skill::from(i);
            let pri_stat = skill.get_pri_stat();
            let stat_count = self.aspects[pri_stat.index()];
            let rounded_stat_count = stat_count as f64 - (stat_count % 10) as f64;
            let discount = 1.00 - (rounded_stat_count / 100.0);
                                 

            let base_cost = level.level_up_cost() as f64;
            let discounted_cost = base_cost * discount;

            self.costs[i] = discounted_cost as u32;
        }
    }

    pub fn get_levels_chunk(&self, category: SkillCategory)  -> &[Level] {
        match category {
            SkillCategory::SURVIVAL => &self.levels[0..7],
            SkillCategory::GENERAL => &self.levels[7..14],
            SkillCategory::PROFESSIONAL => &self.levels[14..21],
            SkillCategory::CONFLICT => &self.levels[21..28],
        }
    }

    pub fn get_costs_chunk(&self, category: SkillCategory) -> &[u32] {
        match category {
            SkillCategory::SURVIVAL => &self.costs[0..7],
            SkillCategory::GENERAL => &self.costs[7..14],
            SkillCategory::PROFESSIONAL => &self.costs[14..21],
            SkillCategory::CONFLICT => &self.costs[21..28],
        }
    }

    pub fn level_skill(&mut self, skill: Skill) {
        self.steps.push(skill);
        let level = self.levels[skill.index()];
        self.levels[skill.index()] = level.increase();
    }

    pub fn delevel_skill(&mut self, skill: Skill) {
        match self.steps.iter().rposition(|s| *s == skill) {
            Some(idx) => { 
                self.steps.remove(idx); 
                let level = self.levels[skill.index()];
                self.levels[skill.index()] = level.decrease();
            },
            None => {},
        };
    }

    pub fn move_selection_up(&mut self, i: usize) {
        if i == 0 { return }
        if i >= self.steps.len() { return }

        self.steps.swap(i, i-1);
    }

    pub fn move_selection_down(&mut self, i: usize) {
        if i >= self.steps.len()-1 { return }

        self.steps.swap(i, i+1);
    }
}

#[derive(Default, Clone, Copy, PartialEq)]
pub enum Level {
    KLUTZ,
    #[default]
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

impl TryFrom<usize> for Level {
    type Error = ();

    fn try_from(i: usize) -> Result<Self, Self::Error> {
        match i {
            x if x == Level::KLUTZ as usize => Ok(Level::KLUTZ),
            x if x == Level::JUVENILE as usize => Ok(Level::JUVENILE),
            x if x == Level::INITIATE as usize => Ok(Level::INITIATE),
            x if x == Level::HOPEFUL as usize => Ok(Level::HOPEFUL),
            x if x == Level::GOOD as usize => Ok(Level::GOOD),
            x if x == Level::FOCUSED as usize => Ok(Level::FOCUSED),
            x if x == Level::EXPERIENCED as usize => Ok(Level::EXPERIENCED),
            x if x == Level::DEDICATED as usize => Ok(Level::DEDICATED),
            x if x == Level::COMPETITIVE as usize => Ok(Level::COMPETITIVE),
            x if x == Level::BRILLIANT as usize => Ok(Level::BRILLIANT),
            x if x == Level::ASTONISHING as usize => Ok(Level::ASTONISHING),
            _ => Err(()),
        }
    }
}

impl Level {
    const COUNT: usize = 11;

    pub const fn index(self) -> usize { self as usize }

    pub fn level_up_cost(&self) -> u32 {
        let level_int = self.index() as u32;

        if level_int < 4 { level_int * 10 }
        else { (level_int * 10) + 10 }
    }

    pub fn increase(self) -> Self {
        if self == Self::KLUTZ { return self }

        let current = self as usize + 1;

        if current >= Self::COUNT {
            return Self::ASTONISHING;
        }

        match Level::try_from(current) {
            Ok(level) => {
                level
            }
            Err(_) => self,
        }
    }

    pub fn decrease(self) -> Self {
        if self == Self::KLUTZ { return self }

        let mut current = self as usize;

        // KLUTZ can only be achieved through a 'weakness'
        if current == 1 {
            return Self::JUVENILE;
        }

        current -= 1;

        match Level::try_from(current) {
            Ok(level) => level,
            Err(_) => self,
        }
    }

    pub const fn to_string(&self) -> &'static str {
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
