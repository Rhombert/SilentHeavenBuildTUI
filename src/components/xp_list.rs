use ratatui::{Frame, layout::{Constraint, Rect}, widgets::{Row, Table}};

pub struct XPList {
}

impl XPList {
    pub fn render(frame: &mut Frame, area: Rect, 
                  xp_total: u32, xp_locked: u32, xp_remaining: u32) {
        let rows = [
            Row::new([String::from("total:"), xp_total.to_string()]),
            Row::new([String::from("locked:"), xp_locked.to_string()]),
            Row::new([String::from("remaining:"), xp_remaining.to_string()]),
        ];
        let t = Table::new(
            rows,
            [
                Constraint::Length(10),
                Constraint::Length(4),
            ]
        );

        frame.render_widget(t, area);
    }
}
