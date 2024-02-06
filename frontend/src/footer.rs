use leptos::*;

const LOGO: &str = include_str!("../assets/logo_BMWK_NKI.svg");
const FOOTER_MD: &str = include_str!("../content/footer.md");
const WIKI_URL: &str = "https://codeberg.org/slowtec/klick/wiki";

use crate::{Markdown, Page, CHANGELOG_URL, VERSION};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
      <footer class="bg-gray-100">
        <div class="mx-auto max-w-7xl overflow-hidden px-6 py-16 sm:py-20 lg:px-8">
          <div class="mb-14">
            <div class="text-gray-600 text-center grid place-content-center">
              <div class="grid place-content-center mb-8" inner_html = LOGO></div>
              <Markdown content = FOOTER_MD />
            </div>
          </div>
          <div>
            <nav class="-mb-6 columns-2 sm:flex sm:justify-center sm:space-x-12" aria-label="Footer">
              <FooterLink link = LinkType::Page(Page::Home) label ="Über KlicK" />
              <FooterLink link = LinkType::Page(Page::Tool) label ="Tool" />
              <FooterLink link = LinkType::Page(Page::Faq) label ="FAQs" />
              <FooterLink link = LinkType::External(WIKI_URL) label ="Wiki" />
              <FooterLink link = LinkType::Page(Page::OpenSource) label ="Open Source" />
              <FooterLink link = LinkType::Page(Page::Imprint) label ="Impressum" />
            </nav>
            <div class="mt-10 flex justify-center space-x-10">
              <a href="https://codeberg.org/slowtec/klick" class="text-gray-400 hover:text-gray-500">
                <span class="sr-only">Codeberg</span>
                <svg aria-hidden="true" viewBox="-2 -2 28 28" class="h-6 w-6 fill-slate-500 group-hover:fill-slate-700">
                  <path d="M11.955.49A12 12 0 0 0 0 12.49a12 12 0 0 0 1.832 6.373L11.838 5.928a.187.14 0 0 1 .324 0l10.006 12.935A12 12 0 0 0 24 12.49a12 12 0 0 0-12-12 12 12 0 0 0-.045 0zm.375 6.467 4.416 16.553a12 12 0 0 0 5.137-4.213z"></path>
                </svg>
              </a>
            </div>
            <p class="mt-10 text-center text-xs leading-5 text-gray-500">
              "Made with ♥ by "
              <a href="https://slowtec.de">"slowtec GmbH"</a>
            </p>
            <p class="mt-10 text-center text-xs leading-5 text-gray-400">
              <a class="no-underline hover:underline" href= { CHANGELOG_URL} >"v" { VERSION }</a>
            </p>
          </div>
        </div>
      </footer>
    }
}

#[component]
fn FooterLink(link: LinkType, label: &'static str) -> impl IntoView {
    view! {
      <div class="pb-6">
        <a
          href = {
            match link {
              LinkType::Page(page) => page.path(),
              LinkType::External(url) => url
            }
          }
          class = "text-sm leading-6 text-gray-600 hover:text-gray-900">
          { label }
        </a>
      </div>
    }
}

enum LinkType {
  Page(Page),
  External(&'static str)
}
