use color_eyre::Result;
use crossterm::event::{self, KeyCode};
use ratatui::{DefaultTerminal, Frame, layout::{Constraint, Layout, Rect}, style::{Color, Style}, text::Text, widgets::{Cell, HighlightSpacing, Row, Table, TableState}};

use crate::{level::Level, skill::Skill, types::{Strengths, Weaknesses}};
use crate::stat::Stat;

pub struct Data {
    stat_primary: Stat,
    stat_secondary: Stat,
    skill: Skill,
    rank: Level,
    cost: u32,
}

impl Data {
    pub fn new(skill: &Skill) ->Self {
        Data {
            skill: skill.clone(),
            stat_primary: skill.get_pri_stat(),
            stat_secondary: skill.get_sec_stat(),
            rank: Level::JUVENILE,
            cost: 10,
        }
    }

    fn formatted_array(&self, 
                       strengths: &Strengths, 
                       weaknesses: &Weaknesses) -> [Text; 5] {
        let mut arr = [
            self.stat_secondary.to_sec_text(),
            self.stat_primary.to_pri_text(),
            Text::from(self.skill.to_string()),
            Text::from(self.rank.to_string()),
            Text::from(self.cost.to_string()),
        ];

        if self.rank == Level::KLUTZ {
            arr[4] = Text::from("weakness")
                .style(Style::default().fg(Color::DarkGray));
        }
        
        if strengths.contains(&self.skill) {
            arr[3] = arr[3].clone().style(Style::default().fg(Color::Yellow));
        }
        if weaknesses.contains(&self.skill) {
            arr[3] = arr[3].clone().style(Style::default().fg(Color::DarkGray));
        }

        arr
    }

    fn ref_array(&self) -> [Text; 5] {
        [
            self.stat_secondary.to_sec_text(),
            self.stat_primary.to_pri_text(),
            Text::from(self.skill.to_string()),
            Text::from(self.rank.to_string()),
            Text::from(self.cost.to_string()),
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

pub struct SkillTable {
    state: TableState,
    items: Vec<Data>,
}

// Should be able to move the table rendering functionality into its own
// impl, and have the App call multiple with Rects it predetermines.
impl SkillTable {
    pub fn new(skills: Vec<Data>) -> Self {
        Self {
            state: TableState::default(),
            items: skills,
        }
    }

    pub fn update_levels(&mut self, levels: &[Level]) {
        if levels.len() != 7 { return; }

        for (i, level) in levels.iter().enumerate() {
            self.items[i].rank = *level;
        }
    }

    pub fn update_costs(&mut self, costs: &[u32]) {
        if costs.len() != 7 { return; }

        for (i, cost) in costs.iter().enumerate() {
            self.items[i].cost = *cost;
        }
    }

    pub fn selected_skill(&self) -> Option<Skill> {
        match self.state.selected() {
            Some(s) => Some(self.items[s].skill),
            None => None,
        }
    }

    pub fn currently_selected(&self) -> Option<usize> {
        self.state.selected()
    }

    pub fn set_selected(&mut self, idx: Option<usize>) {
        self.state.select(idx);
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

    pub fn render_table(&mut self, 
                        frame: &mut Frame, 
                        area: Rect, 
                        ranks: &[Level], 
                        costs: &[u32],
                        strengths: &Strengths,
                        weaknesses: &Weaknesses,
    ) {
        self.update_levels(ranks);
        self.update_costs(costs);

        let header_style = Style::default();
        let selected_row_style = Style::default()
            .bg(Color::DarkGray);
        let header = ["(Sec", "PRI)", "NAME", "RANK", "XP for level up"]
            .into_iter()
            .map(Cell::from)
            .collect::<Row>()
            .style(header_style);

        let rows = self.items.iter().map(|data| {
            let item = data.formatted_array(strengths, weaknesses);
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
                Constraint::Length(12),
                Constraint::Length(16),
            ],
            )
            .header(header)
            .row_highlight_style(selected_row_style)
            .highlight_spacing(HighlightSpacing::Always);

        frame.render_stateful_widget(t, area, &mut self.state);
    }
}

pub fn generate_survival_skills() -> Vec<Data> {
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

pub fn generate_general_skills() -> Vec<Data> {
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

pub fn generate_professional_skills() -> Vec<Data> {
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

pub fn generate_conflict_skills() -> Vec<Data> {
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
