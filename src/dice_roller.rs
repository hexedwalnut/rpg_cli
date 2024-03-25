use lrlex::lrlex_mod;
use lrpar::lrpar_mod;   

use crate::Activity;
use crate::App;

use tui::{
    backend::{Backend}, 
    widgets::{Block, Borders, List, ListItem, Paragraph, BorderType},
    layout::{Layout, Constraint, Direction, Rect, Corner},
    style::{Color, Style},
    text::{Span, Spans},
    Frame
};

use unicode_width::UnicodeWidthStr;

lrlex_mod!("dice.l");
lrpar_mod!("dice.y");

pub struct DiceRoller {
    dice_roll_results: Vec<String>,
}

impl Default for DiceRoller {
    fn default() -> DiceRoller {
        DiceRoller {
            dice_roll_results: vec![],
        }
    }
}

impl Activity for DiceRoller {
    fn render_op_widget<'a, B: Backend>(f: &mut Frame<B>, _app: &mut App, layout: &Vec<Rect>) {
        let active = Paragraph::new("Dice Roller").block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Black))
            .border_type(BorderType::Plain),
        );
        f.render_widget(active, layout[1]);
    }
    
    fn render_tab_widget<'a, B: Backend>(f: &mut Frame<B>, _app: &mut App, layout: &Vec<Rect>) {
        let active = Paragraph::new("Dice Roller").block(
            Block::default()
            .borders(Borders::ALL)
            .style(Style::default().fg(Color::Black))
            .border_type(BorderType::Plain),
        );
        f.render_widget(active, layout[1]);
    }

    fn render_active_widget<'a, B: Backend>(f: &mut Frame<B>, _app: &mut App, layout: &Vec<Rect>) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .margin(0)
            .constraints(
                [
                    Constraint::Min(2),     // Result Log
                    Constraint::Length(3),  // Input
                ]
                .as_ref(),
            )
            .split(layout[1]);
        
        let input_block = Paragraph::new(_app.input.as_ref())
            .style(Style::default())
            .block(Block::default().borders(Borders::ALL)
                .title("Input"));
        f.render_widget(input_block, chunks[1]);
        f.set_cursor(
            chunks[1].x + _app.input.width() as u16 + 1,
            chunks[1].y + 1,
        );

        let results: Vec<ListItem> = _app
            .dice_roller.get_results()
            .iter().rev()
            .enumerate()
            .map(|(i, m)| {
                let content = vec![Spans::from(
                    Span::raw(format!("{}: {}", i, m))
                )];
                ListItem::new(content)
            })
            .collect();
        let results = 
            List::new(results).block(Block::default()
                .borders(Borders::ALL).title("Result"))
                .start_corner(Corner::BottomLeft);
        f.render_widget(results, chunks[0]);
    }
}

impl DiceRoller {
    pub fn eval_dice_roll(&mut self, command: String) {
        let mut output: String = String::new();
        let parts: Vec<_> = command.split(',').collect();

        let lexerdef = dice_l::lexerdef();
        for i in 0..parts.len() {
            let part = parts[i];
            let lexer = lexerdef.lexer(part);
            let (res, errs) = dice_y::parse(&lexer);
            for e in errs {
                output.push_str(format!("{}", 
                        e.pp(&lexer, &dice_y::token_epp)).as_str());
            }
            let result = match res {
                Some(Ok(r)) => format!("{} = {}", r.0, r.1),
                _ => format!("Unable to evaluate expression.")
            };
            output.push_str(result.as_str());

            if i < parts.len() - 1 {
                output.push_str(", ");
            }
        }
        self.dice_roll_results.push(output);
    }   

    pub fn clear_results(&mut self) {
        self.dice_roll_results.clear();
    }

    pub fn get_results(&mut self) -> Vec<String> {
        self.dice_roll_results.clone()
    }
}
