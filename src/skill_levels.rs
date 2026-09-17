use std::ops::{Index, IndexMut, Range};

use crate::skill::Skill;
use crate::level::Level;
use crate::types::{Strengths, Weaknesses};

#[derive(Default)]
pub struct SkillLevels {
    // Contains the current level for each
    levels: [Level; Skill::length()]
}

impl SkillLevels {
    pub fn new() -> Self {
        Self {
            levels: [Level::JUVENILE; 28],
        }
    }

    pub fn reset(&mut self, strengths: Strengths, weaknesses: Weaknesses) {
        self.levels = [Level::JUVENILE; 28];
        self.process_strengths(strengths);
        self.process_weaknesses(weaknesses);
    }

    fn process_strengths(&mut self, strengths: Strengths) {
        for strength in strengths {
            if strength == Skill::NONE { continue }
            self.levels[strength as usize] = Level::GOOD;
        }
    }

    fn process_weaknesses(&mut self, weaknesses: Weaknesses) {
        for weakness in weaknesses {
            if weakness == Skill::NONE { continue }
            self.levels[weakness as usize] = Level::KLUTZ;
        }
    }

    pub fn increase_skill(&mut self, skill: Skill) {
        self.levels[skill.index()] = self.levels[skill.index()].increase();
    }

    pub fn decrease_skill(&mut self, skill: Skill) {
        self.levels[skill.index()] = self.levels[skill.index()].decrease();
    }

    pub fn iter(&self) -> std::slice::Iter<'_, Level> {
        self.levels.iter()
    }
}

impl Index<usize> for SkillLevels {
    type Output = Level;

    fn index(&self, index: usize) -> &Self::Output {
        &self.levels[index]
    }
}

impl Index<Range<usize>> for SkillLevels {
    type Output = [Level];

    fn index(&self, range: Range<usize>) -> &Self::Output {
        &self.levels[range]
    }
}

impl IndexMut<usize> for SkillLevels {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        &mut self.levels[index]
    }
}

impl<'a> IntoIterator for &'a SkillLevels {
    type Item = &'a Level;
    type IntoIter = std::slice::Iter<'a, Level>;

    fn into_iter(self) -> Self::IntoIter {
        self.levels.iter()
    }
}
