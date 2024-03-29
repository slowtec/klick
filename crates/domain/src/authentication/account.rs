use time::OffsetDateTime;

use crate::authentication::EmailAddress;

#[derive(Debug, Clone)]
pub struct Account {
    pub email_address: EmailAddress,
    pub email_confirmed: bool,
    pub created_at: OffsetDateTime,
}
