extern crate native_tls;
pub mod email;
pub mod state;
pub mod ui;
use std::{
    io::{self, Stdout},
    time::Duration,
};

use color_eyre::Result;
use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use eyre::Context;
use ratatui::{prelude::CrosstermBackend, Terminal};
use state::app_state::{self, HomeFocusedUiState, UiState};
use testing::home_screen_state_test::{self, provide_test_state};
use ui::home_screen::home_screen;
pub mod testing;

enum TestingFlag {
    None,
    HomeTest,
    LoadingTest,
}

fn main() -> Result<()> {
    color_eyre::install()?;

    let mut terminal = setup_terminal()?;

    //TODO: Remove flag when in prod
    run(&mut terminal, TestingFlag::HomeTest)?;

    restore_terminal(&mut terminal)?;

    Ok(())
}

/// Setup the terminal. This is where you would enable raw mode, enter the alternate screen, and
/// hide the cursor. This example does not handle errors. A more robust application would probably
/// want to handle errors and ensure that the terminal is restored to a sane state before exiting.
fn setup_terminal() -> Result<Terminal<CrosstermBackend<Stdout>>> {
    let mut stdout = io::stdout();
    enable_raw_mode().context("failed to enable raw mode")?;
    execute!(stdout, EnterAlternateScreen).context("unable to enter alternate screen")?;
    Terminal::new(CrosstermBackend::new(stdout)).context("creating terminal failed")
}

/// Restore the terminal. This is where you disable raw mode, leave the alternate screen, and show
/// the cursor.
fn restore_terminal(terminal: &mut Terminal<CrosstermBackend<Stdout>>) -> Result<()> {
    disable_raw_mode().context("failed to disable raw mode")?;
    execute!(terminal.backend_mut(), LeaveAlternateScreen)
        .context("unable to switch to main screen")?;
    terminal.show_cursor().context("unable to show cursor")
}

/// Run the application loop. This is where you would handle events and update the application
/// state. This example exits when the user presses 'q'. Other styles of application loops are
/// possible, for example, you could have multiple application states and switch between them based
/// on events, or you could have a single application state and update it based on events.
fn run(terminal: &mut Terminal<CrosstermBackend<Stdout>>, testing: TestingFlag) -> Result<()> {
    let mut state = UiState::Loading;
    match testing {
        TestingFlag::None => state = UiState::Loading,
        TestingFlag::HomeTest => state = provide_test_state(),
        TestingFlag::LoadingTest => todo!(),
    }

    loop {
        terminal.draw(|f| crate::render_app(f, &state))?;
        if should_quit()? {
            break;
        }
    }
    Ok(())
}

/// Render the application. This is where you would draw the application UI. This example just
/// draws a greeting.
fn render_app(frame: &mut ratatui::Frame<CrosstermBackend<Stdout>>, app_state: &UiState) {
    match app_state {
        UiState::Setup {
            has_existing_account: _,
        } => todo!(),
        UiState::Home {
            email_in_focus,
            emails_from_selected_account,
            selected_account,
            focus_state,
            available_accounts,
        } => home_screen(
            frame,
            email_in_focus,
            emails_from_selected_account,
            selected_account,
            available_accounts,
            focus_state,
        ),
        UiState::Loading => todo!(),
        UiState::SendEmail { state, address } => todo!(),
    }
}

/// Check if the user has pressed 'q'. This is where you would handle events. This example just
/// checks if the user has pressed 'q' and returns true if they have. It does not handle any other
/// events. There is a 250ms timeout on the event poll so that the application can exit in a timely
/// manner, and to ensure that the terminal is rendered at least once every 250ms.
fn should_quit() -> Result<bool> {
    if event::poll(Duration::from_millis(250)).context("event poll failed")? {
        if let Event::Key(key) = event::read().context("event read failed")? {
            return Ok(KeyCode::Char('q') == key.code);
        }
    }
    Ok(false)
}
