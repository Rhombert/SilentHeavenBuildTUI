use std::convert::TryFrom;

use ratatui::text::Text;
use serde::{Deserialize, Serialize};

use crate::skill::{STRENGTHS, Skill, WEAKNESSES};
use crate::skill_levels::SkillLevels;
use crate::types::{ Strengths, Weaknesses, AspectCounts, CostCounts };


fn calculate_next_level_cost(
    level: Level,
    skill: Skill,
    aspects: &[u32],
) -> u32 {
    let base_cost = level.level_up_cost() as f64;
    let pri_stat = skill.get_pri_stat();
    let stat_count = aspects[pri_stat.index()];

    let rounded_stat_count = stat_count as f64 - (stat_count % 10) as f64;
    let mut discount = 1.00 - (rounded_stat_count / 100.0);
    if discount < 0.2 { discount = 0.2; }

    let final_cost = base_cost * discount;

    final_cost as u32
}

pub enum SkillCategory {
    SURVIVAL,
    GENERAL,
    PROFESSIONAL,
    CONFLICT,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct LevelStep {
    skill: Skill,
    level: Level,
    cost: u32,
}

impl LevelStep {
    pub fn new(skill: Skill, level: Level, cost: u32) -> Self {
        LevelStep { skill, level, cost }
    }

    pub fn ref_array(&self) -> [Text; 3] {
        [
            Text::from(self.skill.to_string()),
            Text::from(self.level.to_string()),
            Text::from(self.cost.to_string()),
        ]
    }

    pub fn calculate_base_cost(&self) -> u32 {
        self.level.level_up_cost()
    }

    pub fn calculate_final_cost(&self, aspects: &[u32]) -> u32 {
        calculate_next_level_cost(self.level.decrease(), self.skill, aspects)
    }
}

#[derive(Clone)]
pub struct LevelPlan {
    steps: Vec<LevelStep>,
    levels: SkillLevels,
    strengths: Strengths,
    weaknesses: Weaknesses,
    aspects: AspectCounts,
    costs: CostCounts,
    xp_total: u32,
    xp_locked: u32,
    xp_remaining: u32,
}

impl LevelPlan {
    pub fn new() -> Self {
        Self {
            steps: vec!(),
            levels: SkillLevels::new(),
            strengths: [Skill::NONE; 4],
            weaknesses: [Skill::NONE; 3],
            aspects: core::array::from_fn(|_| 0),
            costs: core::array::from_fn(|_| 10),
            xp_total: 2000,
            xp_locked: 0,
            xp_remaining: 2000,
        }
    }

    pub fn get_xp_total(&self) -> u32 { self.xp_total }
    pub fn get_xp_locked(&self) -> u32 { self.xp_locked }
    pub fn get_xp_remaining(&self) -> u32 { self.xp_remaining }

    pub fn get_steps(&self) -> &Vec<LevelStep> { &self.steps }
    pub fn set_steps(&mut self, steps: Vec<LevelStep>) { 
        self.steps = steps;
    }

    pub fn get_aspects(&self) -> &AspectCounts {
        return &self.aspects
    }

    pub fn calculate(&mut self) {
        self.levels.reset(self.strengths, self.weaknesses);

        // Called here to calculate the initial, no XP spent aspects.
        // This will vary based on character strengths and weaknesses.
        self.calculate_aspects();

        self.drop_weaknesses();
        self.recalculate_plan();

        // This needs to be called again after levels have been
        //  applied, to calculate the final aspects.
        self.calculate_aspects();

        self.calculate_costs();
        self.calculate_xp_usage();
    }

    fn drop_weaknesses(&mut self) {
        self.steps.retain(
            | step | {
               if self.weaknesses.contains(&step.skill) {
                   return false
               }
               true
            }
        )
    }

    fn recalculate_plan(&mut self) {
        let mut aspects: AspectCounts = self.aspects.clone();

        for i in 0..self.steps.len() {
            let step = &mut self.steps[i];

            self.levels.increase_skill(step.skill);
            step.level = self.levels[step.skill.index()];
            let step_cost = step.calculate_final_cost(&aspects);
            step.cost = step_cost;

            aspects[step.skill.get_pri_stat().index()] += 1;
            aspects[step.skill.get_sec_stat().index()] += 1;
        }
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

            self.costs[i] = calculate_next_level_cost(
                self.levels[i], 
                Skill::from(i), 
                &self.aspects,
            )
        }
    }

    fn calculate_xp_usage(&mut self) {
        self.xp_locked = 0;
        self.xp_remaining = self.xp_total;

        for step in &self.steps {
            self.xp_locked += step.cost;
        }

        self.xp_remaining -= self.xp_locked;
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
        let level = self.levels[skill.index()];

        if level == Level::ASTONISHING { return }
        if level == Level::KLUTZ { return }

        self.steps.push(LevelStep { 
            skill, 
            level: level.increase(),
            cost: self.costs[skill.index()],
        });
    }

    pub fn delevel_skill(&mut self, skill: Skill) {
        match self.steps.iter().rposition(|s| s.skill == skill) {
            Some(idx) => { 
                self.steps.remove(idx); 
            },
            None => {},
        };
    }

    pub fn get_strengths(&self) -> &Strengths { &self.strengths }
    pub fn get_weaknesses(&self) -> &Weaknesses { &self.weaknesses }

    pub fn set_strength(&mut self, skill: Skill) {
        for i in 0..STRENGTHS.len() {
            if STRENGTHS[i].contains(&skill) {
                self.strengths[i] = skill;
            }
        }
    }

    pub fn set_weakness(&mut self, skill: Skill) {
        for i in 0..WEAKNESSES.len() {
            if WEAKNESSES[i].contains(&skill) {
                if self.weaknesses[i] != Skill::NONE {
                    self.levels[self.weaknesses[i].index()]
                        = Level::JUVENILE;
                }

                self.weaknesses[i] = skill;
            }
        }
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

#[derive(Default, Clone, Copy, PartialEq, Serialize, Deserialize)]
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

        // KLUTZ can only be achieved through a 'weakness'
        if self == Self::JUVENILE {
            return Self::JUVENILE;
        }

        let mut current = self as usize;

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
