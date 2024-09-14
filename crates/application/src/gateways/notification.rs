use klick_domain::EmailNonce;

pub trait Gateway {
    fn notify(&self, event: Event);
}

#[derive(Debug)]
pub enum Event {
    AccountWasCreated { email_nonce: EmailNonce },
    AccountResetPasswordRequested { email_nonce: EmailNonce },
}
