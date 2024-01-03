mod faq;
mod login;
mod register;
mod reset_password;
mod tool;

pub use self::{faq::*, login::*, register::*, reset_password::*, tool::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Tool,
    Faq,
    Imprint,
    OpenSource,
    Login,
    Register,
    ResetPassword,
}

impl Page {
    pub const fn path(self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Tool => "/tool",
            Self::Faq => "/faq",
            Self::Imprint => "/imprint",
            Self::OpenSource => "/open-source",
            Self::Login => "/login",
            Self::Register => "/register",
            Self::ResetPassword => "/reset-password",
        }
    }
}
