use std::rc::Rc;

use ratatui::{layout, Frame};

use ratatui::layout::Direction;
use ratatui::prelude::{Backend, Constraint, Rect};
use ratatui::widgets::{Block, Borders};

pub fn home_screen<'a, B: ratatui::backend::Backend>(frame: &'a mut Frame<B>) {
    let layout: Rc<[Rect]> = ratatui::layout::Layout::default()
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

    let account_pane = account_list_pane();
    frame.render_widget(account_pane, layout[0]);
    let email_list_pane = email_list_pane();
    frame.render_widget(email_list_pane, layout[1]);
    let email_pane_preview = email_preview_pane();
    frame.render_widget(email_pane_preview, layout[2]);
}

//account list component
fn account_list_pane<'a>() -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Accounts")
}

//email list component
fn email_list_pane<'a>() -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Emails")
}
//email preview component
fn email_preview_pane<'a>() -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Preview")
}
