use std::cmp;

use ratatui::{Frame, layout::{Constraint, Rect}, style::{Color, Style}, widgets::{Cell, Row, Table, TableState}};

use crate::{level::{LevelPlan, LevelStep}, skill::Skill, traits::table_control::TableControl};

pub struct HistoryTable {
    state: TableState,
    items: Vec<LevelStep>,

    selection_idx: usize,
    visible_rows: usize,
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
        //let i = match self.state.selected() {
        //    Some(i) => {
        //        if i >= self.items.len() -1 {
        //            0
        //        } else {
        //            i + 1
        //        }
        //    }
        //    None => 0,
        //};
        //self.state.select(Some(i));
        if self.selection_idx >= self.items.len() - 1 {
            self.selection_idx = 0
        } else {
            self.selection_idx += 1
        }
    }

    fn previous_row(&mut self) {
        //let i = match self.state.selected() {
        //    Some(i) => {
        //        if i == 0 {
        //            self.items.len() - 1
        //        } else {
        //            i - 1
        //        }
        //    }
        //    None => 0,
        //};
        //self.state.select(Some(i));
        self.selection_idx = match self.selection_idx {
            0 => self.items.len() - 1,
            _ => self.selection_idx - 1
        };
    }
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
            items: vec!(),

            selection_idx: 0,
            visible_rows: 30,
        }
    }
    
    pub fn move_up(&mut self, level_plan: &mut LevelPlan) {
        level_plan.move_selection_up(self.selection_idx);
        self.previous_row();
    }

    pub fn move_down(&mut self, level_plan: &mut LevelPlan) {
        level_plan.move_selection_down(self.selection_idx);
        self.next_row();
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, items: &Vec<LevelStep>) {
        self.items = items.clone();

        let range_start = self.selection_idx.saturating_sub(self.visible_rows/2);
        let range_end = {
            if items.len() < self.visible_rows { items.len() }
            else { 
                cmp::min(
                    range_start + self.visible_rows,
                    items.len(),
                )
            }
        };

        self.state.select({
            if self.selection_idx < self.visible_rows/2 {
                Some(self.selection_idx)
            }
            else {
                Some(self.visible_rows/2)
            }
        });

        let rows = items[range_start..range_end]
            .iter().map( |steps| {

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
