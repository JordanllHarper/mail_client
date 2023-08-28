use std::rc::Rc;

use ratatui::style::{Color, Stylize};
use ratatui::text::{Line, Span, Text};
use ratatui::{layout, Frame};

use ratatui::layout::Direction;
use ratatui::prelude::{Backend, Constraint, Margin, Rect};
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

use crate::email::email_account::EmailAccount;
use crate::email::email_message::EmailMessage;
use crate::state::app_state::{HomeFocusedUiState, UiState};

pub fn home_screen<'a, B: ratatui::backend::Backend>(
    frame: &'a mut Frame<B>,
    email_in_focus: &Option<EmailMessage>,
    emails_from_selected_account: &Vec<EmailMessage>,
    selected_account: &EmailAccount,
    accounts: &Vec<EmailAccount>,
    focus_state: &HomeFocusedUiState,
) {
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
    let account_pane_inner = layout[0].inner(&Margin {
        horizontal: 1,
        vertical: 1,
    });
    let account_pane = account_list_pane_block(accounts, selected_account, account_pane_inner);

    frame.render_widget(account_pane, layout[0]);
    let email_list_pane_inner = layout[1].inner(&Margin {
        horizontal: 1,
        vertical: 1,
    });
    let email_list_pane = email_list_pane(emails_from_selected_account);
    let email_list = email_list(emails_from_selected_account);

    frame.render_widget(email_list_pane, layout[1]);
    frame.render_widget(email_list, email_list_pane_inner);
    let email_pane_preview_inner = layout[2].inner(&Margin {
        horizontal: 1,
        vertical: 1,
    });
    let email_pane_preview = email_preview_pane(email_in_focus, email_list_pane_inner);
    let text = email_preview_text(email_in_focus);
    frame.render_widget(email_pane_preview, layout[2]);
    frame.render_widget(text, email_pane_preview_inner)
}

//account list component
fn account_list_pane_block<'a>(
    accounts: &Vec<EmailAccount>,
    selected_account: &EmailAccount,
    account_pane_inner: Rect,
) -> List<'a> {
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
fn email_list_pane<'a>(emails_from_selected_account: &Vec<EmailMessage>) -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Emails")
}

fn email_list<'a>(emails: &Vec<EmailMessage>) -> List<'a> {
    let list_items: Vec<ListItem> = emails
        .into_iter()
        .map(|e| ListItem::new(e.clone().headers.subject))
        .collect();
    List::new(list_items)
}
//email preview component
fn email_preview_pane<'a>(email_in_focus: &Option<EmailMessage>, parent_inner: Rect) -> Block<'a> {
    Block::default().borders(Borders::ALL).title("Preview")
}

fn email_preview_text(email_in_focus: &Option<EmailMessage>) -> Paragraph {
    match email_in_focus {
        Some(v) => Paragraph::new(v.body.data.clone()),
        None => Paragraph::new("No email selected"),
    }
}
