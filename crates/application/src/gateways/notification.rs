use klick_domain::{EmailAddress, EmailNonce};

pub trait Gateway {
    fn notify(&self, event: Event);
}

pub enum Event {
    AccountWasCreated(EmailAddress, EmailNonce),
}
