use ratatui::{Frame, layout::{Constraint, Rect}, style::Style, text::Text, widgets::{Row, Table}};

use crate::stat::{Stat, stat_to_pri_string};

pub struct AspectList {
}

impl AspectList {
    pub fn render(frame: &mut Frame, area: Rect, aspects: &[u32]) {
        let rows = aspects.iter().enumerate().map(
            |(i, value)| {
                let stat = Stat::from(i);
                Row::new([
                    Text::from(String::from(" ") + stat_to_pri_string(&stat))
                        .style(Style::default().fg(stat.to_color())),
                    Text::from(value.to_string()),
                ])
            }
        );
        let t = Table::new(
            rows,
            [
                Constraint::Length(4),
                Constraint::Length(3),
            ]
        );

        frame.render_widget(t, area);
    }
}
