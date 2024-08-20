use leptos::*;

pub const FOOTER_MD: &str = include_str!("../content/footer.md");

use klick_app_components::{
    icons,
    links::{ACCESSIBILITY, DATENSCHUTZ, FAQ_DE, LINKEDIN, WIKI_URL},
};

use crate::{Markdown, Page, CHANGELOG_URL, VERSION};

#[component]
pub fn Footer() -> impl IntoView {
    view! {
      <footer class="bg-gray-100">
        <div class="mx-auto max-w-7xl overflow-hidden px-6 pt-16 sm:pt-20 lg:px-8">
          <div class="mb-14">
            <div class="text-gray-600 text-center grid place-content-center">
              <div
                class="grid place-content-center mb-8"
                role="img"
                aria-label="Logo des Bundesministeriums für Wirtschaft und Klimaschutz (BMWK) sowie der Nationalen Klimaschutzinitiative (NKI). Über dem BMWK-Logo steht die Beschreibung: gefördert durch und unter dem BMWK-Logo steht dazugehörig: aufgrund eines Beschlusses des Deutschen Bundestages">
                <img src="logo_BMWK_NKI.svg" />
              </div>
              <Markdown content = FOOTER_MD />
            </div>
          </div>
          <div>
            <div class="mt-10 flex justify-center space-x-10">
              <a href="https://codeberg.org/slowtec/klick" class="text-gray-400 hover:text-gray-500">
                <span class="sr-only">Codeberg</span>
                <svg aria-hidden="true" viewBox="-2 -2 28 28" class="h-6 w-6 fill-slate-500 group-hover:fill-slate-700">
                  <path d="M11.955.49A12 12 0 0 0 0 12.49a12 12 0 0 0 1.832 6.373L11.838 5.928a.187.14 0 0 1 .324 0l10.006 12.935A12 12 0 0 0 24 12.49a12 12 0 0 0-12-12 12 12 0 0 0-.045 0zm.375 6.467 4.416 16.553a12 12 0 0 0 5.137-4.213z"></path>
                </svg>
              </a>
            </div>
            <p class="mb-8 mt-4 text-center text-xs leading-5 text-gray-400">
              <a class="no-underline hover:underline" href= { CHANGELOG_URL} >"v" { VERSION }</a>
            </p>
            <p class="mb-8 text-center text-xs leading-5 text-gray-500">
              "Made with ♥ by "
              <a href="https://slowtec.de">"slowtec GmbH"</a>
            </p>
          </div>

          </div>
        <div class="bg-black">
         <nav
            class="py-4 grid grid-cols-1 xl:grid-cols-10"
            aria-label="Footer"
          >
            <div class="grid gap-3 items-center justify-center justify-items-center md:grid-cols-8 xl:col-span-6">
              <FooterLink link = LinkType::Page(Page::Home) label ="KlicK" />
              <FooterLink link = LinkType::Page(Page::Tool) label ="Tool" />
              <FooterLink link = LinkType::External(WIKI_URL) label ="Wiki" />
              <FooterLink link = LinkType::External(FAQ_DE) label ="FAQs" />
              <FooterLink link = LinkType::External(DATENSCHUTZ) label ="Datenschutz" />
              <FooterLink link = LinkType::External(ACCESSIBILITY) label ="Barrierefreiheit" />
              <FooterLink link = LinkType::Page(Page::OpenSource) label ="Open Source" />
              <FooterLink link = LinkType::Page(Page::Imprint) label ="Impressum" />
            </div>
            <div class="my-3 grid gap-3 justify-center justify-items-center xl:grid-cols-2 xl:col-span-4">
              <div class="my-3">
                <a href="https://www.thelaend.de">
                  <icons::TheLaend />
                </a>
              </div>
              <div class="flex items-center">
                <span class="text-white mr-3">"Folgen Sie uns auf LinkedIn"</span>
                <a class="inline-block" href={ LINKEDIN }>
                  <icons::LinkedIn />
                </a>
              </div>
            </div>
          </nav>
        </div>
      </footer>
    }
}

#[component]
fn FooterLink(link: LinkType, label: &'static str) -> impl IntoView {
    view! {
      <div class="">
        <a
          href = {
            match link {
              LinkType::Page(page) => page.path(),
              LinkType::External(url) => url
            }
          }
          class = "text-sm leading-6 text-white hover:text-gray-200">
          { label }
        </a>
      </div>
    }
}

#[derive(Clone, Copy)]
enum LinkType {
    Page(Page),
    External(&'static str),
}
