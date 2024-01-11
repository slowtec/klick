use crate::EmailAddress;

#[derive(Debug, Clone)]
pub struct Account {
    // TODO: rename to `email_address`
    pub email: EmailAddress,
    pub email_confirmed: bool,
}
