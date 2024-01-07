use crate::EmailAddress;

#[derive(Debug, Clone)]
pub struct Account {
    pub email: EmailAddress,
    pub email_confirmed: bool,
}
