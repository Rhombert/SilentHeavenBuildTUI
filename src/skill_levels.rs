use crate::skill::Skill;
use crate::level::Level;

#[derive(Default)]
struct SkillLevels {
    // Contains the current level for each
    skills: [Level; Skill::length()]
}

impl SkillLevels {
    fn increase_skill(&mut self, skill: Skill) {
        self.skills[skill.index()].increase();
    }

    fn decrease_skill(&mut self, skill: Skill) {
        self.skills[skill.index()].decrease();
    }
}
