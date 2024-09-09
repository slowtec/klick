mod confirm_email_address;
mod login;
mod projects;
mod register;
mod reset_password;
mod reset_password_request;
mod tool;

pub use self::{
    confirm_email_address::*, login::*, projects::*, register::*, reset_password::*,
    reset_password_request::*, tool::*,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Tool,
    Faq,
    Imprint,
    OpenSource,
    Login,
    Register,
    ResetPasswordRequest,
    ResetPassword,
    ConfirmEmailAddress,
    Projects,
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
            Self::ResetPasswordRequest => "/reset-password-request",
            Self::ResetPassword => "/reset-password",
            Self::ConfirmEmailAddress => "/confirm-email-address",
            Self::Projects => "/projects",
        }
    }
}
