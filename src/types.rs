use crate::skill::Skill;
use crate::level::Level;

pub type Strengths = [Skill; 4];
pub type Weaknesses = [Skill; 3];
pub type CurrentLevels = [Level; 28];
pub type AspectCounts = [u32; 7];
pub type CostCounts = [u32; 28];
