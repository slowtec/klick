use klick_domain::EmailAddress;

pub trait Gateway {
    fn notify(&self, event: Event);
}

pub enum Event {
    AccountWasCreated(EmailAddress),
}
