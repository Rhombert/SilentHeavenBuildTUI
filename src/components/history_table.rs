use ratatui::{Frame, layout::{Constraint, Rect}, style::{Color, Style}, widgets::{Cell, Row, Table, TableState}};

use crate::{level::{LevelPlan, LevelStep}, skill::Skill, traits::table_control::TableControl};

pub struct HistoryTable {
    state: TableState,
    items: Vec<LevelStep>,
}

impl TableControl for HistoryTable {
    fn selected_skill(&self) -> Option<Skill> {
        None
    }

    fn currently_selected(&self) -> Option<usize> {
        self.state.selected()
    }

    fn set_selected(&mut self, idx: Option<usize>) {
        self.state.select(idx);
    }

    fn next_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i >= self.items.len() -1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }

    fn previous_row(&mut self) {
        let i = match self.state.selected() {
            Some(i) => {
                if i == 0 {
                    self.items.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.state.select(Some(i));
    }
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: vec!(),
        }
    }
    
    pub fn move_up(&mut self, level_plan: &mut LevelPlan) {
        match self.state.selected() {
            Some(idx) => {
                level_plan.move_selection_up(idx);

                self.previous_row();
            }
            None => {},
        }
    }

    pub fn move_down(&mut self, level_plan: &mut LevelPlan) {
        match self.state.selected() {
            Some(idx) => {
                level_plan.move_selection_down(idx);

                self.next_row();
            }
            None => {},
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, items: &Vec<LevelStep>) {
        self.items = items.clone();

        let rows = items.iter().map( |steps| {
            let step = steps.ref_array();
            
            step.into_iter()
                .map(|content| {
                    Cell::from(content)
                })
                .collect::<Row>()
                .style(Style::new())
                .height(1)
        });

        let t = Table::new(
            rows,
            [
                Constraint::Length(12),
                Constraint::Length(12),
                Constraint::Length(12),
            ]
        )
        .row_highlight_style(Style::default().bg(Color::DarkGray));

        frame.render_stateful_widget(t, area, &mut self.state);
    }
}
