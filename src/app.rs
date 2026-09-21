use std::rc::Rc;

use color_eyre::Result;

use crossterm::event::{self, KeyCode, KeyModifiers};
use ratatui::layout::{Constraint, Layout, Rect};
use ratatui::{DefaultTerminal, Frame};

use crate::components::history_table::HistoryTable;
use crate::components::{
    aspect_list::AspectList,
    xp_list::XPList,
};
use crate::components::skill_table::{SkillTable, generate_conflict_skills, generate_general_skills, generate_professional_skills, generate_survival_skills};
use crate::fs::data::SaveData;
use crate::level::{LevelPlan, SkillCategory};
use crate::traits::table_control::TableControl;


pub struct App {
    skill_tables: [SkillTable; 4],
    history_table: HistoryTable,
    selected_table: usize,

    level_plan: LevelPlan,
}

impl App {
    pub fn new() -> Self {
        Self {
            skill_tables: [
                SkillTable::new(generate_survival_skills()),
                SkillTable::new(generate_general_skills()),
                SkillTable::new(generate_professional_skills()),
                SkillTable::new(generate_conflict_skills()),
            ],
            history_table: HistoryTable::new(),
            selected_table: 0,
            level_plan: LevelPlan::new(),
        }
    }

    fn get_table_control(&mut self, idx: usize) -> Option<&mut dyn TableControl> {
        match idx {
            0..4 => Some(&mut self.skill_tables[idx]),
            4 => Some(&mut self.history_table),
            _ => None,
        }
    }

    fn decrement_selected_table(&mut self) {
        if self.selected_table == 0 { self.selected_table = 4; }
        else { self.selected_table -= 1; }
    }

    fn increment_selected_table(&mut self) {
        self.selected_table += 1;
        self.selected_table %= 5;
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.skill_tables[0].set_selected(Some(0));

        loop {
            self.level_plan.calculate();

            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('1') => {
                        // Set Strength
                        let selected_skill = self.skill_tables[self.selected_table]
                                                .selected_skill();
                        match selected_skill {
                            Some(skill) => self.level_plan.set_strength(skill),
                            None => {},
                        }
                    }
                    KeyCode::Char('2') => {
                        // Set Weakness
                        let selected_skill = self.skill_tables[self.selected_table]
                                                .selected_skill();
                        match selected_skill {
                            Some(skill) => self.level_plan.set_weakness(skill),
                            None => {},
                        }
                    }
                    KeyCode::Char('a') => {
                        if self.selected_table != 4 {
                            let selected_skill = self.skill_tables[self.selected_table]
                                                    .selected_skill();
                            match selected_skill {
                                Some(skill) => self.level_plan.level_skill(skill),
                                None => {},
                            }
                        }
                    }
                    KeyCode::Char('s') => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            let save_data = SaveData::copy_plan(&self.level_plan);
                            save_data.save_to("test.sav");
                        } else {
                            if self.selected_table != 4 {
                                let selected_skill = self.skill_tables[self.selected_table]
                                                        .selected_skill();
                                match selected_skill {
                                    Some(skill) => self.level_plan.delevel_skill(skill),
                                    None => {},
                                }
                            }
                        }
                    }
                    KeyCode::Char('c') => {
                        if self.selected_table == 4 {
                            self.history_table.lock_selection(&mut self.level_plan);
                        }
                    }
                    KeyCode::Char('J') => {
                        if self.selected_table == 4 {
                            self.history_table.move_down(&mut self.level_plan);
                        }
                    }
                    KeyCode::Char('j') => {
                        match self.get_table_control(self.selected_table) {
                            Some(tc) => tc.next_row(),
                            None => {},
                        }
                    }
                    KeyCode::Char('K') => {
                        if self.selected_table == 4 {
                            self.history_table.move_up(&mut self.level_plan);
                        }
                    }
                    KeyCode::Char('k') => {
                        match self.get_table_control(self.selected_table) {
                            Some(tc) => tc.previous_row(),
                            None => {},
                        }
                    }
                    KeyCode::Char('h') => {
                        let previous_tc = self.get_table_control(self.selected_table);
                        // dbg!(previous_tc.is_some());
                        if previous_tc.is_some() {
                            let selection = previous_tc.unwrap().currently_selected();

                            match self.get_table_control(self.selected_table) {
                                Some(tc) => tc.set_selected(None),
                                None => {},
                            }

                            self.decrement_selected_table();

                            match self.get_table_control(self.selected_table) {
                                Some(tc) => tc.set_selected(selection),
                                None => {},
                            }
                        }
                    },
                    KeyCode::Char('l') => {
                        if key.modifiers.contains(KeyModifiers::CONTROL) {
                            let data = SaveData::load_from("test.sav");
                            match data {
                                Ok(data) => {
                                    self.level_plan.set_steps(data.plan);
                                    for strength in data.strengths {
                                        self.level_plan.set_strength(strength);
                                    }
                                    for weakness in data.weaknesses {
                                        self.level_plan.set_weakness(weakness);
                                    }
                                }
                                Err(e) => {
                                }
                            };
                        } else {
                            let previous_tc = self.get_table_control(self.selected_table);
                            if previous_tc.is_some() {
                                let selection = previous_tc.unwrap().currently_selected();

                                match self.get_table_control(self.selected_table) {
                                    Some(tc) => tc.set_selected(None),
                                    None => {},
                                }

                                self.increment_selected_table();

                                match self.get_table_control(self.selected_table) {
                                    Some(tc) => tc.set_selected(selection),
                                    None => {},
                                }
                            }
                        }
                    },
                    _ => {},
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let main_container = Layout::vertical([
            Constraint::Max(20),
            Constraint::Fill(1),
        ]);

