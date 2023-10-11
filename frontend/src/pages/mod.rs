mod faq;
mod home;
mod tool;

pub use self::{faq::*, home::*, tool::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Tool,
    Faq,
    Imprint,
}

impl Page {
    pub const fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Tool => "/tool",
            Self::Faq => "/faq",
            Self::Imprint => "/imprint",
        }
    }
}
