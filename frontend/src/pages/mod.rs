mod faq;
mod tool;

pub use self::{faq::*, tool::*};

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
