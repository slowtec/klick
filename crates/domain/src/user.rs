use crate::EmailAddress;

#[derive(Debug, Clone)]
pub struct User {
    pub email: EmailAddress,
    pub email_confirmed: bool,
}
