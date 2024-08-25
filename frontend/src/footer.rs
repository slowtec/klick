use leptos::*;
use leptos_fluent::*;

use klick_app_components::{
    icons,
    links::{ACCESSIBILITY, DATENSCHUTZ, FAQ_DE, LINKEDIN, WIKI_URL},
};
use klick_presenter::Lng;

use crate::{ContentLoader, Page, CHANGELOG_URL, VERSION};

#[component]
pub fn Footer(lng: Signal<Lng>) -> impl IntoView {
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
              <ContentLoader
                file = "footer.html"
                lng = lng
              />
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
              <FooterLink link = LinkType::Page(Page::Home) label = move_tr!("klick") />
              <FooterLink link = LinkType::Page(Page::Tool) label = move_tr!("tool") />
              <FooterLink link = LinkType::External(WIKI_URL) label = move_tr!("wiki") />
              <FooterLink link = LinkType::External(FAQ_DE) label = move_tr!("faqs") />
              <FooterLink link = LinkType::External(DATENSCHUTZ) label = move_tr!("data-privacy") />
              <FooterLink link = LinkType::External(ACCESSIBILITY) label = move_tr!("accessibility") />
              <FooterLink link = LinkType::Page(Page::OpenSource) label = move_tr!("open-source") />
              <FooterLink link = LinkType::Page(Page::Imprint) label = move_tr!("imprint") />
            </div>
            <div class="my-3 grid gap-3 justify-center justify-items-center xl:grid-cols-2 xl:col-span-4">
              <div class="my-3">
                <a href="https://www.thelaend.de">
                  <icons::TheLaend />
                </a>
              </div>
              <div class="flex items-center">
                <span class="text-white mr-3">{ move_tr!("follow-us-on-linkedin") }</span>
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
fn FooterLink(link: LinkType, label: Signal<String>) -> impl IntoView {
    let href = match link {
        LinkType::Page(page) => page.path(),
        LinkType::External(url) => url,
    };

    view! {
      <div>
        <a
          href = href
          class = "text-sm leading-6 text-white hover:text-gray-200"
        >
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
