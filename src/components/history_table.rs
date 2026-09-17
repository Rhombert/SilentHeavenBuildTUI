use ratatui::{Frame, layout::{Constraint, Rect}, style::Style, widgets::{Cell, Row, Table, TableState}};

use crate::level::LevelStep;

pub struct HistoryTable {
    state: TableState,
}

impl HistoryTable {
    pub fn new() -> Self {
        Self {
            state: TableState::default(),
        }
    }

    pub fn render(&mut self, frame: &mut Frame, area: Rect, items: &Vec<LevelStep>) {
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
        );

        frame.render_stateful_widget(t, area, &mut self.state);
    }
}
