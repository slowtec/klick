#![allow(clippy::wildcard_imports)]

use leptos::*;
use leptos_meta::provide_meta_context;
use leptos_router::{Route, Router, Routes};

mod footer;
mod forms;
mod nav;
mod pages;
mod sankey;

use self::{
    footer::Footer,
    nav::Nav,
    pages::{Faq, Page, Tool},
};

const VERSION: &str = env!("CARGO_PKG_VERSION");
const IMPRINT_MD: &str = include_str!("../content/imprint.md");
const ABOUT_MD: &str = include_str!("../content/about.md");
const OPEN_SOURCE_MD: &str = include_str!("../content/open-source.md");
const CHANGELOG_URL: &str = concat!(
    "https://codeberg.org/slowtec/klick/src/tag/v",
    env!("CARGO_PKG_VERSION"),
    "/CHANGELOG.md"
);

#[component]
pub fn App() -> impl IntoView {
    provide_meta_context();

    let (current_page, set_current_page) = create_signal(Page::Home);

    view! {
      <Nav current_page = current_page.into() />
      <Router>
        <Routes>
          <Route
            path=Page::Home.path()
            view= move ||{
              set_current_page.update(|p|*p = Page::Home);
              view! {
                <Main>
                  <Markdown content = ABOUT_MD />
                </Main>
              }
            }
          />
          <Route
            path=Page::Tool.path()
            view= move ||{
              set_current_page.update(|p|*p = Page::Tool);
              view! {
                <Main>
                  <header class="prose">
                    <h1 class="mb-8">
                      "KlicK-Tool "
                      <span class="font-light text-xl text-gray-600">
                        "(Betaversion "
                        <a class="font-light text-xl no-underline hover:underline" href= { CHANGELOG_URL} >"v" { VERSION } ")"</a>
                      </span>
                    </h1>
                  </header>
                  <Tool />
                </Main>
              }
            }
          />
          <Route
            path=Page::Faq.path()
            view= move || {
              set_current_page.update(|p|*p = Page::Faq);
              view! {
                <Main>
                  <header class="prose">
                    <h1>"FAQs"</h1>
                  </header>
                  <Faq />
                </Main>
              }
            }
          />
          <Route
            path=Page::OpenSource.path()
            view= move || {
              set_current_page.update(|p|*p = Page::OpenSource);
              view! {
                <Main>
                  <Markdown content = OPEN_SOURCE_MD />
                </Main>
              }
            }
          />
          <Route
            path=Page::Imprint.path()
            view= move || {
              set_current_page.update(|p|*p = Page::Imprint);
              view! {
                <Main>
                  <Markdown content = IMPRINT_MD />
                </Main>
              }
            }
          />
        </Routes>
      </Router>
      <Footer />
    }
}

#[component]
fn Main(children: Children) -> impl IntoView {
    view! {
      <div class="py-10">
        <main>
          <div class="mx-auto max-w-7xl sm:px-6 lg:px-8">
            <div class="px-4 py-8 sm:px-0">
              { children() }
            </div>
          </div>
        </main>
      </div>
    }
}

#[component]
fn Markdown(content: &'static str) -> impl IntoView {
    use pulldown_cmark::{html, Options, Parser};

    let mut options = Options::empty();
    options.insert(Options::ENABLE_STRIKETHROUGH);
    let parser = Parser::new_ext(content, options);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    view! {
      <div class="prose" inner_html = html_output></div>
    }
}
