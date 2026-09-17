use crate::skill::Skill;

pub trait TableControl {
    fn selected_skill(&self) -> Option<Skill>;
    fn currently_selected(&self) -> Option<usize>;
    fn set_selected(&mut self, idx: Option<usize>);
    fn next_row(&mut self);
    fn previous_row(&mut self);
}
