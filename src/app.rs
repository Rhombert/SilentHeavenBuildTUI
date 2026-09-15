use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Cell, HighlightSpacing, Row, Table, TableState}};

use crate::skill::Skill;
use crate::stat::Stat;

struct SkillLevels {
    // Contains the current level for each
    skills: [usize; Skill::length()]
}

struct Data {
    stat_primary: Stat,
    stat_secondary: Stat,
    skill: Skill,
}

impl Data {
    fn new(skill: &Skill) -> Self {
        Data {
            skill: skill.clone(),
            stat_primary: skill.get_pri_stat(),
            stat_secondary: skill.get_sec_stat(),
        }
    }

    fn ref_array(&self) -> [Text; 3] {
        [
            self.stat_primary.to_pri_text(),
            self.stat_secondary.to_sec_text(),
            Text::from(self.skill.to_string()),
        ]
    }

    fn stat_primary(&self) -> &Stat {
        &self.stat_primary
    }

    fn stat_secondary(&self) -> &Stat {
        &self.stat_secondary
    }

    fn skill(&self) -> &Skill {
        &self.skill
    }
}

pub struct App {
    state: TableState,
    items: Vec<Data>,
}

// Should be able to move the table rendering functionality into its own
// impl, and have the App call multiple with Rects it predetermines.
impl App {
    pub fn new() -> Self {
        let data_vec = generate_general_skills();
        Self {
            state: TableState::default().with_selected(0),
            items: data_vec,
        }
    }

    pub const fn next_row(&mut self) {
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

    pub const fn previous_row(&mut self) {
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

    pub fn run(mut self, terminal: &mut DefaultTerminal) -> Result<()> {
        loop {
            terminal.draw(|frame| self.render(frame))?;


            if let Some(key) = event::read()?.as_key_press_event() {
                // let shift_pressed = key.modifiers.contains(KeyModifiers::SHIFT);
                match key.code {
                    KeyCode::Char('q') | KeyCode::Esc => return Ok(()),
                    KeyCode::Char('j') => self.next_row(),
                    KeyCode::Char('k') => self.previous_row(),
                    _ => {},
                }
            }
        }
    }

    fn render(&mut self, frame: &mut Frame) {
        let layout = Layout::vertical([Constraint::Min(5), Constraint::Length(4)]);
        let rects = frame.area().layout_vec(&layout);

        self.render_table(frame, rects[0]);
    }

    fn render_table(&mut self, frame: &mut Frame, area: Rect) {
        let header_style = Style::default();
        let selected_row_style = Style::default()
            .bg(Color::DarkGray);
        let header = ["(Sec", "PRI)", "NAME"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style);

        let rows = self.items.iter().enumerate().map(|(i, data)| {
            let item = data.ref_array();
            item.into_iter()
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
                Constraint::Length(4),
                Constraint::Length(5),
                Constraint::Length(12),
            ],
            )
            .header(header)
            .row_highlight_style(selected_row_style)
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(t, area, &mut self.state);
    }
}

fn generate_survival_skills() -> Vec<Data> {
    vec![
        Data::new(&Skill::HEALTH),
        Data::new(&Skill::SEARCH),
        Data::new(&Skill::STEALTH),
        Data::new(&Skill::MEDICINE),
        Data::new(&Skill::DECEPTION),
        Data::new(&Skill::CLIMB),
        Data::new(&Skill::DILIGENCE),
    ]
}

fn generate_general_skills() -> Vec<Data> {
    vec![
        Data::new(&Skill::STAMINA),
        Data::new(&Skill::EAVESDROP),
        Data::new(&Skill::SPRINT),
        Data::new(&Skill::CHRONICLE),
        Data::new(&Skill::APPEARANCE),
        Data::new(&Skill::LABOR),
        Data::new(&Skill::OCCULT),
    ]
}

fn generate_professional_skills() -> Vec<Data> {
    vec![
        Data::new(&Skill::TRAVEL),
        Data::new(&Skill::INVESTIGATE),
        Data::new(&Skill::REPAIR),
        Data::new(&Skill::CHEMISTRY),
        Data::new(&Skill::COOKING),
        Data::new(&Skill::CREATIVITY),
        Data::new(&Skill::TALENT),
    ]
}

fn generate_conflict_skills() -> Vec<Data> {
    vec![
        Data::new(&Skill::FISTS),
        Data::new(&Skill::GUNS),
        Data::new(&Skill::BLADES),
        Data::new(&Skill::PERFORMANCE),
        Data::new(&Skill::TOXINS),
        Data::new(&Skill::MELEE),
        Data::new(&Skill::ESCAPE),
    ]
}
