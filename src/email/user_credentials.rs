use lettre::transport::smtp::authentication::Credentials;

pub struct UserCredentials {
    pub authentication_identity: String,
    pub secret: String,
}
impl UserCredentials {
    pub fn to_lettre_creds(self) -> Credentials {
        Credentials::new(self.authentication_identity, self.secret)
    }
}
