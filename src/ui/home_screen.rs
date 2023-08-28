use std::rc::Rc;

use ratatui::Frame;

use ratatui::layout::Direction;
use ratatui::prelude::{Backend, Constraint, Rect};
use ratatui::widgets::{Block, Borders};

pub fn home_screen<B: ratatui::backend::Backend>(frame: &mut Frame<B>) {
    let layout = ratatui::layout::Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [
                Constraint::Percentage(15),
                Constraint::Percentage(35),
                Constraint::Percentage(50),
            ]
            .as_ref(),
        )
        .split(frame.size());

    account_pane(&layout, frame);
    email_list_pane(&layout, frame);
    email_preview_pane(&layout, frame);
}

fn account_pane<B: Backend>(layout: &Rc<[Rect]>, frame: &mut Frame<B>) {
    let block = Block::default().borders(Borders::ALL).title("Accounts");

    frame.render_widget(block, layout[0]);
}

fn email_list_pane<B: Backend>(layout: &Rc<[Rect]>, frame: &mut Frame<B>) {
    let block = Block::default().borders(Borders::ALL).title("Emails");

    frame.render_widget(block, layout[1]);
}
fn email_preview_pane<B: Backend>(layout: &Rc<[Rect]>, frame: &mut Frame<B>) {
    let block = Block::default().borders(Borders::ALL).title("Preview");

    frame.render_widget(block, layout[2]);
}
