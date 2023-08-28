use std::rc::Rc;

use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::{layout, Frame};

use ratatui::layout::Direction;
use ratatui::prelude::{Backend, Constraint, Rect};
use ratatui::widgets::{Block, Borders, List, ListItem};

use crate::email::email_account::EmailAccount;

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

    let test_accounts = vec![EmailAccount {
        account_type: crate::email::email_account::AccountType::Gmail,
        credentials: crate::email::user_credentials::UserCredentials {
            email: "an_email_that_is_toooooooooo_long@somewhere.com".to_owned(),
            password: "123bad".to_owned(),
            authentication_identity: "identity".to_owned(),
            secret: "secret".to_owned(),
        },
    }];
    let account_pane = account_list_pane(test_accounts);
    frame.render_widget(account_pane, layout[0]);
    let email_list_pane = email_list_pane();
    frame.render_widget(email_list_pane, layout[1]);
    let email_pane_preview = email_preview_pane();
    frame.render_widget(email_pane_preview, layout[2]);
}

//account list component
fn account_list_pane<'a>(accounts: Vec<EmailAccount>) -> List<'a> {
    let list_items: Vec<ListItem> = accounts
        .iter()
        .map(|a| {
            let color: Color = match a.account_type {
                crate::email::email_account::AccountType::Outlook => Color::Blue,
                crate::email::email_account::AccountType::Gmail => Color::Red,
            };
            let initial = &a.credentials.email;

            let content = Line::from(Span::raw(format!("{initial}")));
            ListItem::new(content).bg(color)
        })
        .into_iter()
        .collect();
    let list =
        List::new(list_items).block(Block::default().borders(Borders::ALL).title("Accounts"));
    list
}

//email list component
fn email_list_pane<'a>() -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Emails")
}
//email preview component
fn email_preview_pane<'a>() -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Preview")
}
