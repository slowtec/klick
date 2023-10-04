mod documentation;
mod home;

pub use self::{documentation::*, home::*};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Page {
    Home,
    Faq,
    Imprint,
}

impl Page {
    pub const fn path(&self) -> &'static str {
        match self {
            Self::Home => "/",
            Self::Faq => "/faq",
            Self::Imprint => "/imprint",
        }
    }
}
