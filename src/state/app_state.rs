use crate::email::{
    email_message::{Addresses, EmailMessage},
    user_credentials::UserCredentials,
};

enum UiState {
    //Setup - user journey for setting up an initial account,
    //this isn't used unless
    Setup {
        //informs if we want to make a new file for storing account creds or add to an existing
        //file
        has_existing_account: bool,
    },
    //Home screen - overview of emails
    Home {
        email_in_focus: EmailMessage,
        emails: Vec<EmailMessage>,
        focus_state: HomeUiState,
    },
    //Send email screen - input text to write and send an email
    //None option signifies brand new email with no specified addresses
    //Some option signifies
    SendEmail {
        state: SendEmailUiState,
        address: Option<Addresses>,
    },
}

enum HomeUiState {
    NavigateEmailsFocus(Vec<EmailMessage>),
    EmailSelectedFocus(EmailMessage),
    AccountPaneFocus,
}

enum SendEmailUiState {
    SendNewEmail(UserCredentials),
    SendReplyEmail {
        creds: UserCredentials,
        previous_email: EmailMessage,
    },
}
