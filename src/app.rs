use color_eyre::Result;

use crossterm::event::{self, KeyCode};
use ratatui::layout::{Constraint, Layout};
use ratatui::{DefaultTerminal, Frame};

use crate::components::skill_table::{SkillTable, generate_conflict_skills, generate_general_skills, generate_professional_skills, generate_survival_skills};
use crate::level::{LevelPlan, SkillCategory};


pub struct App {
    skill_tables: [SkillTable; 4],
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
            selected_table: 0,

            level_plan: LevelPlan::new(),
        }
    }

    fn decrement_selected_table(&mut self) {
        if self.selected_table == 0 { self.selected_table = 3; }
        else { self.selected_table -= 1; }
    }

    fn increment_selected_table(&mut self) {
        self.selected_table += 1;
        self.selected_table %= 4;
    }

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        self.skill_tables[0].set_selected(Some(0));

        loop {
            self.level_plan.calculate();

            terminal.draw(|frame| self.render(frame))?;

            if let Some(key) = event::read()?.as_key_press_event() {
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Enter => {
                        let selected_skill = self.skill_tables[self.selected_table]
                                                .selected_skill();
                        match selected_skill {
                            Some(skill) => self.level_plan.level_skill(skill),
                            None => {}
                        }
                    }
                    KeyCode::Backspace => {
                        let selected_skill = self.skill_tables[self.selected_table]
                                                .selected_skill();
                        match selected_skill {
                            Some(skill) => self.level_plan.delevel_skill(skill),
                            None => {}
                        }
                    }
                    KeyCode::Char('j') => self.skill_tables[self.selected_table].next_row(),
                    KeyCode::Char('k') => self.skill_tables[self.selected_table].previous_row(),
                    KeyCode::Char('h') => {
                        let selection = match self.skill_tables[self.selected_table]
                            .currently_selected() {
                            Some(s) => s,
                            _ => 0,
                        };
                        self.skill_tables[self.selected_table].set_selected(None);

                        self.decrement_selected_table();

                        self.skill_tables[self.selected_table].set_selected(Some(selection));
                    },
                    KeyCode::Char('l') => {
                        let selection = match self.skill_tables[self.selected_table]
                            .currently_selected() {
                            Some(s) => s,
                            _ => 0,
                        };
                        self.skill_tables[self.selected_table].set_selected(None);

                        self.increment_selected_table();

                        self.skill_tables[self.selected_table].set_selected(Some(selection));
                    },
                    _ => {},
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        // Create the four areas to render tables in.
        let skills_container = Layout::vertical([
            Constraint::Length(10),
            Constraint::Length(10),
        ]);
        let rects_container = frame.area().layout_vec(&skills_container);

        let blocks_top = Layout::horizontal([
            Constraint::Length(53),
            Constraint::Length(53),
        ]).split(rects_container[0]);
        let blocks_bottom = Layout::horizontal([
            Constraint::Length(53),
            Constraint::Length(53),
        ]).split(rects_container[1]);


        // Render tables into each area.
        self.skill_tables[0].render_table(
            frame, 
            blocks_top[0], 
            self.level_plan.get_levels_chunk(SkillCategory::SURVIVAL),
            self.level_plan.get_costs_chunk(SkillCategory::SURVIVAL),
        );
        self.skill_tables[1].render_table(
            frame, 
            blocks_top[1], 
            self.level_plan.get_levels_chunk(SkillCategory::GENERAL),
            self.level_plan.get_costs_chunk(SkillCategory::GENERAL),
        );
        self.skill_tables[2].render_table(
            frame, 
            blocks_bottom[0], 
            self.level_plan.get_levels_chunk(SkillCategory::PROFESSIONAL),
            self.level_plan.get_costs_chunk(SkillCategory::PROFESSIONAL),
        );
        self.skill_tables[3].render_table(
            frame, 
            blocks_bottom[1], 
            self.level_plan.get_levels_chunk(SkillCategory::CONFLICT),
            self.level_plan.get_costs_chunk(SkillCategory::CONFLICT),
        );
    }
}
