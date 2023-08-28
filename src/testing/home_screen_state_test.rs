use crate::{
    email::{
        email_account::EmailAccount,
        email_message::{ContentType, EmailMessage, Headers},
        user_credentials::UserCredentials,
    },
    state::app_state::UiState,
};

pub fn provide_test_email_account_list() -> Vec<EmailAccount> {
    vec![
        EmailAccount {
            account_type: crate::email::email_account::AccountType::Outlook,
            credentials: UserCredentials {
                email: "someone@somewhere.com".to_string(),
                password: "123bad".to_string(),
                authentication_identity: "idk".to_string(),
                secret: "shhhh".to_string(),
            },
        },
        EmailAccount {
            account_type: crate::email::email_account::AccountType::Gmail,
            credentials: UserCredentials {
                email: "someone@somewhere.com".to_string(),
                password: "123bad".to_string(),
                authentication_identity: "idk".to_string(),
                secret: "shhhh".to_string(),
            },
        },
    ]
}

pub fn provide_test_email_list() -> Vec<EmailMessage> {
    vec![
        EmailMessage {
            headers: Headers {
                addresses: crate::email::email_message::Addresses {
                    from: "someone@somewhere.com".to_string(),
                    to: "someoneElse@somewhere.com".to_string(),
                },
                subject: "heya".to_string(),
                content_type: ContentType::TextPlain,
            },
            body: crate::email::email_message::MessageBody {
                data: "Hello :)".to_string(),
            },
        },
        EmailMessage {
            headers: Headers {
                addresses: crate::email::email_message::Addresses {
                    from: "someoneElse@somewhere.com".to_string(),
                    to: "someone@somewhere.com".to_string(),
                },
                subject: "hi heaaaaayyyaya".to_string(),
                content_type: ContentType::TextPlain,
            },
            body: crate::email::email_message::MessageBody {
                data: "Hello num 2 :)".to_string(),
            },
        },
    ]
}

pub fn provide_test_state() -> UiState {
    let email_list = provide_test_email_list();
    let email_accounts = provide_test_email_account_list();
    let focused_email = email_list[0].clone();
    UiState::Home {
        email_in_focus: None,
        emails_from_selected_account: email_list,
        selected_account: EmailAccount {
            account_type: crate::email::email_account::AccountType::Outlook,
            credentials: crate::email::user_credentials::UserCredentials {
                email: "someone@somewhere.com".to_string(),
                password: "abadpass123".to_string(),
                authentication_identity: "idk".to_string(),
                secret: "ssshhh".to_string(),
            },
        },
        focus_state: crate::state::app_state::HomeFocusedUiState::EmailSelectedFocus,
        available_accounts: email_accounts,
    }
}
