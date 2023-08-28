use super::user_credentials::UserCredentials;

pub struct EmailAccount {
    pub account_type: AccountType,
    //credentials for this account
    pub credentials: UserCredentials,
}

//TODO: Add more supported once up and running
//TODO Could add color schememing based on type
pub enum AccountType {
    Outlook,
    Gmail,
}
