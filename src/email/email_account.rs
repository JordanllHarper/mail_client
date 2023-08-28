use super::user_credentials::UserCredentials;

pub struct EmailAccount {
    account_type: AccountType,
    //credentials for this account
    credentials: UserCredentials,
}

//TODO: Add more supported once up and running
//TODO Could add color schememing based on type
pub enum AccountType {
    Outlook,
    Gmail,
}