        let main_rects = frame.area().layout_vec(&main_container);

        // Create the four areas to render tables in.
        let skills_container = Layout::vertical([
            Constraint::Length(10),
            Constraint::Length(10),
        ]).split(main_rects[0]);

        let bottom_panes = Layout::horizontal([
            Constraint::Length(20),
            Constraint::Fill(1),
        ]).split(main_rects[1]);


        let left_bottom_pane_rects = Layout::vertical([
            Constraint::Length(8),
            Constraint::Length(3),
        ]).split(bottom_panes[0]);


        self.render_skills(frame, skills_container);
        self.render_aspects(frame, left_bottom_pane_rects[0]);
        self.render_xp(frame, left_bottom_pane_rects[1]);
        self.render_history(frame, bottom_panes[1]);
    }

    fn render_skills(&mut self, frame: &mut Frame, skills_container: Rc<[Rect]>) {
        let blocks_top = Layout::horizontal([
            Constraint::Length(53),
            Constraint::Length(53),
        ]).split(skills_container[0]);
        let blocks_bottom = Layout::horizontal([
            Constraint::Length(53),
            Constraint::Length(53),
        ]).split(skills_container[1]);

        self.skill_tables[0].render_table(
            frame, 
            blocks_top[0], 
            self.level_plan.get_levels_chunk(SkillCategory::SURVIVAL),
            self.level_plan.get_costs_chunk(SkillCategory::SURVIVAL),
            self.level_plan.get_strengths(),
            self.level_plan.get_weaknesses(),
        );
        self.skill_tables[1].render_table(
            frame, 
            blocks_top[1], 
            self.level_plan.get_levels_chunk(SkillCategory::GENERAL),
            self.level_plan.get_costs_chunk(SkillCategory::GENERAL),
            self.level_plan.get_strengths(),
            self.level_plan.get_weaknesses(),
        );
        self.skill_tables[2].render_table(
            frame, 
            blocks_bottom[0], 
            self.level_plan.get_levels_chunk(SkillCategory::PROFESSIONAL),
            self.level_plan.get_costs_chunk(SkillCategory::PROFESSIONAL),
            self.level_plan.get_strengths(),
            self.level_plan.get_weaknesses(),
        );
        self.skill_tables[3].render_table(
            frame, 
            blocks_bottom[1], 
            self.level_plan.get_levels_chunk(SkillCategory::CONFLICT),
            self.level_plan.get_costs_chunk(SkillCategory::CONFLICT),
            self.level_plan.get_strengths(),
            self.level_plan.get_weaknesses(),
        );
    }

    fn render_aspects(&mut self, frame: &mut Frame, container: Rect) {
        AspectList::render(frame, container, self.level_plan.get_aspects());
    }

    fn render_xp(&mut self, frame: &mut Frame, container: Rect) {
        XPList::render(
            frame, container, 
            self.level_plan.get_xp_total(),
            self.level_plan.get_xp_locked(),
            self.level_plan.get_xp_remaining(),
        );
    }

    fn render_history(&mut self, frame: &mut Frame, container: Rect) {
        self.history_table.render(frame, container, self.level_plan.get_steps());
    }
}
