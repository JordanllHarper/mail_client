use crate::email::{
    email_account::EmailAccount,
    email_message::{Addresses, EmailMessage},
};

pub enum UiState {
    //Setup - user journey for setting up an initial account,
    //this is only used on first load if there is no other account stored
    Setup {
        //informs if we want to make a new file for storing account creds or add to an existing
        //file
        has_existing_account: bool,
    },
    //Home screen - overview of emails
    //All information - accounts and emails within the account selected
    Home {
        //email shown on the email overview screen
        email_in_focus: Option<EmailMessage>,
        //emails shown in the list view
        emails_from_selected_account: Vec<EmailMessage>,
        //account selected shown in account pane
        selected_account: EmailAccount,
        //accounts that are overall
        available_accounts: Vec<EmailAccount>,
        //state monitoring which pane the user has selected
        focus_state: HomeFocusedUiState,
    },

    //intermediate state for going between main states
    Loading,
    //Send email screen - input text to write and send an email
    //None option signifies brand new email with no specified addresses
    //Some option signifies a response
    SendEmail {
        state: SendEmailUiState,
        address: Option<Addresses>,
    },
}

pub enum SetupUserJourneyUiState {
    //select account type from supported types
    SelectAccountType,
    //enter email
    EnterEmail,
    //enter password
    EnterPassword,
    //validate the credentials
    Validation(SetupIntermediateJourneyState),
}

pub enum SetupIntermediateJourneyState {
    //The authentication was successful
    Success,
    //The authentication failed
    Failure,
    //Current loading state
    Loading,
}

//Dictates what is interactable and ui colouring
pub enum HomeFocusedUiState {
    NavigateEmailsFocus(Vec<EmailMessage>),
    EmailSelectedFocus,
    AccountPaneFocus { selected_account: EmailAccount },
}

//The Ui state informing to move to the send email flow
pub enum SendEmailUiState {
    SendNewEmail(EmailAccount),
    SendReplyEmail {
        creds: EmailAccount,
        previous_email: EmailMessage,
    },
    //TODO: Support email forwarding
}
